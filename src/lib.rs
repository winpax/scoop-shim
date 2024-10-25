use std::path::PathBuf;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Shim {
    path: PathBuf,
    args: Vec<String>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_shim() {
        unimplemented!()
    }
}
