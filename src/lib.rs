pub fn add(left: usize, right: usize) -> usize {
    left + right
}

fn normalize(name: impl AsRef<str>) -> String {
    // https://peps.python.org/pep-0503/#normalized-names
    name.as_ref()
        .to_lowercase()
        .replace("_", "-")
        .replace(".", "-")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
