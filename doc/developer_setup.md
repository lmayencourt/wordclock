# Developer setup

## VS Code configuration for Rust

Create a `.vscode/settings.json` with the following content:
````
{
    "rust-analyzer.linkedProjects": [
        "Cargo.toml",
        "crates/cross_compiled/Cargo.toml",
    ]
}
````

## Rust references

- [Rust by Example](https://doc.rust-lang.org/rust-by-example/index.html)
- [rustdoc book](https://doc.rust-lang.org/rustdoc/what-is-rustdoc.html)
- [Rust API Guidelines](https://rust-lang.github.io/api-guidelines/about.html)
- [Large Rust Workspaces](https://matklad.github.io/2021/08/22/large-rust-workspaces.html)