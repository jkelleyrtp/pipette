# Pipette

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
