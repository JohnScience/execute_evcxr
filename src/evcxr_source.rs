use proc_macro::TokenStream;

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

impl EvcxrSource {
    pub(crate) fn parse_dependencies<'a>(&self) -> Vec<&str>
    {
        self.0.lines()
            .map(|line| line.trim())
            .filter(|trimmed| trimmed.starts_with(":dep"))
            .collect()
    }
}