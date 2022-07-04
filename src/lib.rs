use std::io::Write;

mod evcxr_source;
mod scriptlike_rust;
pub(crate) mod parsed_evcxr;

use parsed_evcxr::ParsedEvcxr;
pub(crate) use evcxr_source::EvcxrSource;

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
    let mut file = std::fs::File::create("output.txt").unwrap();
    //file.write_fmt(format_args!("{}", src.0)).unwrap();
    //file.write_fmt(format_args!("{:?}", parsed.prefixed_dependencies)).unwrap();
    file.write_fmt(format_args!("{:#?}", parsed.pure_rust.items)).unwrap();
}
