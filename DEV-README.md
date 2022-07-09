# For library devs

Ideally, [`execute_evcxr`] should cease to exist. It's purpose is to compensate for shortcomings of [`evcxr` kernel]. Consequently, spending too much time on the project is not recommended.

There is, however, a relatively non-dev-friendly [STRUCTURE.md] that exposes such details as what is the responsibility of each file in the source directory. In the ideal world, [STRUCTURE.md] should be machine-generated from some `src_description.json`. However, it's not and it can get outdated really fast. At the moment of writing, it is.

*Note: [STRUCTURE.md] was born because the girlfriend of the author was curious what it's like to code and the author found no better way to explain how he works on [`execute_evcxr`], as well as how [`execute_evcxr`] does its job.*

[`execute_evcxr`]: https://crates.io/crates/execute_evcxr
[`evcxr` kernel]: https://github.com/google/evcxr/tree/main/evcxr_jupyter
[STRUCTURE.md]: https://github.com/JohnScience/execute_evcxr/blob/main/STRUCTURE.md