enum Ty {
    Primitive(Primitive),
    Custom(String),
}

enum Primitive {
    //     tag("auto"),
    // tag("void"),
    // tag("bool"),
    // recognize(pair(tag("i"), digit1)),
    // recognize(pair(tag("u"), digit1)),
    // tag("isize"),
    // tag("usize"),
    // // IEEE16 bit float
    // tag("f16"),
    // // brain16 1 sign 8 exp 7 frac
    // tag("bf16"),
    // tag("f32"),
    // tag("f64"),
    // tag("fp128"),
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

struct FuncPtrType {}

enum Ptr {
    PTR,
    VPTR,
}

struct NormalType {
    namespace: Option<Vec<String>>,
    ident: String,
    ptrs: Vec<Ptr>,
}
