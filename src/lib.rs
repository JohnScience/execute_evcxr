use proc_macro::TokenStream;

mod evcxr_source;
pub(crate) mod parsed_evcxr;

pub(crate) use evcxr_source::EvcxrSource;
use parsed_evcxr::ParsedEvcxr;

#[proc_macro]
pub fn execute_evcxr(evcxr_src: TokenStream) -> TokenStream {
    let src = EvcxrSource::from(evcxr_src);
    let parsed = ParsedEvcxr::from(src);
    // TokenStream::new()
    unimplemented!()
}
