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
    // Allow for empty lines and comments
    if line.is_empty() || line.starts_with('#') {
        return Ok(None);
    }

    let mut parts = line.splitn(2, '=');

    let key = parse_possible_string(parts.next().ok_or(Error::MissingKey)?.trim());
    let value = parse_possible_string(parts.next().ok_or(Error::MissingValue)?.trim());

    Ok(Some((key, value)))
}

fn parse_possible_string(key: &str) -> &str {
    key.trim_matches('"')
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

fn parse_path(path: &str) -> PathBuf {
    PathBuf::from(path)
}

fn parse_args(args: &str) -> Vec<String> {
    args.split(' ').map(|s| s.to_string()).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_possible_string() {
        let quoted_string = parse_possible_string("\"path\"");
        assert_eq!(quoted_string, "path");

        let unquoted_string = parse_possible_string("path");
        assert_eq!(unquoted_string, "path");
    }
}
