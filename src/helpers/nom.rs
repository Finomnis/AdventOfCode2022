use std::{
    fmt::{Debug, Display},
    ops::Deref,
};

use nom::{
    error::{convert_error, VerboseError},
    IResult, Needed,
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

pub type VResult<'a, T> = IResult<&'a str, T, VerboseError<&'a str>>;

pub fn finalize<I, T>(input: I) -> impl FnMut(Result<(I, T), nom::Err<VerboseError<I>>>) -> T
where
    I: Deref<Target = str> + Clone,
{
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

// Re-exports, for convenience
pub use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{anychar, char, line_ending, space0, space1, u16},
    combinator::map,
    multi::{count, many1_count, separated_list1},
    sequence::{delimited, pair, preceded, tuple},
};
