use std::{
    hash::{Hash, Hasher},
    ops::Deref,
    str::from_utf8_unchecked,
};
use syn::parse_str;

use crate::{ParsedEvcxr, ScriptlikeRust};

pub(crate) struct EvcxrSource(pub String);

impl From<String> for EvcxrSource {
    fn from(s: String) -> Self {
        EvcxrSource(s)
    }
}

impl EvcxrSource {
    pub(crate) fn parse<'a>(&'a self) -> syn::Result<ParsedEvcxr<'a>> {
        let mut hasher = std::collections::hash_map::DefaultHasher::new();
        self.0.hash(&mut hasher);
        let hash = hasher.finish();

        let mut prefixed_dependencies = Vec::<&str>::with_capacity(255);
        for dep in self
            .0
            .lines()
            .map(|line| line.trim())
            .filter(|line| line.starts_with(":dep"))
        {
            prefixed_dependencies.push(dep);
        }
        let scriptlike_rust = match prefixed_dependencies.last().map(Deref::deref) {
            None => self.0.as_str(),
            Some(line) => {
                let src_ptr = self.0.as_ptr();
                let src_len = self.0.len();
                let line_ptr = line.as_ptr();
                let line_len = line.len();
                // TODO: use sub_ptr when available
                // https://doc.rust-lang.org/stable/std/primitive.pointer.html#method.sub_ptr
                let src_to_line_offset = unsafe { line_ptr.offset_from(src_ptr) };
                debug_assert!(src_to_line_offset >= 0);
                let scriptlike_rust_ptr =
                    unsafe { src_ptr.offset(src_to_line_offset + line_len as isize) };
                let scriptlike_rust_len = src_len - src_to_line_offset as usize - line_len;
                unsafe {
                    from_utf8_unchecked(std::slice::from_raw_parts(
                        scriptlike_rust_ptr,
                        scriptlike_rust_len,
                    ))
                }
                .trim()
            }
        };
        let scriptlike_rust: ScriptlikeRust = parse_str(scriptlike_rust)?;
        Ok(ParsedEvcxr {
            prefixed_dependencies,
            scriptlike_rust,
            hash,
        })
    }
}
