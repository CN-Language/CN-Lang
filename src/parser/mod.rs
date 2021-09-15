use nom::{bytes::complete::take_till1, character::complete::one_of, IResult};

mod document;

mod name;

pub mod expr;
mod ident;
pub mod stmt;
mod ty;
mod utils;

trait Parse {
    fn parse(s: &str) -> IResult<&str, Self>
    where
        Self: Sized;
}

#[derive(Debug, PartialEq)]
pub enum Ident {
    Literal(Literal),
    Ident(String),
}
#[derive(Debug, PartialEq)]
pub enum Struct {
    InitSeq(Vec<Literal>),
    InitFields(Vec<(String, Ident)>),
}

#[derive(Debug, PartialEq)]
pub enum Number {
    Integer(i128),
    Floating(f64),
}
impl Number {
    pub fn int(self) -> i128 {
        match self {
            Number::Integer(r) => r,
            Number::Floating(_) => panic!("Unknown Error"),
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum Literal {
    Char(char),
    // Struct(Struct),
    // Array(Vec<Literal>),
    String(String),
    Number(Number),
    // Default,
}

pub fn is_ws(s: char) -> bool {
    match s {
        '\u{0009}' => true,
        '\u{000A}' => true,
        '\u{000B}' => true,
        '\u{000C}' => true,
        '\u{000D}' => true,
        '\u{0020}' => true,
        '\u{0085}' => true,
        '\u{200E}' => true,
        '\u{200F}' => true,
        '\u{2028}' => true,
        '\u{2029}' => true,
        _ => false,
    }
}

pub fn ws(s: &str) -> IResult<&str, &str> {
    // '\u{0009}' (horizontal tab, '\t')
    // '\u{000A}' (line feed, '\n')
    // '\u{000B}' (vertical tab)
    // '\u{000C}' (form feed)
    // '\u{000D}' (carriage return, '\r')
    // '\u{0020}' (space, ' ')
    // '\u{0085}' (next line)
    // '\u{200E}' (left-to-right mark)
    // '\u{200F}' (right-to-left mark)
    // '\u{2028}' (line separator)
    // '\u{2029}' (paragraph separator)
    take_till1(|c| !is_ws(c))(s)
}

#[test]
fn test_ws() {
    let wss = " \n \t \r ";
    println!("{:?}", ws(wss).unwrap())
}
