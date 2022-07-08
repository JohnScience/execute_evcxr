use std::{
    path::Path,
    fs::{File, create_dir_all},
    io::Write
};

use crate::{ScriptlikeRust, BinaryCrate};

const MANIFEST_CONTENTS_START: &str =
r#"[package]
name = "anonymous"
version = "0.1.0"
edition = "2021"

# See more keys and their definitions at https://doc.rust-lang.org/cargo/reference/manifest.html

[dependencies]
"#;

pub(crate) struct ParsedEvcxr<'a> {
    pub(crate) prefixed_dependencies: Vec<&'a str>,
    pub(crate) scriptlike_rust: ScriptlikeRust<'a>,
    pub(crate) hash: u64,
}

impl<'a> ParsedEvcxr<'a> {
    pub(crate) fn write_deps_for_cargo<W>(&self, dest: &mut W) -> std::io::Result<()>
    where
        W: std::io::Write
    {
        const DEP_PREFIX_LEN: usize = ":dep ".len();

        for prefixed_dep in &self.prefixed_dependencies {
            let prefixless_dep = &prefixed_dep[DEP_PREFIX_LEN..];
            writeln!(dest, "{}", prefixless_dep)?;
        }
        Ok(())
    }

    pub(crate) fn create_binary_crate<'b, const PERMANENT: bool>(&self, dir_path: &'b Path) -> std::io::Result<BinaryCrate<'b, PERMANENT>> {
        // if the directories exist, nothing will happen
        create_dir_all(dir_path.join("src"))?;
        match File::create(dir_path.join("Cargo.toml")) {
            Ok(mut manifest) => {
                write!(&mut manifest, "{MANIFEST_CONTENTS_START}")?;
                self.write_deps_for_cargo(&mut manifest)?;
            },
            Err(e) => return Err(e),
        };
        match File::create(dir_path.join("src").join("main.rs")) {
            Ok(mut main_rs) => {
                self.scriptlike_rust.write_as_main_rs(&mut main_rs)?
            },
            Err(e) => return Err(e),
        };
        Ok(BinaryCrate { path: dir_path })
    }

    pub(crate) fn execute_via_binary_crate<const KEEP_CRATE: bool>(&self) -> std::io::Result<()> {
        let binary_crate_root = std::env::temp_dir().join("execute_evcxr").join(self.hash.to_string());
        let binary_crate = self.create_binary_crate::<KEEP_CRATE>(&binary_crate_root)?;
        
        let mut cmd = std::process::Command::new("cargo");
        cmd.arg("run");
        cmd.current_dir(&binary_crate.path);

        cmd.status()?;
        Ok(())
    }
}