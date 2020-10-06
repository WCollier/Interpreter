use self::{
    instr::Instr,
    stack::{StackErrorKind, StackKind},
    value::Value,
};

pub mod eval;
pub mod frame;
pub mod instr;
pub mod inter;
pub mod stack;
pub mod value;

type Result<T = ()> = std::result::Result<T, ErrorKind>;

#[derive(Clone, Debug)]
pub(crate) enum ErrorKind {
    StackError(StackKind, StackErrorKind),
    InvalidBinop { instr: Instr, l: Value, r: Value },
    InvalidUnary { instr: Instr, val: Value },
    InvalidJumpValue(Value),
    UnknownConst(String),
}
