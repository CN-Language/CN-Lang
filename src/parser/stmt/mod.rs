//! Decl and Init are **NOT** STMT

struct If {
    cond: Expr,
    ifblock: Block,
    elseblock: Option<Block>,
}

struct While {
    cond: Expr,
    block: Block,
}

enum For {
    ForC {
        c1: Option<Expr>,
        c2: Option<Expr>,
        c3: Option<Expr>,
        block: Block,
    },
    ForIn {
        decl: Decl,
        from: Expr,
        block: Block,
    },
}

struct Block {}

struct Label {}

struct Goto {}

/// Expr with ;
struct ExprStmt {}

enum Stmt {
    If,
    While,
    For,
    Block,
    Label(String),
    Goto(),
    ExprStmt,
}
