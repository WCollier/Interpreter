use super::value::Value;

#[derive(Copy, Clone, Debug)]
pub(crate) enum CompareKind {
    Equal,
    NotEqual,
    LessThan,
    LassThanOrEqual,
    GreaterThan,
    GreaterThanOrEqual,
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
    PushScope(usize),
    PopScope,
}

#[cfg(test)]
mod test {
    use crate::{
        vm::{frame::Frame, Result}, 
        Inter
    };

    use super::{BinopKind, CompareKind, Instr, UnaryKind, Value};

    fn test_instrs(instrs: &[Instr]) -> Result<Inter> {
        let mut inter = Inter::new()?;

        inter.push_instrs(instrs);

        inter.run()?;

        Ok(inter)
    }

    pub(crate) fn top_frame(inter: &Inter) -> Result<&Frame> {
        inter.frames.top()
    }

    fn top_frame_mut(inter: &mut Inter) -> Result<&mut Frame> {
        inter.frames.top_mut()
    }

    #[test]
    fn push_works() -> Result {
        let inter = test_instrs(&[Instr::Push(Value::Int(400))])?;

        assert_eq!(top_frame(&inter)?.vals.top()?, &Value::Int(400));

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

        assert_eq!(top_frame(&inter)?.vals.top()?, &Value::Int(800));

        Ok(())
    }

    #[test]
    fn pop_works() -> Result {
        let inter = test_instrs(&[Instr::Push(Value::Int(400)), Instr::Pop])?;

        assert!(top_frame(&inter)?.vals.is_empty());

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

        assert_eq!(top_frame(&plus_inter)?.vals.top()?, &Value::Int(800));

        assert_eq!(top_frame(&minus_inter)?.vals.top()?, &Value::Int(0));

        assert_eq!(top_frame(&times_inter)?.vals.top()?, &Value::Int(160000));

        assert_eq!(top_frame(&divide_inter)?.vals.top()?, &Value::Int(1));

        assert_eq!(top_frame(&and_inter)?.vals.top()?, &Value::Bool(true));

        assert_eq!(top_frame(&or_inter)?.vals.top()?, &Value::Bool(true));

        Ok(())
    }

    #[test]
    fn unary_works() -> Result {
        let not_inter =
            test_instrs(&[Instr::Push(Value::Bool(true)), Instr::Unary(UnaryKind::Not)])?;

        assert_eq!(top_frame(&not_inter)?.vals.top()?, &Value::Bool(false));

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

        assert_eq!(top_frame(&equal_inter)?.vals.top()?, &Value::Bool(true));

        assert_eq!(top_frame(&ne_equal_inter)?.vals.top()?, &Value::Bool(false));

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

        assert_eq!(top_frame(&inter)?.vals.top()?, &Value::Int(100));

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
        assert!(top_frame(&inter)?.vals.is_empty());

        Ok(())
    }

    #[test]
    fn store_works() -> Result {
        /*
         * x = 100
         */
        let inter = test_instrs(&[Instr::Push(Value::Int(400)), Instr::Store("x".into())])?;

        assert!(top_frame(&inter)?
            .blocks
            .top()?
            .locals
            .contains_key("x".into()));

        Ok(())
    }

    #[test]
    fn store_global_works() -> Result {
        /*
         * x = 100
         */
        let inter = test_instrs(&[Instr::Push(Value::Int(400)), Instr::StoreGlobal("x".into())])?;

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

        assert_eq!(top_frame(&inter)?.vals.top()?, &Value::Int(400));

        Ok(())
    }

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
            Instr::Store("i".into()),   // 1
            // while
            Instr::PushScope(15), // 2
            // i != 3
            Instr::Load("i".into()),               // 3
            Instr::Push(Value::Int(3)),            // 4
            Instr::Compare(CompareKind::NotEqual), // 5
            Instr::PopJumpFalse(14),               // 6
            // x = 4
            Instr::Push(Value::Int(4)), // 7
            Instr::Store("x".into()),   // 8
            // i++
            Instr::Load("i".into()),       // 9
            Instr::Push(Value::Int(1)),    // 10
            Instr::Binop(BinopKind::Plus), // 11
            Instr::Store("i".into()),      // 12
            Instr::Jump(3),                // 13
            Instr::PopScope,               // 14
            Instr::Exit,                   // 15
        ])?;

        assert!(top_frame(&inter)?
            .blocks
            .top()?
            .locals
            .contains_key("i".into()));

        Ok(())
    }

    #[test]
    fn pop_block_works() -> Result {
        let inter = test_instrs(&[
            Instr::Push(Value::Int(0)), // 0
            Instr::PushScope(3),        // 1 setup loop pushes a block
            Instr::PopScope,            // 2
            Instr::Exit,                // 3
        ])?;

        assert!(top_frame(&inter)?.blocks.len() == 1);

        Ok(())
    }
}
