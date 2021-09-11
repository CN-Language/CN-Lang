use nom::combinator::map;
use nom::{
    bytes::complete::{tag, take_until, take_while},
    combinator::not,
    sequence::delimited,
    IResult,
};

use super::is_ws;

fn ignore_ws(s: &str) -> IResult<&str, String> {
    let mut res = String::new();
    let (mut o, mut i) = take_while(|s| !is_ws(s))(s)?;
    res += i;
    if o != "" {
        let (oo, _) = take_while(is_ws)(o)?;
        o = oo;
        let (oo, s) = ignore_ws(o)?;
        o = oo;
        res += &s;
    }
    Ok((o, res))
}

fn list<'a, 'b, O, F: Fn(&str) -> O>(
    s: &'a str,
    parent: (&'b str, &'b str),
    f: F,
) -> IResult<&'a str, O> {
    delimited(tag(parent.0), map(ignore_ws, |s| f(&s)), tag(parent.1))(s)
}

#[test]
fn test_ignore_ws() {
    let s = "aaa   bb \t ccccc \n dd";
    println!("{:?}", ignore_ws(s));
}
