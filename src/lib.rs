#![doc = include_str!("../README.md")]

pub(crate) mod binary_crate;
mod evcxr_source;
pub(crate) mod parsed_evcxr;
mod scriptlike_rust;
mod config;

pub(crate) use binary_crate::BinaryCrate;
pub(crate) use evcxr_source::EvcxrSource;
use parsed_evcxr::ParsedEvcxr;
pub(crate) use scriptlike_rust::ScriptlikeRust;
pub use config::Config;

pub fn execute_evcxr<C,S>(source: S, config: C)
where
    S: AsRef<str>,
    C: AsRef<Config>,
{
    let src = EvcxrSource::from(source);
    let parsed_evcxr = match src.parse() {
        Ok(parsed) => parsed,
        Err(e) => {
            eprintln!("{:?}", e);
            eprintln!("start: {:?}; end: {:?}", e.span().start(), e.span().end());
            return;
        }
    };
    match parsed_evcxr.execute_via_binary_crate(config.as_ref()) {
        Ok(_) => (),
        Err(e) => {
            eprintln!("{:?}", e);
        }
    };
}
