mod parsing;

use std::path::PathBuf;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Shim {
    path: PathBuf,
    args: Vec<String>,
}

impl Shim {
    pub fn new(path: PathBuf, args: Vec<String>) -> Self {
        Self { path, args }
    }

    pub fn path(&self) -> &PathBuf {
        &self.path
    }

    pub fn args(&self) -> &[String] {
        &self.args
    }
}
