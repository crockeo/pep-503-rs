// reference: https://peps.python.org/pep-0503/
use std::str::FromStr;

use kuchiki::traits::TendrilSink;

#[derive(Eq, Debug, PartialEq)]
pub struct RootIndex {
    pub packages: Vec<String>,
}

impl ToString for RootIndex {
    fn to_string(&self) -> String {
        let links = self
            .packages
            .iter()
            .map(|package| -> String { format!("<a href=\"/simple/{package}/\">{package}</a>") })
            .collect::<Vec<String>>()
            .join("<br/>\n    ");

        format!(
            r#"<!DOCTYPE html>
<html>
    <body>
    {links}
    </body>
</html>"#
        )
    }
}

impl FromStr for RootIndex {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let document = kuchiki::parse_html().one(s);

        let mut packages = Vec::new();
        for node_ref in document.descendants() {
            let element_name = node_ref
                .as_element()
                .map(|element| element.name.local.to_string());
            if element_name != Some("a".to_string()) {
                continue;
            }

            let package = if let Some(child) = node_ref.first_child() {
                child.as_text().unwrap().borrow().clone()
            } else {
                continue;
            };
            packages.push(package);
        }
        Ok(Self { packages })
    }
}

#[derive(Eq, Debug, PartialEq)]
pub struct PackageIndex {
    pub releases: Vec<Release>,
}

impl ToString for PackageIndex {
    fn to_string(&self) -> String {
        let links = self
            .releases
            .iter()
            .map(Release::to_string)
            .collect::<Vec<String>>()
            .join("<br/>\n    ");

        format!(
            r#"<!DOCTYPE html>
<html>
    <body>
    {links}
    </body>
</html>"#
        )
    }
}

impl FromStr for PackageIndex {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let document = kuchiki::parse_html().one(s);

        let anchors = document.descendants().filter_map(|node_ref| {
            let element = node_ref.as_element()?.clone();
            if element.name.local.to_string() != "a" {
                return None;
            }
            Some((node_ref, element))
        });

        let mut releases = Vec::new();
        for (node_ref, anchor) in anchors {
            let name = if let Some(child) = node_ref.first_child() {
                child.text_contents()
            } else {
                continue;
            };

            let attributes = anchor.attributes.borrow();
            let uri = if let Some(href) = attributes.get("href") {
                href
            } else {
                continue;
            }
            .to_owned();

            // TODO: do some verification that each has_gpg==true entry
            // also has an associated GPG key
            let has_gpg = attributes.get("data-gpg-sig") == Some("true");
            let requires_python = attributes.get("data-requires-python").map(str::to_owned);

            releases.push(Release {
                name,
                uri,
                has_gpg,
                requires_python,
            })
        }

        Ok(Self { releases })
    }
}

#[derive(Eq, Debug, PartialEq)]
pub struct Release {
    pub name: String,
    pub uri: String,
    pub has_gpg: bool,
    pub requires_python: Option<String>,
}

impl ToString for Release {
    fn to_string(&self) -> String {
        let uri = &self.uri;
        let requires_python_part = if let Some(requires_python) = &self.requires_python {
            format!(" data-requires-python=\"{requires_python}\"")
        } else {
            "".to_string()
        };
        let gpg_sig_part = if self.has_gpg {
            " data-gpg-sig=\"true\""
        } else {
            ""
        };
        let name = &self.name;

        format!("<a href=\"{uri}\"{requires_python_part}{gpg_sig_part}>{name}</a>")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use std::fs;

    use pretty_assertions::assert_eq;

    #[test]
    fn test_root_index_lifecycle() {
        let root_index_html = fs::read_to_string("fixtures/index_fixture.html").unwrap();
        let root_index = RootIndex::from_str(&root_index_html);
        assert_eq!(
            root_index,
            Ok(RootIndex {
                packages: vec![
                    "numpy".to_string(),
                    "protobuf".to_string(),
                    "xgboost".to_string(),
                ],
            }),
        );
        let root_index = root_index.unwrap();
        assert_eq!(
            root_index.to_string(),
            r#"<!DOCTYPE html>
<html>
    <body>
    <a href="/simple/numpy/">numpy</a><br/>
    <a href="/simple/protobuf/">protobuf</a><br/>
    <a href="/simple/xgboost/">xgboost</a>
    </body>
</html>"#,
        );
    }

    #[test]
    fn test_package_index_lifecycle() {
        let package_index_html = fs::read_to_string("fixtures/xgboost_fixture.html").unwrap();
        let package_index = PackageIndex::from_str(&package_index_html);
        assert_eq!(
	    package_index,
	    Ok(PackageIndex{
		releases: vec![
		    Release{
			name: "xgboost-1.5.2-py3-none-macosx_10_15_x86_64.macosx_11_0_x86_64.macosx_12_0_x86_64.whl".to_string(),
			uri: concat!(
			    "https://fakepythonhosted.org/xgboost/",
			    "xgboost-1.5.2-py3-none-macosx_10_15_x86_64.macosx_11_0_x86_64.macosx_12_0_x86_64.whl",
			    "#sha256=51b5dfe553d78ab92f2c60ccead6abc38196c2961f6952f6ec14b1feba6ffd25",
			).to_string(),
			has_gpg: false,
			requires_python: Some(">=3.6".to_string()),
		    },
		    Release {
			name: "xgboost-1.5.2-py3-none-manylinux2014_aarch64.whl".to_string(),
			uri: concat!(
			    "https://fakepythonhosted.org/xgboost/",
			    "xgboost-1.5.2-py3-none-manylinux2014_aarch64.whl",
			    "#sha256=a449b0d0d8a15e72946c9d07e3d4ea153ac12570d054f6020cc5146cb4979261",
			).to_string(),
			has_gpg: false,
			requires_python: Some(
			    ">=3.6".to_string(),
			),
		    },
		    Release {
			name: "xgboost-1.5.2-py3-none-manylinux2014_x86_64.whl".to_string(),
			uri: concat!(
			    "https://fakepythonhosted.org/xgboost/",
			    "xgboost-1.5.2-py3-none-manylinux2014_x86_64.whl",
			    "#sha256=46e43e06b1de260fe4c67e818720485559dab1bed1d97b82275220fab602a2ba",
			).to_string(),
			has_gpg: false,
			requires_python: Some(
			    ">=3.6".to_string(),
			),
		    },
		    Release {
			name: "xgboost-1.5.2-py3-none-win_amd64.whl".to_string(),
			uri: concat!(
			    "https://fakepythonhosted.org/xgboost/",
			    "xgboost-1.5.2-py3-none-win_amd64.whl",
			    "#sha256=f9d459ad42da74c60136123ead36fa562086fb886f52fca229477d327d60dad0",
			).to_string(),
			has_gpg: false,
			requires_python: Some(
			    ">=3.6".to_string(),
			),
		    },
		    Release {
			name: "xgboost-1.5.2.tar.gz".to_string(),
			uri: concat!(
			    "https://fakepythonhosted.org/xgboost/",
			    "xgboost-1.5.2.tar.gz",
			    "#sha256=404dc09dca887ef5a9bc0268f882c54b33bfc16ac365a859a11e7b24d49da387",
			).to_string(),
			has_gpg: false,
			requires_python: Some(
			    ">=3.6".to_string(),
			),
		    },
		],
	    }),
	);
    }
}
