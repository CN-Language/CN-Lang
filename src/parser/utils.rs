use std::rc::Rc;

use nom::character::complete::digit1;
use nom::combinator::{map, opt};
use nom::error::Error;
use nom::multi::{separated_list0, separated_list1};
use nom::sequence::{pair, tuple};
use nom::{
    bytes::complete::{tag, take_until, take_while},
    combinator::not,
    sequence::delimited,
    IResult,
};

use super::{is_ws, ws};

pub fn list0<'a, O>(
    s: &'static str,
    e: &'static str,
    inner: impl Fn(&'a str) -> IResult<&'a str, O>,
) -> impl FnMut(&'a str) -> IResult<&'a str, Vec<O>> {
    delimited(
        pair(tag(s), opt(ws)),
        separated_list0(pair(tag(","), opt(ws)), inner),
        tuple((opt(ws), opt(tag(",")), opt(ws), tag(e))),
    )
}

pub fn list1<'a, O>(
    s: &'static str,
    e: &'static str,
    inner: impl Fn(&'a str) -> IResult<&'a str, O>,
) -> impl FnMut(&'a str) -> IResult<&'a str, Vec<O>> {
    delimited(
        pair(tag(s), opt(ws)),
        separated_list1(pair(tag(","), opt(ws)), inner),
        tuple((opt(ws), opt(tag(",")), opt(ws), tag(e))),
    )
}

#[test]
fn test_list() {
    let ls = "(\n\t\r1,     2,\n3,\t 4,5   , \t )";
    println!("{:?}", list0("(", ")", digit1)(ls))
}
