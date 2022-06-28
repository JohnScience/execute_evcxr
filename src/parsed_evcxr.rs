use ouroboros::self_referencing;

use std::{
    path::Path,
    // fs::{File, write},
};

use crate::EvcxrSource;

#[self_referencing]
pub(crate) struct ParsedEvcxr {
    src: EvcxrSource,
    #[borrows(src)]
    #[covariant]
    dependencies: Vec<&'this str>,
}

impl From<EvcxrSource> for ParsedEvcxr {
    fn from(evcxr_src: EvcxrSource) -> Self {
        ParsedEvcxr::new(evcxr_src, EvcxrSource::parse_dependencies)
    }
}

impl ParsedEvcxr {
    pub(crate) fn create_project(&self, dir_path: &Path) -> std::io::Result<()> {
        // write(dir_path.join("main.rs"), );
        // Ok(())
        unimplemented!()
    }
}