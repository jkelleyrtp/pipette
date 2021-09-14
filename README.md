# Pipette

<div>
  <!-- Crates version -->
  <a href="https://crates.io/crates/pipette">
    <img src="https://img.shields.io/crates/v/pipette.svg?style=flat-square"
    alt="Crates.io version" />
  </a>
  <!-- Downloads -->
  <a href="https://crates.io/crates/pipette">
    <img src="https://img.shields.io/crates/d/pipette.svg?style=flat-square"
      alt="Download" />
  </a>
  <!-- docs.rs docs -->
  <a href="https://docs.rs/pipette">
    <img src="https://img.shields.io/badge/docs-latest-blue.svg?style=flat-square"
      alt="docs.rs docs" />
  </a>
</div>

A small crate for using pipes in Rust.

```rust
let input = 1;

let output = pipette::pipe((
    input
    |a| a * 2,
    |a| a * 3,
    |a| a * 4,
    |a| a * 5,
    |a| a * 6,
    |a| a * 7,
    |a| a * 8,
));

assert_eq!(output, 40_320);
```

Pipette is unique:
- Supports polymorphic pipeline sizes
- Integrates well with IDE
- Does not require macros or custom traits


## License

This project is licensed under the [MIT license].

[MIT license]: https://github.com/tokio-rs/tokio/blob/master/LICENSE

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in Pipette by you, shall be licensed as MIT, without any additional
terms or conditions.
