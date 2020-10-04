use std::{collections::HashMap, default};

use crate::{stack::{Stack, StackKind}, value::Value, Result};

#[derive(Clone, Debug)]
pub(crate) struct Block {
    // Represents the index of the stack at which the block is pushed
    pub(crate) stack_level: usize,

    // Represents the next instruction after the end of the block
    pub(crate) after_instr: usize,

    pub(crate) locals: HashMap<String, Value>,
}

impl Block {
    pub(crate) fn new(stack_level: usize, after_instr: usize) -> Self {
        Block { stack_level, after_instr, locals: HashMap::new() }
    }
}

#[derive(Debug)]
pub(crate) struct Frame {
    pub(crate) vals: Stack<Value>,
    pub(crate) frame_locals: HashMap<String, Value>,
    pub(crate) blocks: Stack<Block>,
}

impl default::Default for Frame {
    fn default() -> Self {
        Self { 
            vals: Stack::new(StackKind::Value), 
            frame_locals: HashMap::new(),
            blocks: Stack::new(StackKind::Block),
        }
    }

}

impl Frame {
    pub(crate) fn get_local(&self, name: &String) -> Option<&Value> {
        match self.blocks.top() {
            Ok(block) => block.locals.get(name),
            _ => match self.frame_locals.get(name) {
                Some(local) => Some(local),
                None => None 
            }
        }
    }
}
