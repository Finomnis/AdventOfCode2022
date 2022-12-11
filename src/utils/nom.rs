use std::{
    fmt::{Debug, Display},
    ops::Deref,
};

use nom::{
    error::{convert_error, VerboseError},
    Needed,
};
use thiserror::Error;

#[derive(Error)]
struct NomParserError {
    msg: String,
}

impl Display for NomParserError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Display::fmt(&self.msg, f)
    }
}
impl Debug for NomParserError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Display::fmt(self, f)
    }
}

pub fn extract_nom_value<I: Deref<Target = str> + Clone, T>(
    input: I,
) -> impl FnMut(Result<(I, T), nom::Err<VerboseError<I>>>) -> T {
    move |res| {
        res.map(|(leftover, val)| {
            assert!(
                leftover.is_empty(),
                "Leftover input data after parsing:\n{:?}",
                &*leftover
            );
            val
        })
        .map_err(|e| match e {
            nom::Err::Incomplete(Needed::Unknown) => NomParserError {
                msg: "Incomplete: further input expected.".to_string(),
            },
            nom::Err::Incomplete(Needed::Size(needed)) => NomParserError {
                msg: format!("Incomplete: {} more characters expected.", needed),
            },
            nom::Err::Error(e) => NomParserError {
                msg: format!("Parser error!\n\n{}", convert_error(input.clone(), e)),
            },
            nom::Err::Failure(e) => NomParserError {
                msg: format!("Parsing failed!\n\n{}", convert_error(input.clone(), e)),
            },
        })
        .unwrap()
    }
}
