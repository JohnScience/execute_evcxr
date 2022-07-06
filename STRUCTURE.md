# Project structure

This a [Rust] project and it follows the [conventional package layout]. 

<pre><code>
execute_evcxr
│   <a href="https://en.wikipedia.org/wiki/README">README.md</a> - specified not by Rust package layout conventions but by <a href="https://docs.github.com/en/repositories/managing-your-repositorys-settings-and-features/customizing-your-repository/about-readmes">GitHub conventions</a>
│   <a href="https://doc.rust-lang.org/cargo/guide/cargo-toml-vs-cargo-lock.html">Cargo.lock</a> - 
│   <a href="https://doc.rust-lang.org/cargo/reference/manifest.html">Cargo.toml</a> - the manifest
│
└───src/ - the source folder
│   │   lib.rs - the default library file
│   │   
│   │
│   └───subfolder1
│       │   file111.txt
│       │   file112.txt
│       │   ...
│   
└───folder2
    │   file021.txt
    │   file022.txt
</code></pre>

Last updated: 7/5/2022.

*Note: the links lead to explanations what's the purpose of each file conventionally and not for the project in question*

[Rust]: https://www.rust-lang.org/
[conventional package layout]: https://doc.rust-lang.org/cargo/guide/project-layout.html
