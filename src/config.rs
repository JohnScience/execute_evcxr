pub struct Config {
    pub verbose: bool,
    pub keep_binary_crate: bool,
}

impl AsRef<Config> for Config {
    fn as_ref(&self) -> &Config {
        self
    }
}

impl Default for Config {
    fn default() -> Self {
        Self {
            verbose: false,
            keep_binary_crate: false,
        }
    }
}