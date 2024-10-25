use std::{borrow::Cow, fmt::Display};

use crate::Shim;

impl Display for Shim {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let lines = Line::from_shim(self);

        for line in lines {
            Display::fmt(&line, f)?;
        }

        Ok(())
    }
}

struct Line<'a> {
    key: &'static str,
    value: Cow<'a, str>,
}

impl<'a> Line<'a> {
    fn from_shim(shim: &'a Shim) -> Vec<Self> {
        let mut lines = vec![Line {
            key: "path",
            value: shim.path().to_string_lossy(),
        }];

        if !shim.args().is_empty() {
            lines.push(Line {
                key: "args",
                value: Cow::Owned(shim.args().join(" ")),
            });
        }

        lines
    }
}

impl<'a> Display for Line<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} = {}", self.key, self.value)
    }
}
