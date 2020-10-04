use std::{collections::HashMap, default};

use crate::{stack::{Stack, StackKind}, value::Value};

#[derive(Copy, Clone, Debug, Default)]
pub(crate) struct Block {
    // Represents the index of the stack at which the block is pushed
    pub(crate) stack_level: usize,

    // Represents the next instruction after the end of the block
    pub(crate) after_instr: usize,
}

#[derive(Debug)]
pub(crate) struct Frame {
    pub(crate) vals: Stack<Value>,
    pub(crate) locals: HashMap<String, Value>,
    pub(crate) blocks: Stack<Block>,
}

impl default::Default for Frame {
    fn default() -> Self {
        Self { 
            vals: Stack::new(StackKind::Value), 
            locals: HashMap::default(),
            blocks: Stack::new(StackKind::Block),
        }
    }
}
