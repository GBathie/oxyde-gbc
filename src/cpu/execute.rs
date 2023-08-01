use super::{Cpu, instruction::Instruction, Src8, Src16};

impl Cpu {
    pub(super) fn execute(&mut self, instr: Instruction) {
        match instr {
            Instruction::Nop => todo!(),
            Instruction::Stop => todo!(),
            Instruction::Load8 { target, source } => self.load8(target, source),
            Instruction::Halt => todo!(),
            Instruction::Alu8(_, _) => todo!(),
            Instruction::RotA(_) => todo!(),
            Instruction::Alu16(_, _) => todo!(),
            Instruction::Control(_, _) => todo!(),
            Instruction::Bit(_, _) => todo!(),
            Instruction::Load16 { target, source } => self.load16(target, source),
            Instruction::DAA => todo!(),
            Instruction::ComplA => todo!(),
            Instruction::CarryFlag { set } => todo!(),
            Instruction::InterruptEnable { enable } => todo!(),
            Instruction::RST(_) => todo!(),
            Instruction::Pop(_) => todo!(),
            Instruction::Push(_) => todo!(),
        }
    }

    fn load8(&mut self, target: Src8, source: Src8) {
        let val = self.read8(source);
        self.write8(target, val);
    }

    fn load16(&mut self, target: Src16, source: Src16) {
        let val = self.read16(source);
        self.write16(target, val);
    }
}