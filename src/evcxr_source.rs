use std::{
    ops::Deref,
    str::{from_utf8_unchecked, FromStr}
};
use crate::ParsedEvcxr;
use syn::{parse2, parse::{Parse, Parser}};
use proc_macro2::TokenStream as TokenStream2;

pub(crate) struct EvcxrSource(pub String);

impl From<String> for EvcxrSource {
    fn from(s: String) -> Self {
        EvcxrSource(s)
    }
}

impl EvcxrSource {
    pub(crate) fn parse<'a>(&'a self) -> syn::Result<ParsedEvcxr<'a>> {
        let mut prefixed_dependencies = Vec::<&str>::with_capacity(255);
        for dep in self.0.lines()
            .map(|line| line.trim())
            .filter(|line| line.starts_with(":dep"))
        {
            prefixed_dependencies.push(dep);
        };
        let pure_rust = match prefixed_dependencies.last().map(Deref::deref) {
            None => "",
            Some(line) => {
                let src_ptr = self.0.as_ptr();
                let src_len = self.0.len();
                let line_ptr = line.as_ptr();
                let line_len = line.len();
                // TODO: use sub_ptr when available
                // https://doc.rust-lang.org/stable/std/primitive.pointer.html#method.sub_ptr
                let src_to_line_offset = unsafe { line_ptr.offset_from(src_ptr) };
                debug_assert!(src_to_line_offset >= 0);
                let pure_rust_ptr = unsafe { src_ptr.offset(src_to_line_offset + line_len as isize) };
                let pure_rust_len = src_len - src_to_line_offset as usize - line_len;
                unsafe { from_utf8_unchecked(std::slice::from_raw_parts(pure_rust_ptr, pure_rust_len)) }
                    .trim()
            }
        };
        let chars: Vec<char> = pure_rust.chars().collect();
        println!("{:?}", chars);
        println!("{:?}", pure_rust.as_bytes());
        let pure_rust_ts = TokenStream2::from_str(pure_rust)
            .map_err(|e| syn::Error::new_spanned(pure_rust, "Lexing error"))?;
        let pure_rust = Parser::parse2(syn::File::parse, pure_rust_ts)?;
        Ok(ParsedEvcxr { prefixed_dependencies, pure_rust })
    }
}