use std::path::PathBuf;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Rom {
    path: PathBuf,
}

impl Rom {
    pub const fn new(path: PathBuf) -> Self {
        Self { path }
    }

    pub const fn path(&self) -> &PathBuf {
        &self.path
    }
}
