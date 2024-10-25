#![warn(
    clippy::all,
    clippy::pedantic,
    rust_2018_idioms,
    rustdoc::all,
    rust_2024_compatibility,
    missing_docs
)]

mod parsing;
mod serializing;

use std::{path::PathBuf, str::FromStr};

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("{0}")]
    ParsingError(#[from] parsing::Error),
}

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

pub fn from_str(s: &str) -> Result<Shim, Error> {
    Ok(Shim::from_str(s)?)
}

pub fn to_string(shim: &Shim) -> Result<String, Error> {
    Ok(shim.to_string())
}
