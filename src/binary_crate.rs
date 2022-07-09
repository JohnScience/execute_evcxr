use std::path::Path;

pub(crate) struct BinaryCrate<'a> {
    pub(crate) path: &'a Path,
    pub(crate) is_permanent: bool,
}

impl<'a> Drop for BinaryCrate<'a> {
    fn drop(&mut self) {
        if !self.is_permanent {
            std::fs::remove_dir_all(self.path).unwrap();
        }
    }
}
