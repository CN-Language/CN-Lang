use lazy_static::*;
use nom::{
    bytes::complete::{take, take_till, take_till1},
    combinator::recognize,
    sequence::{pair, tuple},
    IResult,
};
use std::{
    fs::{read_to_string, File},
    io::Write,
};

lazy_static! {
    static ref avaliable_chars: Vec<char> = {
        let string = read_to_string("chars_lit.txt").unwrap();
        string.chars().into_iter().collect::<Vec<_>>()
    };
}

pub fn valid_start(c: char) -> bool {
    !avaliable_chars[9..].binary_search(&c).is_ok()
}
pub fn valid_tail(c: char) -> bool {
    !avaliable_chars.binary_search(&c).is_ok()
}
pub fn valid_name(s: &str) -> IResult<&str, &str> {
    recognize(pair(take_till1(valid_start), take_till(valid_tail)))(s)
}

#[test]
fn name() {
    let names = [
        "谢乔丽我爱你",
        "Lililikeuuu",
        "CamelCase",
        "__hidden",
        "😊",
        "🐢♂",
    ];
    for n in names.iter().map(|s| valid_name(s).unwrap()) {
        assert_eq!(n.0, "");
        println!("{}", n.1)
    }
}
