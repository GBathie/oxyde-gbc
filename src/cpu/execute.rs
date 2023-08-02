use super::{Cpu, Src8, Src16, Reg8, Flag};
use super::instruction::{Instruction, AluOp};
impl Cpu {
    pub(super) fn execute(&mut self, instr: Instruction) {
        match instr {
            Instruction::Nop => todo!(),
            Instruction::Stop => todo!(),
            Instruction::Load8 { target, source } => self.load8(target, source),
            Instruction::Halt => todo!(),
            Instruction::Alu8(op, src) => self.alu8(op, src),
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

    fn alu8(&mut self, op: AluOp, src: Src8) {
        let lhs = self.read8(Reg8::A.into());
        let rhs = self.read8(src);
        let res = match op {
            AluOp::Add  => self.alu.add8(lhs, rhs),
            AluOp::AddC => todo!("Add actual carry in"),//self.alu.add8c(lhs, rhs),
            AluOp::Sub => self.alu.sub8(lhs, rhs),
            AluOp::SbC => todo!("Add actual carry in"),
            AluOp::And => self.alu.and8(lhs, rhs),
            AluOp::Xor => self.alu.xor8(lhs, rhs),
            AluOp::Or  => self.alu.or8(lhs, rhs),
            AluOp::Cmp => self.alu.sub8(lhs, rhs),
            // Instructions inc/dec (opcodes like 0x04),
            // should act on the src register.
            AluOp::Inc => self.alu.add8(rhs, 1),
            AluOp::Dec => self.alu.sub8(rhs, 1),
        };

        // Store res as needed
        match op {
            AluOp::Inc | AluOp::Dec => self.write8(src, res),
            AluOp::Cmp => (),
            _ => self.write8(Reg8::A.into(), res),
        }

        // Update flags
        self.set_flag(Flag::Z, res == 0);
        todo!();

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