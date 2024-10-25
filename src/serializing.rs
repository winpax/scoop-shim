use std::fmt::Display;

use quoted::Quoted;

use crate::Shim;

mod quoted;

impl Display for Shim {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let lines = Line::from_shim(self);

        for line in lines {
            Display::fmt(&line, f)?;
            Display::fmt(&"\r\n", f)?;
        }

        Ok(())
    }
}

struct Line<'a> {
    key: &'static str,
    value: Quoted<'a>,
}

impl<'a> Line<'a> {
    fn from_shim(shim: &'a Shim) -> Vec<Self> {
        let mut lines = vec![Line {
            key: "path",
            value: shim.path().to_string_lossy().into(),
        }];

        if !shim.args().is_empty() {
            lines.push(Line {
                key: "args",
                value: shim.args().join(" ").into(),
            });
        }

        lines
    }
}

impl<'a> Display for Line<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} = {}", self.key, self.value.to_string())
    }
}
