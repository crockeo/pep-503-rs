# pep-503-rs

A Rusty implementation of [PEP-503](https://peps.python.org/pep-0503/).

In its current form it's ~mostly copy/pasted from
[pyproxide's implementation](https://github.com/crockeo/pyproxide/blob/4355045e06fd8a6d963f9683173ac2f7d36b894a/src/pep_503.rs)
so that I can use it in other projects.
In the future I would like to:

- [ ] Improve the parsing API
  - Maybe: move from `impl FromStr` to a dedicated `parse` func?
  - Change the error type to be non-`()` so it's easier to use with `anyhow`.
- [ ] Fix struct naming so that it aligns with the terms used by PEP503.
- [ ] Change release parsing slightly:
  - Include a hash type when a hash is specified on the URI
    instead of including it on the URI.
  - Replace `has_gpg` with an `Option<String>` or `Option<Url>`
    that points to the `.asc` file.
- [ ] Add documentation for each of the types,
      their use,
      and their corresponding PEP503 concept.
- [ ] Replace kuchiki with an actively-maintained [html5ever](https://github.com/servo/html5ever) sink.
- [ ] Fuzz testing to find the many bugs I've included :)

## Installation

`pep-503-rs` is not yet uploaded to [crates.io](https://crates.io).
You need to take a direct Git dependency to install.
See the [Cargo reference](https://doc.rust-lang.org/cargo/reference/specifying-dependencies.html#specifying-dependencies-from-git-repositories)
for a complete guide.
TL;DR:

```toml
[dependencies.pep-503]
git = "https://github.com/crockeo/pep-503-rs"
rev = "<HEAD of main>"
```

And then in rust you can reference is at `pep_503`:

```rust
use pep_503::RootIndex;
use pep_503::PackageIndex;
```

You can also use `branch = "main"` if you want to track changes.

## License

MIT Open Source, see [LICENSE](./LICENSE).
