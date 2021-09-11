use std::fs::File;
use std::io::read_to_string;

use nom::branch::alt;
use nom::bytes::complete::{escaped, is_a, is_not, tag, tag_no_case, take, take_until, take_while};
use nom::character::complete::{anychar, char, digit1, hex_digit1, none_of, one_of};
use nom::character::streaming::not_line_ending;
use nom::combinator::map;
use nom::error::Error;
use nom::sequence::{delimited, pair, tuple};
use nom::{IResult, Parser};

use crate::parser::{is_ws, ws};

pub fn escape_code(s: &str) -> IResult<&str, char> {
    one_of(r#"\"'nabfnrtvzx01234567"#)(s)
}

enum ControlChar {
    Byte(u8),
    IgnoreWhitespace,
    HexLit,
    OctLit,
    Codepoint,
}

pub fn rewrite<'a>(o: &'a str, st: &'a str) -> IResult<&'a str, String> {
    let mut res = String::new();
    let chars: Vec<char> = st.chars().collect();
    let mut i = 0;
    while i < chars.len() {
        if chars[i] == '\\' {
            if i + 1 != chars.len() {
                i += 1;
                let c = chars[i];
                match c.to_ascii_lowercase() {
                    '\\' => res.push(c),
                    '\'' => res.push(c),
                    '"' => res.push(c),
                    'n' => res.push('\n'),
                    'a' => res.push('\u{07}'), // bell
                    'b' => res.push('\u{08}'), // backspace
                    'f' => res.push('\u{0c}'), // form feed
                    'r' => res.push('\u{0d}'), // carriage return
                    't' => res.push('\u{09}'), // horizontal tab
                    'v' => res.push('\u{0b}'), // vertical tab
                    'x' => {
                        i += 1;
                        let mut string = String::new();
                        while i != chars.len()
                            && "0123456789abcdef"
                                .chars()
                                .collect::<Vec<char>>()
                                .contains(&chars[i].to_ascii_lowercase())
                        {
                            string.push(chars[i]);
                            i += 1;
                        }
                        i -= 1;
                        let c = u32::from_str_radix(&string, 16).unwrap();
                        println!("C is {}", c);
                        let c = char::from_u32(c).unwrap();
                        res.push(c);
                    }
                    c @ '0'..'9' => {
                        i += 1;
                        let mut string = String::new();
                        string.push(c);
                        while i != chars.len() && ('0'..'9').contains(&chars[i]) {
                            string.push(chars[i]);
                            i += 1;
                        }
                        let c = u32::from_str_radix(&string, 8).unwrap();
                        let c = char::from_u32(c).unwrap();
                        res.push(c);
                        i -= 1;
                    }
                    _ => {
                        return Err(nom::Err::Error(nom::error::Error::new(
                            o,
                            nom::error::ErrorKind::Escaped,
                        )));
                    }
                };
                i += 1;
            } else {
                return Err(nom::Err::Error(nom::error::Error::new(
                    o,
                    nom::error::ErrorKind::Escaped,
                )));
            }
        } else {
            res.push(chars[i]);
            i += 1;
        }
    }
    Ok((o, res))
}

#[test]
fn test_rewrite() {
    let toberewrite = r#"ass\n\v\t\r\n\rmother\n\'\"\101\x41"#;
    println!("{}", rewrite("", toberewrite).unwrap().1);
}

pub fn quoted_string(s: &str) -> IResult<&str, String> {
    let (o, i) = delimited(
        char('"'),
        escaped(is_not("\"\\"), '\\', escape_code),
        char('"'),
    )(s)?;
    rewrite(o, i)
}

pub fn raw_string(s: &str) -> IResult<&str, String> {
    let (i, o) = delimited(is_a("r#"), is_not("#"), is_a("#r"))(s)?;
    Ok((i, o.to_owned()))
}

pub fn string(s: &str) -> IResult<&str, String> {
    let (o, s) = alt((quoted_string, raw_string))(s)?;
    Ok((o, s.to_owned()))
}

pub fn string_rewrite(s: &str) -> String {
    let mut res = String::new();
    todo!()
}

#[test]
fn test_string() {
    let qs =
        read_to_string(&mut File::open("./src/parser/test_quoted_string.txt").unwrap()).unwrap();
    assert_eq!(quoted_string(&qs).unwrap().0, "");
    println!("{}", string(&qs).unwrap().1);
    let qs = read_to_string(&mut File::open("./src/parser/test_raw_string.txt").unwrap()).unwrap();
    assert_eq!(raw_string(&qs).unwrap().0, "");
    println!("{}", string(&qs).unwrap().1);
}
