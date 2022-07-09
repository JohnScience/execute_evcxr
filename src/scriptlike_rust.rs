use std::marker::PhantomData;

use quote::ToTokens;
use syn::{
    parse::{discouraged::Speculative, Parse, ParseStream},
    Item, Stmt,
};

// Ideally, the struct must store string slices into the source code.
// However, syn does not offer such parsing by default.
pub(crate) struct ScriptlikeRust<'a> {
    pub(crate) items: Vec<Item>,
    pub(crate) statements: Vec<Stmt>,
    pub(crate) phantom: PhantomData<&'a ()>,
}

impl<'a> Parse for ScriptlikeRust<'a> {
    // The struct must differentiate two kinds of Item::Macro,
    // those whose expansion contains only items and those that contain statements
    // (and possibly no items).
    fn parse(input: ParseStream) -> syn::parse::Result<Self> {
        let mut items = Vec::new();
        let mut statements = Vec::new();
        while !input.is_empty() {
            let cursor_snapshot = input.fork();
            match input.parse::<Item>().ok() {
                Some(item) => match item {
                    Item::Macro(mut i) => {
                        match i
                            .attrs
                            .iter()
                            .position(|attr| attr.path.is_ident("expands_only_to_items"))
                        {
                            Some(idx) => {
                                i.attrs.swap_remove(idx);
                                items.push(Item::Macro(i));
                            }
                            None => statements.push(Stmt::Item(Item::Macro(i))),
                        }
                    }
                    _ => items.push(item),
                },
                None => {
                    input.advance_to(&cursor_snapshot);
                    break;
                }
            }
        }
        while !input.is_empty() {
            match input.parse::<Stmt>() {
                Ok(stmt) => statements.push(stmt),
                Err(e) => return Err(e),
            }
        }
        Ok(ScriptlikeRust {
            items,
            statements,
            phantom: PhantomData,
        })
    }
}

impl<'a> ScriptlikeRust<'a> {
    pub(crate) fn write_as_main_rs<W>(&self, dest: &mut W) -> std::io::Result<()>
    where
        W: std::io::Write,
    {
        for item in self.items.iter() {
            match item {
                Item::Const(_i) => writeln!(dest, "/* Item::Const */"),
                Item::Enum(_i) => writeln!(dest, "/* Item::Enum */"),
                Item::ExternCrate(_i) => writeln!(dest, "/* Item::ExternCrate */"),
                Item::Fn(_i) => writeln!(dest, "/* Item::Fn */"),
                Item::ForeignMod(_i) => writeln!(dest, "/* Item::ForeignMod */"),
                Item::Impl(_i) => writeln!(dest, "/* Item::Impl */"),
                Item::Macro(_i) => writeln!(dest, "/* Item::Macro */"),
                Item::Macro2(_i) => writeln!(dest, "/* Item::Macro2 */"),
                Item::Mod(_i) => writeln!(dest, "/* Item::Mod */"),
                Item::Static(_i) => writeln!(dest, "/* Item::Static */"),
                Item::Struct(_i) => writeln!(dest, "/* Item::Struct */"),
                Item::Trait(_i) => writeln!(dest, "/* Item::Trait */"),
                Item::TraitAlias(_i) => writeln!(dest, "/* Item::TraitAlias */"),
                Item::Type(_i) => writeln!(dest, "/* Item::Type */"),
                Item::Union(_i) => writeln!(dest, "/* Item::Union */"),
                Item::Use(_i) => writeln!(dest, "/* Item::Use */"),
                Item::Verbatim(_i) => writeln!(dest, "/* Item::Verbatim */"),
                // when the item kind is unknown, it is necessary to check
                // https://docs.rs/syn/latest/syn/enum.Item.html
                _ => writeln!(dest, "/* unknown item */"),
            }?;
            writeln!(dest, "{}", item.into_token_stream())?;
        }
        writeln!(dest, "fn main(){{")?;
        for stmt in self.statements.iter() {
            writeln!(dest, "{}", stmt.into_token_stream())?;
        }
        writeln!(dest, "}}")?;
        Ok(())
    }
}
