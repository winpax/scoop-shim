use std::{path::PathBuf, str::FromStr};

use crate::Shim;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("missing key")]
    MissingKey,
    #[error("missing value")]
    MissingValue,
    #[error("key must be a valid identifier (path or args)")]
    InvalidKey,

    #[error("path is missing. this is a required field")]
    MissingPath,
}

pub fn parse_line(line: &str) -> Result<Option<(&str, &str)>, Error> {
    let mut parts = line.splitn(2, '=');

    let key = parts.next().ok_or(Error::MissingKey)?.trim();
    let value = parts.next().ok_or(Error::MissingValue)?.trim();

    Ok(Some((key, value)))
}

fn parse_path(path: &str) -> PathBuf {
    PathBuf::from(path)
}

fn parse_args(args: &str) -> Vec<String> {
    args.split(' ').map(|s| s.to_string()).collect()
}

impl FromStr for Shim {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut path = None;
        let mut args = vec![];

        for line in s.lines() {
            let Some((key, value)) = parse_line(line)? else {
                continue;
            };

            match key {
                "path" => path = Some(parse_path(value)),
                "args" => args = parse_args(value),
                _ => return Err(Error::InvalidKey),
            }
        }

        let Some(path) = path else {
            return Err(Error::MissingPath);
        };

        Ok(Shim { path, args })
    }
}
