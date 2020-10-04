use crate::{value::Value, frame::Block};

#[derive(Copy, Clone, Debug)]
pub(crate) enum CompareKind {
    Equal,
    NotEqual,
}

#[derive(Copy, Clone, Debug)]
pub(crate) enum BinopKind {
    Plus,
    Minus,
    Times,
    Divide,
    And,
    Or,
}

#[derive(Copy, Clone, Debug)]
pub(crate) enum UnaryKind {
    Not,
}

#[derive(Clone, Debug)]
pub(crate) enum Instr {
    Binop(BinopKind),
    Unary(UnaryKind),
    Print,
    Exit,
    Push(Value),
    Pop,
    Jump(usize),
    Compare(CompareKind),
    PopJumpFalse(usize),
    PopJumpTrue(usize),
    Store(String),
    StoreGlobal(String),
    Load(String),
    SetupLoop(usize),
    PopBlock,
}

#[cfg(test)]
mod test {
    use crate::{Inter, Result};

    use super::{BinopKind, CompareKind, Instr, UnaryKind, Value, Block};

    fn test_instrs(instrs: &[Instr]) -> Result<Inter> {
        let mut inter = Inter::new()?;

        inter.push_instrs(instrs);

        inter.run()?;

        Ok(inter)
    }

    /*
    #[test]
    fn push_works() -> Result {
        let inter = test_instrs(&[Instr::Push(Value::Int(400))])?;

        assert_eq!(inter.top_frame()?.vals.top()?, &Value::Int(400));

        Ok(())
    }

    #[test]
    fn jump_works() -> Result {
        // We should jump loading the 500 on to the stack, instead 800 is a the top
        let inter = test_instrs(&[
            Instr::Push(Value::Int(400)),
            Instr::Jump(3),
            Instr::Push(Value::Int(500)),
            Instr::Push(Value::Int(800)),
            Instr::Exit,
        ])?;

        assert_eq!(inter.top_frame()?.vals.top()?, &Value::Int(800));

        Ok(())
    }

    #[test]
    fn pop_works() -> Result {
        let inter = test_instrs(&[Instr::Push(Value::Int(400)), Instr::Pop])?;

        assert!(inter.top_frame()?.vals.is_empty());

        Ok(())
    }

    #[test]
    fn binop_works() -> Result {
        let plus_inter = test_instrs(&[
            Instr::Push(Value::Int(400)),
            Instr::Push(Value::Int(400)),
            Instr::Binop(BinopKind::Plus),
        ])?;

        let minus_inter = test_instrs(&[
            Instr::Push(Value::Int(400)),
            Instr::Push(Value::Int(400)),
            Instr::Binop(BinopKind::Minus),
        ])?;

        let times_inter = test_instrs(&[
            Instr::Push(Value::Int(400)),
            Instr::Push(Value::Int(400)),
            Instr::Binop(BinopKind::Times),
        ])?;

        let divide_inter = test_instrs(&[
            Instr::Push(Value::Int(400)),
            Instr::Push(Value::Int(400)),
            Instr::Binop(BinopKind::Divide),
        ])?;

        let and_inter = test_instrs(&[
            Instr::Push(Value::Bool(true)),
            Instr::Push(Value::Bool(true)),
            Instr::Binop(BinopKind::And),
        ])?;

        let or_inter = test_instrs(&[
            Instr::Push(Value::Bool(true)),
            Instr::Push(Value::Bool(true)),
            Instr::Binop(BinopKind::Or),
        ])?;

        assert_eq!(plus_inter.top_frame()?.vals.top()?, &Value::Int(800));

        assert_eq!(minus_inter.top_frame()?.vals.top()?, &Value::Int(0));

        assert_eq!(times_inter.top_frame()?.vals.top()?, &Value::Int(160000));

        assert_eq!(divide_inter.top_frame()?.vals.top()?, &Value::Int(1));

        assert_eq!(and_inter.top_frame()?.vals.top()?, &Value::Bool(true));

        assert_eq!(or_inter.top_frame()?.vals.top()?, &Value::Bool(true));

        Ok(())
    }

    #[test]
    fn unary_works() -> Result {
        let not_inter =
            test_instrs(&[Instr::Push(Value::Bool(true)), Instr::Unary(UnaryKind::Not)])?;

        assert_eq!(not_inter.top_frame()?.vals.top()?, &Value::Bool(false));

        Ok(())
    }

    #[test]
    fn exit_works() -> Result {
        let inter = test_instrs(&[
            Instr::Push(Value::Int(400)),
            Instr::Exit, // We want to exit before we print
            Instr::Print,
        ])?;

        assert!(!inter.evaler.running);

        Ok(())
    }

    #[test]
    fn compare_works() -> Result {
        let equal_inter = test_instrs(&[
            Instr::Push(Value::Bool(true)),
            Instr::Push(Value::Bool(true)),
            Instr::Compare(CompareKind::Equal),
        ])?;

        let ne_equal_inter = test_instrs(&[
            Instr::Push(Value::Bool(true)),
            Instr::Push(Value::Bool(true)),
            Instr::Compare(CompareKind::NotEqual),
        ])?;

        assert_eq!(equal_inter.top_frame()?.vals.top()?, &Value::Bool(true));

        assert_eq!(ne_equal_inter.top_frame()?.vals.top()?, &Value::Bool(false));

        Ok(())
    }

    #[test]
    fn pop_jump_false_works() -> Result {
        /*
         * x = 400
         * y = 400
         * if x == y {
         *   x = 100
         * }
         * exit
         */
        let inter = test_instrs(&[
            Instr::Push(Value::Int(400)),
            Instr::Push(Value::Int(400)),
            Instr::Compare(CompareKind::Equal),
            Instr::PopJumpFalse(5),
            Instr::Push(Value::Int(100)),
            Instr::Exit,
        ])?;

        assert_eq!(inter.top_frame()?.vals.top()?, &Value::Int(100));

        Ok(())
    }

    #[test]
    fn pop_jump_true_works() -> Result {
        /*
         * x = 400
         * y = 400
         * if x == y {
         *   exit
         * }
         * x = 100
         */
        let inter = test_instrs(&[
            Instr::Push(Value::Int(400)),
            Instr::Push(Value::Int(400)),
            Instr::Compare(CompareKind::Equal),
            Instr::PopJumpTrue(4),
            Instr::Exit,
            Instr::Push(Value::Int(100)),
        ])?;

        // By this point, 400, 400, and the true should've been popped,
        // leaving the stack empty
        assert!(inter.top_frame()?.vals.is_empty());

        Ok(())
    }

    #[test]
    fn store_works() -> Result {
        /*
         * x = 100
         */
        let inter = test_instrs(&[
            Instr::Push(Value::Int(400)),
            Instr::Store("x".into()),
        ])?;

        assert!(inter.top_frame()?.locals.contains_key("x".into()));

        Ok(())
    }

    #[test]
    fn store_global_works() -> Result {
        /*
         * x = 100
         */
        let inter = test_instrs(&[
            Instr::Push(Value::Int(400)),
            Instr::StoreGlobal("x".into()),
        ])?;

        assert!(inter.evaler.globals.contains_key("x".into()));

        Ok(())
    }

    #[test]
    fn load_works() -> Result {
        /*
         * x = 100
         * x
         */
        let inter = test_instrs(&[
            Instr::Push(Value::Int(400)),
            Instr::Store("x".into()),
            Instr::Load("x".into()),
        ])?;

        assert_eq!(inter.top_frame()?.vals.top()?, &Value::Int(400));

        Ok(())
    }
    */

    #[test]
    fn setup_loop_works() -> Result {
        /*
         * (We want x to no longer appear on the value stack)
         *
         * i = 0
         * while i != 3 {
         *   x = 4 
         *   i++
         * }
         */
        let inter = test_instrs(&[
            // i = 0
            Instr::Push(Value::Int(0)), // 0
            Instr::Store("i".into()), // 1

            // while
            Instr::SetupLoop(15), // 2

            // i != 3
            Instr::Load("i".into()), // 3
            Instr::Push(Value::Int(3)), // 4
            Instr::Compare(CompareKind::NotEqual), // 5
            Instr::PopJumpFalse(14), // 6

            // x = 4
            Instr::Push(Value::Int(4)), // 7
            Instr::Store("x".into()), // 8

            // i++
            Instr::Load("i".into()), // 9
            Instr::Push(Value::Int(1)), // 10
            Instr::Binop(BinopKind::Plus), // 11
            Instr::Store("i".into()), // 12 

            Instr::Jump(3), // 13

            Instr::PopBlock, // 14
            Instr::Exit // 15
        ])?;

        //println!("{:#?}", inter);

        //assert(inter.

        Ok(())
    }
}
