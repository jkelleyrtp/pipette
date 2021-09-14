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
use pipette::{Pipeline, pipe};

let input = 1;

let output = pipe((
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

## How to use Pipette

Pipette uses trait-based polymorphism (the `Pipeline` trait) to make it easy to assemble closure-based pipelines in Rust. A single pipeline may consist of up to 12 closures and does not require static typing (ie pipe3, pipe4, pipe5 etc.). Instead, the Pipeline trait will allow any tuple, calling `compute` any any size of pipeline.

```rust
fn add_one(a: i32) -> i32 {
    a + 1
}

let r0 = pipe((0, add_one, add_one));
let r1 = pipe((0, add_one, add_one, add_one));
let r2 = pipe((0, add_one, add_one, add_one, add_one));
let r3 = pipe((0, add_one, add_one, add_one, add_one, add_one));

// lazy pipeline
let add_three = (0, add_one, add_one, add_one);

add_three.compute();
```
## License

This project is licensed under the [MIT license].

[MIT license]: https://github.com/tokio-rs/tokio/blob/master/LICENSE

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in Pipette by you, shall be licensed as MIT, without any additional
terms or conditions.
