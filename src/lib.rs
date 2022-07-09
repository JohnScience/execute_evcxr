pub(crate) mod binary_crate;
mod evcxr_source;
pub(crate) mod parsed_evcxr;
mod scriptlike_rust;

pub(crate) use binary_crate::BinaryCrate;
pub(crate) use evcxr_source::EvcxrSource;
use parsed_evcxr::ParsedEvcxr;
pub(crate) use scriptlike_rust::ScriptlikeRust;

pub fn execute_evcxr(s: String) {
    let src = EvcxrSource::from(s);
    let parsed_evcxr = match src.parse() {
        Ok(parsed) => parsed,
        Err(e) => {
            eprintln!("{:?}", e);
            eprintln!("start: {:?}; end: {:?}", e.span().start(), e.span().end());
            return;
        }
    };
    match parsed_evcxr.execute_via_binary_crate::</*KEEP_CRATE*/false>() {
        Ok(_) => (),
        Err(e) => {
            eprintln!("{:?}", e);
        }
    };
}
