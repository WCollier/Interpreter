pub(crate) type Result<T = ()> = std::result::Result<T, ErrorKind>;

mod eval;
mod instr;
mod inter;
mod stack;
mod value;
mod frame;

use crate::{
    instr::{Instr, BinopKind, CompareKind},
    inter::Inter,
    value::Value,
    stack::{StackKind, StackErrorKind},
};

#[derive(Clone, Debug)]
pub(crate) enum ErrorKind {
    StackError(StackKind, StackErrorKind),
    InvalidBinop { instr: Instr, l: Value, r: Value },
    InvalidUnary { instr: Instr, val: Value },
    InvalidJumpValue(Value),
    UnknownConst(String),
}

fn main() -> Result {
    let mut inter = Inter::new()?;

    /*
     * i = 0
     *
     * while i != 3:
     *     print(i)
     *
     *     i += 1
     */
    inter.push_instrs(&[
        // i = 0
        Instr::Push(Value::Int(0)),
        Instr::Store("i".into()),

        // while i != 3
        Instr::Load("i".into()),
        Instr::Push(Value::Int(3)),
        Instr::Compare(CompareKind::NotEqual),
        Instr::PopJumpFalse(13),

        // print i
        Instr::Load("i".into()),
        Instr::Print,

        // i += 1
        Instr::Load("i".into()),
        Instr::Push(Value::Int(1)),
        Instr::Binop(BinopKind::Plus),
        Instr::Store("i".into()),

        // to start of loop
        Instr::Jump(2),

        Instr::Exit,
    ]);

    inter.run()
}
