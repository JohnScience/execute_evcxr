use std::{
    fs::{create_dir_all, File},
    io::Write,
    path::Path,
};

use crate::{BinaryCrate, ScriptlikeRust, Config};

const MANIFEST_CONTENTS_START: &str = r#"[package]
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
        W: std::io::Write,
    {
        const DEP_PREFIX_LEN: usize = ":dep ".len();

        for prefixed_dep in &self.prefixed_dependencies {
            let prefixless_dep = &prefixed_dep[DEP_PREFIX_LEN..];
            writeln!(dest, "{}", prefixless_dep)?;
        }
        Ok(())
    }

    pub(crate) fn create_binary_crate<'b>(
        &self,
        dir_path: &'b Path,
        config: &Config,
    ) -> std::io::Result<BinaryCrate<'b>> {
        // if the directories exist, nothing will happen
        create_dir_all(dir_path.join("src"))?;
        match File::create(dir_path.join("Cargo.toml")) {
            Ok(mut manifest) => {
                write!(&mut manifest, "{MANIFEST_CONTENTS_START}")?;
                self.write_deps_for_cargo(&mut manifest)?;
            }
            Err(e) => return Err(e),
        };
        match File::create(dir_path.join("src").join("main.rs")) {
            Ok(mut main_rs) => self.scriptlike_rust.write_as_main_rs(&mut main_rs)?,
            Err(e) => return Err(e),
        };
        Ok(BinaryCrate { path: dir_path, is_permanent: config.keep_binary_crate })
    }

    pub(crate) fn execute_via_binary_crate(&self, config: &Config) -> std::io::Result<()> {
        let epoch_time = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))?
            .as_millis();
        // The same script can be run in different doc tests simultaneously, so we need to make sure
        // that the binary crate is unique for each doc test because the author doesn't know how to
        // synchronize the execution of the script in different doc tests.
        let binary_crate_root = std::env::temp_dir()
            .join("execute_evcxr")
            .join(self.hash.to_string())
            .join(epoch_time.to_string());
        let binary_crate = self.create_binary_crate(&binary_crate_root, config)?;

        let mut cmd = std::process::Command::new("cargo");
        cmd.arg("run");
        if !config.verbose { cmd.arg("-q"); };
        cmd.current_dir(&binary_crate.path);

        cmd.status()?;
        Ok(())
    }
}
