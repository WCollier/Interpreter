use crate::{ErrorKind, Result};

#[derive(Copy, Clone, Debug)]
pub(crate) enum StackKind {
    Value,
    Frame,
    Scope,
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub(crate) enum StackErrorKind {
    StackOverflow,
    StackUnderflow,
}

#[derive(Debug)]
pub(crate) struct Stack<T> {
    pub(crate) stack: Vec<T>,
    kind: StackKind,
}

impl<T> Stack<T> {
    pub(crate) fn new(kind: StackKind) -> Stack<T> {
        Stack {
            stack: vec![],
            kind,
        }
    }

    pub(crate) fn push(&mut self, value: T) -> Result {
        self.stack.push(value);

        Ok(())
    }

    pub(crate) fn pop(&mut self) -> Result<T> {
        self.stack.pop().ok_or_else(|| self.determine_stack_error())
    }

    pub(crate) fn top(&self) -> Result<&T> {
        self.stack
            .last()
            .ok_or_else(|| self.determine_stack_error())
    }

    pub(crate) fn top_mut(&mut self) -> Result<&mut T> {
        let err = self.determine_stack_error();

        self.stack.last_mut().ok_or_else(|| err)
    }

    pub(crate) fn is_empty(&self) -> bool {
        self.stack.is_empty()
    }

    pub(crate) fn truncate(&mut self, len: usize) {
        self.stack.truncate(len);
    }

    pub(crate) fn len(&self) -> usize {
        self.stack.len()
    }

    fn determine_stack_error(&self) -> ErrorKind {
        if self.is_empty() {
            ErrorKind::StackError(self.kind, StackErrorKind::StackUnderflow)
        } else {
            ErrorKind::StackError(self.kind, StackErrorKind::StackOverflow)
        }
    }
}
