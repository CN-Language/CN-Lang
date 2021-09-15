pub fn rearange_operators() {}

enum operators {
    // dot elem and dot call are different
    // we 'd like to use explicit Dot instead of rust implicit dot
    Dot(),
    // a->b = (*a).b
    // a --> b = (**a).b
    Arrow(),
    // a[expr:T]
    Index(Expr),
    Call(Vec<Expr>),
    Inc(Expr),
    Dec(Expr),
    Cast(Type, Expr),
    /*
    1:
    as path
    a::b
    2:
    a.x
    a-*>x
    a[x]
    a(x)
    a++
    a--
    (b)a
    3:
    sizeof(a)
    ++a
    --a
    ~a
    !a
    -a
    +a
    &a
    *a
    new x
    delete x
    (x)
    5:
    a * b
    a / b
    a % b
    6:
    a + b
    a - b
    7:
    a << b
    a >> b
    8:
    a < b
    a > b
    a <= b
    a >= b
    9:
    ==
    !=
    10:
    a & b
    11:
    a ^ b
    12:
    a | b
    13:
    a && b
    14:
    a || b
    15:
    a ? b : c
    a = b
    a oprator= b
    throw
    a?
    */
}
