[![crates.io](https://img.shields.io/crates/v/execute_evcxr.svg)][`execute_evcxr`]
[![crates.io](https://img.shields.io/crates/d/execute_evcxr.svg)][`execute_evcxr`]

# Execution of [`evcxr`]-flavored Rust

Further [`evcxr`] can refer to the [evaluation context](https://crates.io/crates/evcxr),
to [Jupyter](https://en.wikipedia.org/wiki/Project_Jupyter) [kernel](https://docs.jupyter.org/en/latest/projects/kernels.html) for [Rust programming language](https://www.rust-lang.org/), or to its supported script-like syntax.

## Example of [`evcxr`]-flavored Rust

```rust,no_run
:dep image = "0.23"
:dep evcxr_image = "1.1"
// In pure Rust project, the dependencies above would be specified in Cargo.toml
use evcxr_image::ImageDisplay;

image::ImageBuffer::from_fn(256, 256, |x, y| {
    if (x as i32 - y as i32).abs() < 3 {
        image::Rgb([0, 0, 255])
    } else {
        image::Rgb([0, 0, 0])
    }
})
```

At the moment of writing, the original [`evcxr` kernel] supports a lot but not all <sup><a href="https://github.com/google/evcxr/issues/165">1</a></sup> features of Rust. This crate has been born as a **temporary** solution to the problem.

## Note on [`evcxr`]-flavored Rust

The syntax supported by [`evcxr` kernel], as opposed to pure Rust, is implementation-defined. As a result, the same [`evcxr`]-flavored script can be executed differently by [`execute_evcxr`] and [`evcxr`] itself. In case of discrepancies, the behavior of [`evcxr`] is considered correct and the deviation on the side of [`execute_evcxr`] must be treated a bug, unless stated otherwise.

## Note on limitations

* As its name suggests, [`execute_evcxr`] is **not** an evaluation context in contrast to [`evcxr`]. It does **not** "return" a value. It executes a given script supplied in the form of string instead of attempting to evaluate it, unlike [`evcxr`].

* [`execute_evcxr`] gets its job done by parsing the supplied script only. It does **not** analyze the source code of the crates-dependencies. *This fact will become important further in the text.*

* Because [`execute_evcxr`] is a **temporary** solution, it does not try to memoize which macros were defined in the script and to which kinds of syntactic entities it would expand. *This fact will become important further in the text.*

* [`execute_evcxr`] works by parsing [`evcxr`]-flavored Rust, building a binary crate from it in the temporary dir, executing the binary crate, and cleaning up. Therefore, it must know which syntactic entities (or, more precisely, ["statements"](https://doc.rust-lang.org/reference/statements.html)) should go inside the `main()` function. *This fact will become important further in the text.*

* Due to the last three limitations above, the developer might need to annotate every macro invocation that eventually expands to ["items"](https://doc.rust-lang.org/reference/items.html) using `#[expands_only_to_items]` attribute. Otherwise, they will be placed inside main function. Luckily, the most common macros do **not** require the attribute and in many cases even if the macros do expands to ["items"](https://doc.rust-lang.org/reference/items.html) in the `main()`, the binary crate can still work as expected.

[`evcxr`]: https://github.com/google/evcxr/blob/main/evcxr_jupyter/samples/evcxr_jupyter_tour.ipynb
[`evcxr` kernel]: https://github.com/google/evcxr/tree/main/evcxr_jupyter