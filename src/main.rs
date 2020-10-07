mod ast;
mod lexer;
mod parser;
mod vm;

use crate::vm::{
    instr::{BinopKind, CompareKind, Instr},
    inter::Inter,
    value::Value,
    ErrorKind as VmErrorKind,
};

use crate::{
    lexer::{ErrorKind as LexerErrorKind, Lexer},
    parser::{ErrorKind as ParserErrorKind, Parser},
};

type Result<T = ()> = std::result::Result<T, ErrorKind>;

#[derive(Debug)]
pub(crate) enum ErrorKind {
    VmError(VmErrorKind),
    LexerError(LexerErrorKind),
    ParserError(ParserErrorKind),
}

fn main() -> Result {
    let mut lexer = Lexer::new("(400+400)*2");

    let parser = Parser::new(lexer.run().map_err(|err| ErrorKind::LexerError(err))?);

    let parse = parser.parse().map_err(|err| ErrorKind::ParserError(err))?;

    println!("{:?}", parse);

    Ok(())

    /*
    let mut inter = Inter::new().map_err(|err| ErrorKind::VmError(err))?;

    /*
     * i = 0
     *
     * while i < 3:
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
        Instr::Compare(CompareKind::GreaterThan),
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

    /*
    inter.push_instrs(&[
        Instr::Push(Value::Int(400)),
        Instr::Store("x".into()),
        Instr::Load("x".into()),
        Instr::Print,
    ]);
    */

    inter.run().map_err(|err| ErrorKind::VmError(err))
        */
}
