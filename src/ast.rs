#[derive(Copy, Clone, Debug)]
pub(crate) enum BinopKind {
    Plus,
    Minus,
    Times,
    Divide,
}

#[derive(Clone, Debug)]
pub(crate) enum Expr {
    Number(i32),
    Binop(BinopKind, Box<Expr>, Box<Expr>),
}
