use proc_macro::TokenStream;
use std::io::Write;

mod evcxr_source;
pub(crate) mod parsed_evcxr;

use parsed_evcxr::ParsedEvcxr;
pub(crate) use evcxr_source::EvcxrSource;


// The problem is https://www.reddit.com/r/rust/comments/ac08ef/is_there_no_way_to_get_the_exact_string_within_a/
#[proc_macro]
pub fn execute_evcxr(ts: TokenStream) -> TokenStream {
    let src = EvcxrSource::from(ts);
    let parsed = src.parse();
    let mut file = std::fs::File::create("output.txt").unwrap();
    file.write_fmt(format_args!("{}", src.0)).unwrap();
    file.write_fmt(format_args!("{:?}", parsed.prefixed_dependencies)).unwrap();
    file.write_all(parsed.pure_rust.as_bytes()).unwrap();
    TokenStream::new()
}
