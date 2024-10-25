use std::borrow::Cow;

pub(crate) struct EscapedString<'a> {
    pub(crate) value: Cow<'a, str>,
    pub(crate) quoted: bool,
}

impl<'a> EscapedString<'a> {
    pub(crate) fn quote(&mut self) {
        self.quoted = true;
    }

    pub(crate) fn to_string(&'a self) -> Cow<'a, str> {
        if self.quoted {
            Cow::Owned(format!("\"{}\"", self.value))
        } else {
            match &self.value {
                Cow::Borrowed(value) => Cow::Borrowed(*value),
                Cow::Owned(value) => Cow::Borrowed(value),
            }
        }
    }

    pub(crate) fn quoted(self) -> Self {
        Self {
            quoted: true,
            ..self
        }
    }

    pub(crate) fn unquoted(self) -> Self {
        Self {
            quoted: false,
            ..self
        }
    }
}

impl<'a> From<Cow<'a, str>> for EscapedString<'a> {
    fn from(value: Cow<'a, str>) -> Self {
        Self {
            value,
            quoted: false,
        }
    }
}

impl<'a> From<&'a str> for EscapedString<'a> {
    fn from(value: &'a str) -> Self {
        Cow::Borrowed(value).into()
    }
}

impl<'a> From<String> for EscapedString<'a> {
    fn from(value: String) -> Self {
        Cow::<'a, str>::Owned(value).into()
    }
}

// impl<'a> std::fmt::Display for Quoted<'a> {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         if self.quoted {
//             Display::fmt(&'"', f)?;
//         }

//         Display::fmt(self.value.as_ref(), f)?;

//         if self.quoted {
//             Display::fmt(&'"', f)?;
//         }

//         Ok(())
//     }
// }
