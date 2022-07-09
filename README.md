[![crates.io](https://img.shields.io/crates/v/execute_evcxr.svg)][`execute_evcxr`]
[![crates.io](https://img.shields.io/crates/d/execute_evcxr.svg)][`execute_evcxr`]

# Execution of [`evcxr`]-flavored Rust

Further [`evcxr`] can refer to the [evaluation context](https://crates.io/crates/evcxr),
to [Jupyter](https://en.wikipedia.org/wiki/Project_Jupyter) [kernel](https://docs.jupyter.org/en/latest/projects/kernels.html) for [Rust programming language](https://www.rust-lang.org/), or to its supported script-like syntax.

## Example of [`evcxr`]-flavored Rust

```rust,ignore
:dep image = "0.23"
:dep evcxr_image = "1.1"
// In a pure Rust project, the dependencies above would be specified in Cargo.toml
use evcxr_image::ImageDisplay;

image::ImageBuffer::from_fn(256, 256, |x, y| {
    if (x as i32 - y as i32).abs() < 3 {
        image::Rgb([0, 0, 255])
    } else {
        image::Rgb([0, 0, 0])
    }
})
```

# Example for [Jupyter Notebook] with [`evcxr` kernel]

```rust,ignore
:dep execute_evcxr = { version = "0.1.2" }

use execute_evcxr::{execute_evcxr, Config};

// This way it's possible to specify the non-default values
let config = Config { verbose: false, ..Default::default() };
execute_evcxr(r#"
:dep nalgebra = "0.31.0"
:dep nalgebra_latex = { version = "0.1.6", features = ["lin_sys", "evcxr"] }
 
use nalgebra::{matrix, Const};
use nalgebra_latex::{
    lin_sys::{
        LinSys,
        unknowns::SingleLetterBoldfaceVecOfUnknowns,
        numbering::Numbering,
        fmt::CasesLinSysFormatter,
    },
    fmt::EvcxrOutputFormatter,
};
use std::io::{stdout, Write};

let mut s = String::new();
let m = matrix!(
    1,2,3;
    4,5,6;
    7,8,9;
);
let vec_of_unknowns = SingleLetterBoldfaceVecOfUnknowns::<_,{Numbering::OneBased}>::new('x', Const::<3>);
let ls = LinSys::new(m, vec_of_unknowns);
CasesLinSysFormatter::write_evcxr_output(&mut s, &ls).unwrap();
stdout().write_all(s.as_bytes()).unwrap();
"#, config);
```

# Example for Rust project

```rust
extern crate execute_evcxr;

use execute_evcxr::{execute_evcxr, Config};

fn main() {
    let config = Config { ..Config::default() };
    execute_evcxr(r#"
:dep nalgebra = "0.31.0"
:dep nalgebra_latex = { version = "0.1.6", features = ["lin_sys", "evcxr"] }

use nalgebra::{matrix, Const};
use nalgebra_latex::{
    lin_sys::{
        LinSys,
        unknowns::SingleLetterBoldfaceVecOfUnknowns,
        numbering::Numbering,
        fmt::CasesLinSysFormatter,
    },
    fmt::EvcxrOutputFormatter,
};
use std::io::{stdout, Write};

let mut s = String::new();
let m = matrix!(
    1,2,3;
    4,5,6;
    7,8,9;
);
let vec_of_unknowns = SingleLetterBoldfaceVecOfUnknowns::<_,{Numbering::OneBased}>::new('x', Const::<3>);
let ls = LinSys::new(m, vec_of_unknowns);
CasesLinSysFormatter::write_evcxr_output(&mut s, &ls).unwrap();
stdout().write_all(s.as_bytes()).unwrap();
"#, config);
}
```

At the moment of writing, the original [`evcxr` kernel] supports a lot but not all <sup><a href="https://github.com/google/evcxr/issues/165">1</a></sup> features of Rust. This crate has been born as a **temporary** solution to the problem.

## Note on [`evcxr`]-flavored Rust

The syntax supported by [`evcxr` kernel], as opposed to pure Rust, is implementation-defined. As a result, the same [`evcxr`]-flavored script can be executed differently by [`execute_evcxr`] and [`evcxr`] itself. In case of discrepancies, the behavior of [`evcxr`] is considered correct and the deviation on the side of [`execute_evcxr`] must be treated a bug, unless stated otherwise.

## Note on limitations

* As its name suggests, [`execute_evcxr`] is **not** an evaluation context in contrast to [`evcxr`]. It does **not** "return" a value. It executes a given script supplied in the form of string instead of attempting to evaluate it, unlike [`evcxr`].

* [`execute_evcxr`] gets its job done by parsing the supplied script only. It does **not** analyze the source code of the crates-dependencies. *This fact will become important further in the text.*

* Because [`execute_evcxr`] is a **temporary** solution, it does not try to memoize which macros were defined in the script and to which kinds of syntactic entities it would expand. *This fact will become important further in the text.*

* [`execute_evcxr`] works by parsing [`evcxr`]-flavored Rust, building a [binary crate](https://doc.rust-lang.org/book/ch07-01-packages-and-crates.html#:~:text=a%20library%20crate.-,Binary%20crates,-are%20programs%20you) from it in the temporary dir, executing the binary crate, and cleaning up. Therefore, it must know which syntactic entities (or, more precisely, ["statements"](https://doc.rust-lang.org/reference/statements.html)) should go inside the `main()` function. *This fact will become important further in the text.*

* Due to the last three limitations above, the developer might need to annotate every macro invocation that eventually expands to ["items"](https://doc.rust-lang.org/reference/items.html) using `#[expands_only_to_items]` attribute. Otherwise, they will be placed inside main function. Luckily, the most common macros do **not** require the attribute and in many cases even if the macros do expands to ["items"](https://doc.rust-lang.org/reference/items.html) in the `main()`, the binary crate can still work as expected.

* Execution speed of the scripts is bounded to building them anew, including downloading dependencies from the internet. It can be a wise idea to set up caching. The library author hasn't faced the need for caching, yet.

# For library developers

This is a [README](https://en.wikipedia.org/wiki/README) for library users. There's a separate [DEV-README.md](https://github.com/JohnScience/execute_evcxr/blob/main/DEV-README.md) with information that can be relevant only to library developers.

[`execute_evcxr`]: https://crates.io/crates/execute_evcxr
[`evcxr`]: https://github.com/google/evcxr/blob/main/evcxr_jupyter/samples/evcxr_jupyter_tour.ipynb
[`evcxr` kernel]: https://github.com/google/evcxr/tree/main/evcxr_jupyter
[Jupyter Notebook]: https://en.wikipedia.org/wiki/Project_Jupyter#Jupyter_Notebook

# License

<sup>
Licensed under either of <a href="LICENSE-APACHE">Apache License, Version
2.0</a> or <a href="LICENSE-MIT">MIT license</a> at your option.
</sup>

<br>

<sub>
Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in this crate by you, as defined in the Apache-2.0 license, shall
be dual licensed as above, without any additional terms or conditions.
</sub>