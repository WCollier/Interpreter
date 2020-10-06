use crate::{
    eval::Evaluator,
    frame::Frame,
    stack::{Stack, StackKind},
    Instr, Result,
};

#[derive(Debug)]
pub(crate) struct Inter {
    pub(crate) evaler: Evaluator,
    pub(crate) instrs: Vec<Instr>,
    pub(crate) frames: Stack<Frame>,
}

impl Inter {
    pub(crate) fn new() -> Result<Self> {
        let mut inter = Self {
            evaler: Evaluator::default(),
            instrs: vec![],
            frames: Stack::new(StackKind::Frame),
        };

        inter.frames.push(Frame::new()?)?;

        Ok(inter)
    }

    pub(crate) fn run(&mut self) -> Result {
        while self.evaler.pc < self.instrs.len() && self.evaler.running {
            let top_frame = self.frames.top_mut()?;

            if let Some(instr) = self.instrs.get(self.evaler.pc) {
                self.evaler.eval(top_frame, instr)?
            }
        }

        Ok(())
    }

    pub(crate) fn push_instrs(&mut self, instrs: &[Instr]) {
        instrs
            .to_vec()
            .drain(0..)
            .for_each(|instr| self.push_instr(instr));
    }

    fn push_instr(&mut self, instr: Instr) {
        self.instrs.push(instr);
    }
}
