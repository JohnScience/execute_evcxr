# Project structure

This a [Rust] project and it follows the [conventional package layout]. 

<pre><code>
execute_evcxr
│   STRUCTURE.md - project structure (package layout); semi-conventional. You're here
│   <a href="https://en.wikipedia.org/wiki/README">README.md</a> - library description (for library users); specified not by Rust package layout conventions but by <a href="https://docs.github.com/en/repositories/managing-your-repositorys-settings-and-features/customizing-your-repository/about-readmes">GitHub conventions</a>
│   <a href="https://en.wikipedia.org/wiki/README">DEV-README.md</a> - library description (for library developers); semi-conventional
│   <a href="https://doc.rust-lang.org/cargo/guide/cargo-toml-vs-cargo-lock.html">Cargo.lock</a> - machine-generated file containing the exact information about the <a href="https://doc.rust-lang.org/cargo/guide/dependencies.html">dependencies</a>
│   <a href="https://doc.rust-lang.org/cargo/reference/manifest.html">Cargo.toml</a> - the manifest (akin to package.json [JavaScript (npm)], requirements.txt [Python (pip)], or pom.xml [Java (Maven)])
│
└───src/ - the source directory
:   │   lib.rs - the default <a href="https://teamtreehouse.com/community/what-exactly-is-a-library-in-programming">library</a> file
:   │   ... - the remaining files will be described separately
:
.... target/ - the build directory that appears after either directly or indirectly building the package with Cargo
    :   ... - the contents of the build directory are machine-dependent and rarely should be of interest to the developer
</code></pre>

*Note: the links in the directory tree above lead to explanations of the purpose of each file conventionally and not for the project in question*

# Source code overview

*Note: In this context, evcxr will refer both to the [library providing the evaluation context](https://crates.io/crates/evcxr) and to its supported syntax, which is distinct from pure Rust.*

*Note: Checking 'README.md' for the purpose of the crate <sup><a href="https://doc.rust-lang.org/book/ch07-01-packages-and-crates.html">in Rust</a></sup> prior to further reading can be valuable.*

<pre><code>
execute_evcxr/src
│   🗎 lib.rs
│       The file with the definition <sup><a href="https://en.wikipedia.org/wiki/Declaration_(computer_programming)#Declaration_vs._definition">in CompSci</a></sup> of 'pub fn execute_evcxr<S: AsRef<str>>(s: S)' function <sup><a href="https://www.futurelearn.com/info/courses/programming-102-think-like-a-computer-scientist/0/steps/53095#:~:text=A%20function%20is%20simply%20a,which%20performs%20a%20particular%20task.">in CompSci</a></sup> similar to <a href="https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/eval">eval() [JavaScript]</a> or
│       <a href="https://github.com/google/evcxr/blob/main/evcxr/README.md">evcxr::EvalContext::eval() [Rust (evcxr)]</a>.
│   🗎 evcxr_source.rs
│       The file with pieces of code related to evcxr code parsing <sup><a href="https://en.wikipedia.org/wiki/Parsing#Computer_languages">in CompSci</a></sup>. It contains the definition of EvcxrSource <a href="https://en.wikipedia.org/wiki/Generic_programming#In_object-oriented_languages">generic</a> <a href="https://doc.rust-lang.org/book/ch03-02-data-types.html">type</a>-<a href="https://stackoverflow.com/a/38134190/8341513">"wrapper"</a> for for
│       String <sup><a href="https://en.wikipedia.org/wiki/String_(computer_science)">in CompSci</a>, <a href="https://doc.rust-lang.org/rust-by-example/std/str.html">in Rust By Example</a>, <a href="https://doc.rust-lang.org/std/string/struct.String.html">in Rust docs</a></sup>-like types as well as the <a href="https://en.wikipedia.org/wiki/Implementation#Computer_science">implementation</a> of parsing <sup><a href="https://en.wikipedia.org/wiki/Parsing#Computer_languages">in CompSci</a></sup> of the source.
│       Parsing of script-like Rust is delegated to the responsible data type 'ScriptlikeRust' that resides in a dedicated file.
│   🗎 scriptlike_rust.rs
│       The file with pieces of code related to script-like Rust supported by <a href="https://doc.rust-lang.org/rustdoc/what-is-rustdoc.html">'rustdoc'</a> <a href="https://doc.rust-lang.org/rustdoc/write-documentation/documentation-tests.html">doc tests</a>. It contains the definition of
│       ScriptlikeRust <a href="https://en.wikipedia.org/wiki/Generic_programming#In_object-oriented_languages">generic</a> <a href="https://doc.rust-lang.org/book/ch03-02-data-types.html">type</a>-<a href="https://stackoverflow.com/a/38134190/8341513">"wrapper"</a> for string-like types; the implementation of the conversion from
│       script-like Rust to pure Rust that is permissible in a <a href="https://doc.rust-lang.org/book/ch07-01-packages-and-crates.html#:~:text=a%20library%20crate.-,Binary%20crates,-are%20programs%20you">binary crate</a> via 'write_as_main_rs' function;
│       as well as the aforementioned parsing of script-like Rust, in the process of which the custom '#[expands_only_to_items]'
|       attribute gets removed.
|   🗎 binary_crate.rs
|       The file with the definition of BinaryCrate generic type-"wrapper" for '&'a Path', whose generic constant
|       parameter is the 'PERMANENT: bool', as well as custom implementation of <a href="https://doc.rust-lang.org/std/ops/trait.Drop.html">'Drop'</a> trait that enables cleanup. 
│   🗎 parsed_evcxr.rs
│       The file with pieces of code related to the "parsed evcxr" as the result of parsing evcxr. It contains the definition of
│       ParsedEvcxr type as well as the implementation of (1) creation of the binary crate from the "parsed evcxr", (2) execution
│       of evcxr via creation and running the binary crate.
</code></pre>

[Rust]: https://www.rust-lang.org/
[conventional package layout]: https://doc.rust-lang.org/cargo/guide/project-layout.html
