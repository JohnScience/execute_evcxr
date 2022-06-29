use proc_macro::TokenStream;
use core::{
    iter::{Map, Enumerate, Filter},
    str::Lines,
};

pub(crate) struct EvcxrSource(pub String);

impl From<String> for EvcxrSource {
    fn from(s: String) -> Self {
        EvcxrSource(s)
    }
}

impl From<TokenStream> for EvcxrSource {
    fn from(ts: TokenStream) -> Self {
        let s = ts.to_string();
        EvcxrSource(s)
    }
}

impl Into<String> for EvcxrSource {
    fn into(self) -> String {
        self.0
    }
}

pub(super) type EnumeratedDepsLinsIter<'a> = Filter<
    Map<Enumerate<Lines<'a>>, fn((usize, &str)) -> (usize, &str)>,
    fn(&(usize, &str)) -> bool
>;

impl EvcxrSource {
    pub(crate) fn build_enumerated_deps_lines_iter<'a>(&'a self) -> EnumeratedDepsLinsIter<'a>
    {
        // The closures are coerced to function pointers so that their types could be named.
        //
        // Quoting the reference, "A closure expression produces a closure value with a unique,
        // anonymous type that cannot be written out.".
        //
        // Source: https://doc.rust-lang.org/reference/types/closure.html
        // 
        // The type of the whole iterator is needed so that ouroboros crate could handle the
        // field of the type of the iterator.
        let trim_lines: fn((usize, &str)) -> (usize, &str) = |(i, line)| (i, line.trim());
        let is_trimmed_dep_line: fn(&(usize, &str)) -> bool = |(_i, line)| line.starts_with(":dep");

        // The function is not written in functional style to highlight the gradual piling up of types
        let lines_iter: Lines = self.0.lines();
        let enum_lines_iter: Enumerate<Lines> = lines_iter.enumerate();
        let enum_trimmed_lines_iter: Map<Enumerate<Lines>, fn((usize, &str)) -> (usize, &str)> =
            enum_lines_iter.map(trim_lines);

        let enumerated_deps_lines_iter: EnumeratedDepsLinsIter = 
            enum_trimmed_lines_iter
            .filter(is_trimmed_dep_line);
        enumerated_deps_lines_iter
    }
}