use nom::{
    branch::alt,
    bytes::complete::{tag, take_while1},
    character::{complete::digit1, is_digit},
    combinator::recognize,
    sequence::pair,
    IResult,
};

pub fn looping(s: &str) -> IResult<&str, &str> {
    alt((
        // looping
        tag("do"),
        tag("for"),
        tag("while"),
        tag("loop"),
        tag("continue"),
        tag("break"),
        tag("goto"),
    ))(s)
}
pub fn conditional(s: &str) -> IResult<&str, &str> {
    alt((
        // conditional
        tag("if"),
        tag("else"),
        tag("switch"),
        tag("case"),
        tag("match"),
    ))(s)
}
pub fn toplevel(s: &str) -> IResult<&str, &str> {
    alt((
        // top level
        tag("struct"),
        tag("module"),
        tag("import"),
        tag("pub"),
    ))(s)
}
pub fn ty(s: &str) -> IResult<&str, &str> {
    alt((
        // type
        tag("auto"),
        tag("void"),
        tag("bool"),
        recognize(pair(tag("i"), digit1)),
        recognize(pair(tag("u"), digit1)),
        tag("isize"),
        tag("usize"),
        // IEEE16 bit float
        tag("f16"),
        // brain16 1 sign 8 exp 7 frac
        tag("bf16"),
        tag("f32"),
        tag("f64"),
        tag("fp128"),
        tag("where"),
    ))(s)
}

pub fn value(s: &str) -> IResult<&str, &str> {
    alt((
        // value
        tag("default"),
        tag("nullptr"),
    ))(s)
}
pub fn error_handling(s: &str) -> IResult<&str, &str> {
    alt((
        // error handling
        tag("try"),
        tag("catch"),
        tag("throw"),
        tag("continue"),
        tag("break"),
    ))(s)
}

pub fn ptr(s: &str) -> IResult<&str, &str> {
    alt((tag("*"), tag("^")))(s)
}

pub fn key_word(s: &str) -> IResult<&str, &str> {
    alt((
        // allow user to override
        // must release the override table
        looping,
        conditional,
        toplevel,
        ty,
        value,
        error_handling,
    ))(s)
}
