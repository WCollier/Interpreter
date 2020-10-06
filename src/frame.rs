use std::collections::HashMap;

use crate::{
    stack::{Stack, StackKind},
    value::Value,
    Result,
};

#[derive(Clone, Debug, PartialEq)]
pub(crate) struct Block {
    // Represents the index of the stack at which the block is pushed
    pub(crate) stack_level: usize,

    // Represents the next instruction after the end of the block
    pub(crate) after_instr: usize,

    //pub(crate) locals: HashMap<String, Value>,
    pub(crate) locals: HashMap<String, Value>,
}

impl Block {
    pub(crate) fn new(stack_level: usize, after_instr: usize) -> Self {
        Block {
            stack_level,
            after_instr,
            locals: HashMap::new(),
        }
    }
}

#[derive(Debug)]
pub(crate) struct Frame {
    pub(crate) vals: Stack<Value>,
    pub(crate) blocks: Stack<Block>,
}

impl Frame {
    pub(crate) fn new() -> Result<Self> {
        let mut frame = Self {
            vals: Stack::new(StackKind::Value),
            blocks: Stack::new(StackKind::Block),
        };

        // The frame needs an initial scope
        // After-instr is not needed, I think
        frame.blocks.push(Block::new(0, 0))?;

        Ok(frame)
    }

    pub(crate) fn get_local(&self, name: &String) -> Option<&Value> {
        for block in self.blocks.stack.iter().rev() {
            if let Some(val) = block.locals.get(name) {
                return Some(val);
            }
        }

        None

        /*
        let mut top_index = self.blocks.len() - 1;

        let mut top = self.blocks.get(top_index);

        while let Some(top_block) = top {
            if let Some(val) = top_block.locals.get(name) {
                return Some(val);
            }

            top_index -= 1;

            top = self.blocks.get(top_index);
        }

        None
        */
    }

    pub(crate) fn get_local_mut(&mut self, name: &String) -> Option<&mut Value> {
        for block in self.blocks.stack.iter_mut().rev() {
            if let Some(val) = block.locals.get_mut(name) {
                return Some(val);
            }
        }

        None
    }
}
