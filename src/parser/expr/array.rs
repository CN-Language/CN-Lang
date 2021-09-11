use nom::{
    bytes::complete::tag, combinator::recognize, multi::separated_list0, sequence::tuple, IResult,
};

use crate::parser::Literal;

use super::literal;

pub fn array(s: &str) -> IResult<&str, Vec<Literal>> {
    let (o, (_, v, _)) = tuple((tag("{"), separated_list0(tag(","), literal), tag("}")))(s)?;
    Ok((o, v))
}

#[test]
fn test_array() {
    let arrays = [
        "{1,1,1}",
        "{\"aaa\",r#bbb
    cc
    😊#}",
        "{{1,2,3},{}}",
    ];
    for a in arrays.iter().map(|s| array(s).unwrap()) {
        assert_eq!(a.0, "");
        println!("{:?}", a.1);
    }
}
