mod evcxr_source;
mod scriptlike_rust;
pub(crate) mod parsed_evcxr;

pub(crate) use evcxr_source::EvcxrSource;
pub(crate) use scriptlike_rust::ScriptlikeRust;
use parsed_evcxr::ParsedEvcxr;

pub fn execute_evcxr(s: String) {
    let src = EvcxrSource::from(s);
    let parsed = match src.parse() {
        Ok(parsed) => parsed,
        Err(e) => {
            eprintln!("{:?}", e);
            eprintln!("start: {:?}; end: {:?}", e.span().start(), e.span().end());
            return;
        }
    };
    parsed.create_project(std::env::current_dir().unwrap().join("output").as_path()).unwrap();
}
