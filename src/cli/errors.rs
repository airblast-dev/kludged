use std::{error::Error, fmt::Display};

#[derive(Clone, Debug)]
pub struct InvalidColor;

impl Error for InvalidColor {}

impl Display for InvalidColor {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            r###"The provided color must be in hex format such as "#ff0012", or a named color such as "red"."###
        )
    }
}
