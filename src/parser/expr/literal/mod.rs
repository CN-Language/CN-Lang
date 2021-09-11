use super::super::Literal;
use nom::{branch::alt, bytes::complete::tag, combinator::map, IResult};

// pub mod array;
pub mod char;
pub mod number;
pub mod string;
// pub mod struct_;

pub fn literal(s: &str) -> IResult<&str, Literal> {
    alt((
        // map(struct_::struct_, |s| Literal::Struct(s)),
        // map(array::array, |s| Literal::Array(s)),
        map(number::number, |s| Literal::Number(s)),
        map(string::string, |s| Literal::String(s)),
        //char
        //byte
        //bool
        // map(tag("Default"), |_| Literal::Default),
    ))(s)
}
