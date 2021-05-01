use std::iter::Iterator;
use std::path;

pub struct LocalStateContext {
    pub paths: Vec<path::PathBuf>,
}

impl LocalStateContext {
    pub fn new() -> Self {
        Self { paths: Vec::new() }
    }

    pub fn set_paths<I>(&mut self, vals: I)
    where
        I: Iterator<Item = path::PathBuf>,
    {
        self.paths = vals.collect();
    }
}
