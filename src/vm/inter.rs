use super::{
    eval::Evaluator,
    instr::Instr,
    Result,
};

#[derive(Debug)]
pub(crate) struct Inter {
    pub(crate) evaler: Evaluator,
    pub(crate) instrs: Vec<Instr>,
}

impl Inter {
    pub(crate) fn new() -> Result<Self> {
        Ok(Self {
            evaler: Evaluator::new()?,
            instrs: vec![],
        })
    }

    pub(crate) fn run(&mut self) -> Result {
        while self.evaler.pc < self.instrs.len() && self.evaler.running {
            //let top_frame = self.frames.top_mut()?;

            if let Some(instr) = self.instrs.get(self.evaler.pc) {
                self.evaler.eval(instr)?
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
