use crate::parser::ident::ident;
use nom::{
    branch::alt,
    bytes::complete::tag,
    combinator::{opt, recognize},
    multi::separated_list0,
    sequence::{pair, tuple},
    IResult,
};

use crate::parser::{ident, name::valid_name};

use super::{array, literal};
pub fn field_init(s: &str) -> IResult<&str, &str> {
    recognize(tuple((valid_name, tag(":"), ident)))(s)
}
pub fn cast(s: &str) -> IResult<&str, &str> {
    //TODO: valid_name -> type
    recognize(tuple((tag("("), valid_name, tag(")"))))(s)
}
pub fn struct_(s: &str) -> IResult<&str, &str> {
    alt((
        recognize(pair(
            opt(cast),
            alt((
                recognize(tuple((
                    tag("{"),
                    separated_list0(tag(","), field_init),
                    tag("}"),
                ))),
                //
            )),
        )),
        recognize(pair(opt(cast), array)),
    ))(s)
}

#[test]
fn test_struct() {
    let structs = ["default", "{a:1,b:c,c:{1,3},d:{a:b}}", "(Ty){1,2,\"C\"}"];
    for a in structs.iter().map(|s| struct_(s).unwrap()) {
        assert_eq!(a.0, "");
        println!("{:?}", a.1);
    }
}
