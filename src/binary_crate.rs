use std::path::Path;

pub(crate) struct BinaryCrate<'a, const PERMANENT: bool> {
    pub(crate) path: &'a Path,
}

impl<'a, const PERMANENT: bool> Drop for BinaryCrate<'a, PERMANENT> {
    fn drop(&mut self) {
        if !PERMANENT {
            std::fs::remove_dir_all(self.path).unwrap();
        }
    }
}
