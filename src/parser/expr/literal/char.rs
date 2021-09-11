use crate::parser::expr::literal::string::escape_code;
use nom::{
    bytes::complete::{escaped, is_not, tag},
    combinator::map,
    sequence::delimited,
    IResult,
};
// TODO: limit one char
fn char(s: &str) -> IResult<&str, char> {
    map(
        delimited(
            tag("\'"),
            escaped(is_not("\"\\"), '\\', escape_code),
            tag("\'"),
        ),
        |s| s.chars().into_iter().next().unwrap(),
    )(s)
}
