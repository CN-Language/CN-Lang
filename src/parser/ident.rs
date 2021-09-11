use std::{collections::HashMap, hash::Hash};

use nom::{
    branch::alt, bytes::complete::tag, combinator::not, error::Error, sequence::pair, IResult,
};

use crate::lexer::*;

use super::name::valid_name;

pub fn ident(s: &str) -> IResult<&str, &str> {
    let (o, name) = valid_name(s)?;
    let k = key_word(name);
    match k {
        Ok(("", _)) => Err(nom::Err::Error(nom::error::Error::new(
            name,
            nom::error::ErrorKind::Not,
        ))),
        Ok((_, _)) => Ok((o, name)),
        Err(_) => Ok((o, name)),
    }
}

#[test]
fn test_valid_name() {
    let names = [
        "谢乔丽我爱你",
        "CamelCase",
        "__hidden",
        "😊",
        "🐢♂",
        "gotoa",
        "int1",
    ];
    for n in names.iter().map(|s| ident(s).unwrap()) {
        assert_eq!(n.0, "");
        println!("{}", n.1)
    }
}

#[test]
fn test_invalid_name() {
    let names = ["0谢乔丽我爱你", "int", "  aaa", "@😊", "!🐢♂"];
    for n in names.iter().map(|s| ident(s)) {
        assert!(n.is_err())
    }
}
