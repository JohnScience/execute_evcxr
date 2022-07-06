# Project structure

This a [Rust] project and it follows the [conventional package layout]. 

<pre><code>
execute_evcxr
│   STRUCTURE.md - you're here; the project structure
│   <a href="https://en.wikipedia.org/wiki/README">README.md</a> - repository description; specified not by Rust package layout conventions but by <a href="https://docs.github.com/en/repositories/managing-your-repositorys-settings-and-features/customizing-your-repository/about-readmes">GitHub conventions</a>
│   <a href="https://doc.rust-lang.org/cargo/guide/cargo-toml-vs-cargo-lock.html">Cargo.lock</a> - machine-generated file containing the exact information about dependencies
│   <a href="https://doc.rust-lang.org/cargo/reference/manifest.html">Cargo.toml</a> - the manifest
│
└───src/ - the source directory
    │   lib.rs - the default library file
    │   ... - the remaining files will be described separately

</code></pre>

Last updated: 7/6/2022.

*Note: the links lead to explanations what's the purpose of each file conventionally and not for the project in question*

[Rust]: https://www.rust-lang.org/
[conventional package layout]: https://doc.rust-lang.org/cargo/guide/project-layout.html
