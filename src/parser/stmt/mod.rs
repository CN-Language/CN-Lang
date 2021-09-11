use nom::{
    branch::alt,
    bytes::complete::tag,
    combinator::{opt, recognize},
    multi::{many1, separated_list1},
    sequence::{delimited, pair, tuple},
    IResult,
};

use super::{ident::ident, name::valid_name, ws};

pub fn template_param(s: &str) -> IResult<&str, &str> {
    recognize(delimited(
        tag("<"),
        separated_list1(tag(","), valid_name),
        tag(">"),
    ))(s)
}

pub fn args_list(s: &str) -> IResult<&str, &str> {
    recognize(delimited(tag("("), separated_list1(tag(","), ty), tag(")")))(s)
}

pub fn func_ptr_type(s: &str) -> IResult<&str, &str> {
    recognize(tuple((ty, tag("(*)"), template_param)))(s)
}

pub fn ptr(s: &str) -> IResult<&str, &str> {
    alt((tag("*"), tag("^")))(s)
}
pub fn ptrs(s: &str) -> IResult<&str, &str> {
    // recognize(many1(ptr))(s)
    tag("*")(s)
}
pub fn namespace(s: &str) -> IResult<&str, &str> {
    recognize(separated_list1(tag("::"), valid_name))(s)
}

pub fn normal_type(s: &str) -> IResult<&str, &str> {
    recognize(tuple((
        namespace,
        opt(ws),
        opt(template_param),
        opt(ws),
        opt(ptrs),
    )))(s)
}

pub fn ty(s: &str) -> IResult<&str, &str> {
    // mod ns::ns::ty<tmp,tmp> *
    // mod ret(*)<T>(arg,arg)
    alt((func_ptr_type, normal_type, tag("auto")))(s)
}

#[test]
fn test_ty() {
    // let tys = [
    //     "int",
    //     "bool",
    //     "long int",
    //     "unsigned short short",
    //     "std::int::int",
    // ];
    // for n in tys.iter().map(|s| normal_type(s)) {
    //     // assert_eq!(n.0, "");
    //     println!("{:?}", n)
    // }

    let tys = ["int(*)(int,int)"];
    for n in tys.iter().map(|s| func_ptr_type(s)) {
        // assert_eq!(n.0, "");
        println!("{:?}", n)
    }
}

pub fn dec(s: &str) -> IResult<&str, &str> {
    recognize(pair(ty, ident))(s)
}
