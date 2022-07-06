# Project structure

This a [Rust] project and it follows the [conventional package layout]. 

<pre><code>
execute_evcxr
│   STRUCTURE.md - project structure (package layout); semi-conventional. You're here
│   <a href="https://en.wikipedia.org/wiki/README">README.md</a> - repository description; specified not by Rust package layout conventions but by <a href="https://docs.github.com/en/repositories/managing-your-repositorys-settings-and-features/customizing-your-repository/about-readmes">GitHub conventions</a>
│   <a href="https://doc.rust-lang.org/cargo/guide/cargo-toml-vs-cargo-lock.html">Cargo.lock</a> - machine-generated file containing the exact information about the <a href="https://doc.rust-lang.org/cargo/guide/dependencies.html">dependencies</a>
│   <a href="https://doc.rust-lang.org/cargo/reference/manifest.html">Cargo.toml</a> - the manifest (akin to package.json [JavaScript (npm)], requirements.txt [Python (pip)], or pom.xml [Java (Maven)])
│
└───src/ - the source directory
:   │   lib.rs - the default <a href="https://teamtreehouse.com/community/what-exactly-is-a-library-in-programming">library</a> file
:   │   ... - the remaining files will be described separately
:
.... target/ - the build directory that appears after either directory or indirectly building the package with Cargo
    :   ... - the contents of the build directory are machine-dependent and rarely should be of interest to the developer
</code></pre>

Last updated: 7/6/2022.

*Note: the links lead to explanations what's the purpose of each file conventionally and not for the project in question*

<pre><code>
execute_evcxr/src
│   
</code></pre>

[Rust]: https://www.rust-lang.org/
[conventional package layout]: https://doc.rust-lang.org/cargo/guide/project-layout.html
