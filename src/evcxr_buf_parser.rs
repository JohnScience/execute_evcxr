use ouroboros::self_referencing;
use crate::{EvcxrSource, EnumeratedDepsLinsIter};

#[self_referencing]
struct EvcxrBufParser
{
    src: EvcxrSource,
    #[borrows(src)]
    #[not_covariant]
    enumerated_deps_lines_iter: EnumeratedDepsLinsIter<'this>,
}

impl From<EvcxrSource> for EvcxrBufParser {
    fn from(evcxr_src: EvcxrSource) -> Self {
        EvcxrBufParser::new(evcxr_src, EvcxrSource::build_enumerated_deps_lines_iter)
    }
}