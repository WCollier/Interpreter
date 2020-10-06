use std::{collections::HashMap, default};

use crate::{
    frame::{Scope, Frame},
    instr::{BinopKind, CompareKind, UnaryKind},
    stack::Stack,
    ErrorKind, Instr, Result, Value,
};

#[derive(Debug)]
pub(crate) struct Evaluator {
    pub(crate) pc: usize,
    pub(crate) running: bool,
    pub(crate) globals: HashMap<String, Value>,
}

impl default::Default for Evaluator {
    fn default() -> Self {
        Self {
            pc: 0,
            running: true,
            globals: HashMap::default(),
        }
    }
}

impl Evaluator {
    pub(crate) fn eval(&mut self, frame: &mut Frame, instr: &Instr) -> Result {
        self.pc += 1;

        match *instr {
            Instr::Binop(kind) => match kind {
                BinopKind::Plus => {
                    Evaluator::eval_num_binop(&mut frame.vals, instr, |l, r| Value::Int(l + r))
                }

                BinopKind::Minus => {
                    Evaluator::eval_num_binop(&mut frame.vals, instr, |l, r| Value::Int(l - r))
                }

                BinopKind::Times => {
                    Evaluator::eval_num_binop(&mut frame.vals, instr, |l, r| Value::Int(l * r))
                }

                BinopKind::Divide => {
                    Evaluator::eval_num_binop(&mut frame.vals, instr, |l, r| Value::Int(l / r))
                }

                BinopKind::And => {
                    Evaluator::eval_bool_binop(&mut frame.vals, instr, |l, r| Value::Bool(l && r))
                }

                BinopKind::Or => {
                    Evaluator::eval_bool_binop(&mut frame.vals, instr, |l, r| Value::Bool(l || r))
                }
            },

            Instr::Unary(kind) => match kind {
                UnaryKind::Not => match frame.vals.pop()? {
                    Value::Bool(val) => frame.vals.push(Value::Bool(!val)),
                    val => Err(ErrorKind::InvalidUnary {
                        instr: instr.clone(),
                        val,
                    }),
                },
            },

            Instr::Print => {
                println!("{}", frame.vals.pop()?);

                Ok(())
            }

            /*
             * TODO: Try and remove this clone here
             *
             * A clone is performed here because an instruction might be exectuted
             * multiple times, so we can't take ownership of it. However, we can
             * take reference. This means that a value being moved must be cloned
             * as the same value might used multiple times
             */
            Instr::Push(ref value) => frame.vals.push(value.clone()),
            Instr::Pop => frame.vals.pop().map(|_| ()),
            Instr::Exit => {
                self.running = false;

                Ok(())
            }
            Instr::Jump(new_pc) => {
                self.pc = new_pc;

                Ok(())
            }
            Instr::Compare(kind) => {
                let l = frame.vals.pop()?;

                let r = frame.vals.pop()?;

                match kind {
                    CompareKind::Equal => frame.vals.push(Value::Bool(l == r))?,
                    CompareKind::NotEqual => frame.vals.push(Value::Bool(l != r))?,
                };

                Ok(())
            }
            Instr::PopJumpFalse(new_pc) => self.eval_pop_jump(frame, new_pc, |val| !val),
            Instr::PopJumpTrue(new_pc) => self.eval_pop_jump(frame, new_pc, |val| val),
            Instr::Store(ref name) => {
                // TODO: Try and remove clone() here
                //frame.blocks.top_mut()?.locals.insert(name.to_string(), frame.vals.pop()?);
                let top = frame.vals.pop()?;

                match frame.get_local_mut(name) {
                    Some(local) => {
                        *local = top;

                        Ok(())
                    }
                    None => {
                        frame.blocks.top_mut()?.locals.insert(name.to_string(), top);

                        Ok(())
                    }
                }
            }
            Instr::StoreGlobal(ref name) => {
                // TODO: Try and remove clone() here
                self.globals.insert(name.to_string(), frame.vals.pop()?);

                Ok(())
            }
            Instr::Load(ref name) => {
                // Clone here to prevent compiler errors
                let val = frame.get_local(name).and_then(|val| Some(val.clone()));

                match val {
                    Some(val) => frame.vals.push(val),
                    None => match self.globals.get(name) {
                        Some(val) => frame.vals.push(val.clone()),
                        None => Err(ErrorKind::UnknownConst(name.to_string())),
                    },
                }
            }
            /*
            // TODO: Is this correct?
            Instr::Load(ref name) => match frame.blocks.top()?.locals.get(name) {
                // TODO: Try and remove these clones
                Some(val) => frame.vals.push(val.clone()),
                None => match self.globals.get(name) {
                    Some(val) => frame.vals.push(val.clone()),
                    None => Err(ErrorKind::UnknownConst(name.to_string())),
                }
            },
            */
            Instr::PushScope(after_instr) => {
                frame.blocks.push(Scope::new(frame.vals.len(), after_instr))
            }

            Instr::PopScope => {
                let block = frame.blocks.pop()?;

                frame.vals.truncate(block.stack_level);

                self.pc = block.after_instr;

                Ok(())
            }
        }
    }

    fn eval_bool_binop<F>(stack: &mut Stack<Value>, instr: &Instr, eval_fn: F) -> Result
    where
        F: FnOnce(bool, bool) -> Value,
    {
        match (stack.pop()?, stack.pop()?) {
            (Value::Bool(l), Value::Bool(r)) => stack.push(eval_fn(l, r)),
            (l, r) => Err(ErrorKind::InvalidBinop {
                instr: instr.clone(),
                l,
                r,
            }),
        }
    }

    fn eval_num_binop<F>(stack: &mut Stack<Value>, instr: &Instr, eval_fn: F) -> Result
    where
        F: FnOnce(i32, i32) -> Value,
    {
        match (stack.pop()?, stack.pop()?) {
            (Value::Int(l), Value::Int(r)) => stack.push(eval_fn(l, r)),
            // TODO: Try and remove the clone here
            (l, r) => Err(ErrorKind::InvalidBinop {
                instr: instr.clone(),
                l,
                r,
            }),
        }
    }

    fn eval_pop_jump<F>(&mut self, frame: &mut Frame, new_pc: usize, eval_fn: F) -> Result
    where
        F: FnOnce(bool) -> bool,
    {
        let top = frame.vals.pop()?;

        match top {
            Value::Bool(val) => {
                if eval_fn(val) {
                    self.pc = new_pc
                }

                Ok(())
            }
            _ => Err(ErrorKind::InvalidJumpValue(top)),
        }
    }
}
