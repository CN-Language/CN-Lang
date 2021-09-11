enum CPP {
    Include(String),
    Condition(),
}
enum Lit {
    Ident(String),
    Number(f64),
}
enum Op {
    Gt,
    Lt,
    Gteq,
    Lteq,
    Or,
    And,

    Add,
    Min,
    Mul,
    Div,
}
enum ConstEval {}
