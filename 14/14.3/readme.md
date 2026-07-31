日本語版の手順に従うと次のワーニングが出ました。
```
miz@debian:~/src/rust/rust-tutorial/14/14.3/cargo_workspaces_14-7/add$ cargo new adder
    Creating binary (application) `adder` package
warning: virtual workspace defaulting to `resolver = "1"` despite one or more workspace members being on edition 2024 which implies `resolver = "3"`
note: to keep the current resolver, specify `workspace.resolver = "1"` in the workspace root's manifest
note: to use the edition 2024 resolver, specify `workspace.resolver = "3"` in the workspace root's manifest
note: for more details see https://doc.rust-lang.org/cargo/reference/resolver.html#resolver-versions
note: see more `Cargo.toml` keys and their definitions at https://doc.rust-lang.org/cargo/reference/manifest.html
````

[オリジナル版](https://doc.rust-lang.org/stable/book/ch14-03-cargo-workspaces.html) に従うと良いでしょう。
