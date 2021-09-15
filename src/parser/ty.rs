use std::{collections::HashMap, str::FromStr};

use nom::{
    branch::alt,
    bytes::complete::{tag, take_while1},
    character::complete::digit1,
    combinator::{map, opt},
    multi::separated_list1,
    sequence::{delimited, pair, tuple},
    IResult,
};

use crate::lexer;

use super::{
    ident::ident,
    is_ws,
    name::valid_name,
    utils::{list0, list1},
    ws, Parse,
};

#[derive(Debug)]
enum Ty {
    Primitive(Primitive),
    Custom {
        namespace: Option<Vec<String>>,
        ident: String,
    },
}

impl Parse for Ty {
    fn parse(s: &str) -> IResult<&str, Self> {
        alt((
            map(Primitive::parse, |p| Self::Primitive(p)),
            map(separated_list1(tag("::"), valid_name), |v| {
                let len = v.len();
                if len > 1 {
                    Self::Custom {
                        namespace: Some(
                            v[0..len - 1]
                                .iter()
                                .map(|s| String::from_str(s).unwrap())
                                .collect::<Vec<_>>(),
                        ),
                        ident: v[len - 1].to_owned(),
                    }
                } else {
                    Self::Custom {
                        namespace: None,
                        ident: v[len - 1].to_owned(),
                    }
                }
            }),
        ))(s)
    }
}

#[derive(Debug)]
enum Primitive {
    AUTO,
    VOID,
    BOOL,
    I(u64),
    U(u64),
    ISIZE,
    USIZE,
    F16,
    BF16,
    F32,
    F64,
    FP128,
}

impl Parse for Primitive {
    fn parse(s: &str) -> IResult<&str, Primitive> {
        alt((
            map(tag("auto"), |_| Primitive::AUTO),
            map(tag("void"), |_| Primitive::VOID),
            map(tag("bool"), |_| Primitive::BOOL),
            map(pair(tag("i"), digit1), |(_, s)| {
                Primitive::I(u64::from_str_radix(s, 10).unwrap())
            }),
            map(pair(tag("u"), digit1), |(_, s)| {
                Primitive::U(u64::from_str_radix(s, 10).unwrap())
            }),
            map(tag("isize"), |_| Primitive::ISIZE),
            map(tag("usize"), |_| Primitive::USIZE),
            // IEEE16 bit float
            map(tag("f16"), |_| Primitive::F16),
            // brain16 1 sign 8 exp 7 frac
            map(tag("bf16"), |_| Primitive::BF16),
            map(tag("f32"), |_| Primitive::F32),
            map(tag("f64"), |_| Primitive::F64),
            map(tag("fp128"), |_| Primitive::FP128),
        ))(s)
    }
}

#[derive(Debug)]
enum Ptr {
    PTR,
    VPTR,
}

#[derive(Debug)]
struct NormalType {
    ty: Ty,
    //TODO: template: const expr
    // template_param: Option<Vec<ConstExpr>>,
    ptrs: Option<Vec<Ptr>>,
}

impl Parse for NormalType {
    fn parse(s: &str) -> IResult<&str, NormalType> {
        map(tuple((Ty::parse, opt(ws), opt(ptrs))), |(ty, _, ptrs)| {
            NormalType { ty, ptrs }
        })(s)
    }
}

fn ptrs(s: &str) -> IResult<&str, Vec<Ptr>> {
    map(
        take_while1(|s| s == '*' || s == '^' || is_ws(s)),
        |s: &str| {
            let mut ptrs = vec![];
            for c in s.chars() {
                if c == '*' {
                    ptrs.push(Ptr::PTR);
                } else if c == '^' {
                    ptrs.push(Ptr::VPTR);
                }
            }
            ptrs
        },
    )(s)
}

#[test]
fn test_normal_ty() {
    let tys = [
        "a",
        "int",
        "a::b::c",
        "int **",
        "a::b::c *^*",
        "a::b::C *\t^\n*^",
    ];
    for res in tys.iter().map(|s| NormalType::parse(*s).unwrap()) {
        println!("{:?}", res.1);
        assert_eq!(res.0, "");
    }
}

#[derive(Debug)]
struct FunctionPtrType {
    ret: NormalType,
    args: Vec<NormalType>,
    where_clause: Option<HashMap<String, Type>>,
    ptrs: Option<Vec<Ptr>>,
}
fn where_clause(s: &str) -> IResult<&str, HashMap<String, Type>> {
    map(
        tuple((
            tag("where"),
            ws,
            map(
                list1("{", "}", |s| {
                    map(
                        tuple((ident, opt(ws), tag("="), opt(ws), Type::parse)),
                        |(i, _, _, _, t)| (i, t),
                    )(s)
                }),
                |v| {
                    let mut res = HashMap::new();
                    for (k, v) in v {
                        res.insert(k.to_owned(), v);
                    }
                    res
                },
            ),
        )),
        |(_, _, h)| h,
    )(s)
}

#[test]
fn test_where() {
    let w = "where {a = int, \n b = int,}";
    println!("{:?}", where_clause(w))
}

impl Parse for FunctionPtrType {
    fn parse(s: &str) -> IResult<&str, FunctionPtrType> {
        map(
            tuple((
                NormalType::parse,
                opt(ws),
                list0("(", ")", NormalType::parse),
                opt(ws),
                opt(where_clause),
                opt(ws),
                opt(ptrs),
            )),
            |(ret, _, args, _, where_clause, _, ptrs)| FunctionPtrType {
                ret,
                args,
                where_clause,
                ptrs,
            },
        )(s)
    }
}

#[test]
fn test_func_ptr_type() {
    let fptrs = [
        "void(int,int)*",
        r#"std::Result*
        ^(
            std::int::Integer*
            ^*,
            bool,    )
            **"#,
    ];
    for p in fptrs {
        println!("{:?}", FunctionPtrType::parse(p))
    }
}

#[derive(Debug)]
enum Type {
    FuncPtrType(FunctionPtrType),
    NormalType(NormalType),
}

impl Parse for Type {
    fn parse(s: &str) -> IResult<&str, Self>
    where
        Self: Sized,
    {
        alt((
            map(FunctionPtrType::parse, |f| Self::FuncPtrType(f)),
            map(NormalType::parse, |n| Self::NormalType(n)),
        ))(s)
    }
}

#[test]
fn test_type() {
    let tys = [
        // "int",
        // "void",
        // "a::b::c ^\t*",
        "void (int,int)*",
        r#"std::Result*
        ^(
            std::int::Integer*
            ^*,
            bool,    )
            **"#,
        // "auto (a,b)*",
        "callback (ty) where a = int;*",
    ];
    for t in tys {
        println!("{:?}", Type::parse(t))
    }
}
