use std::{
    path::Path,
    // fs::{File, write},
};
use syn::File;

use crate::EvcxrSource;

pub(crate) struct ParsedEvcxr<'a> {
    pub(crate) prefixed_dependencies: Vec<&'a str>,
    pub(crate) pure_rust: File,
}

impl<'a> ParsedEvcxr<'a> {
    pub(crate) fn create_project(&self, dir_path: &Path) -> std::io::Result<()> {
        // write(dir_path.join("main.rs"), );
        // Ok(())
        unimplemented!()
    }
}