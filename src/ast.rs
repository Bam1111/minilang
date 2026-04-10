// ── AST ──────────────────────────────────────────────────────────────
#[derive(Debug, Clone)]
pub enum Expr {
    Number(f64),
    Bool(bool),
    Str(String),
    Var(String),
    BinOp { left: Box<Expr>, op: String, right: Box<Expr> },
    Call { name: String, args: Vec<Expr> },
}

#[derive(Debug, Clone)]
pub enum Stmt {
    Let { name: String, value: Expr },
    Print(Expr),
    If { cond: Expr, then_block: Vec<Stmt>, else_block: Vec<Stmt> },
    While { cond: Expr, block: Vec<Stmt> },
    Expr(Expr),
}