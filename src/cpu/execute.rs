use std::ops::{BitOr, BitAnd};

use crate::cpu::instruction::RotKind;

use super::{Cpu, Src8, Src16, Reg8, Flag, Reg16, u16_to_u8u8, u8u8_to_u16};
use super::instruction::{Instruction, AluOp, BitOp, ControlOp, Condition};


enum FlagUpdate {
    Set,
    Reset,
    Keep,
    AluSync,
}
use FlagUpdate::*;

impl From<bool> for FlagUpdate {
    fn from(value: bool) -> Self {
        if value { Self::Set } else { Self::Reset }
    }
}

static mut INSTR_COUNT : usize = 0;

impl Cpu {
    pub(super) fn execute(&mut self, instr: Instruction) {
        unsafe {
            let ic = INSTR_COUNT;
            INSTR_COUNT += 1;
        //     if ic % 4_190_000 == 0 {
                eprintln!("PC : 0x{:X} -- instr n° {ic}, {instr:05x?}", self.pc);
        //     }
        };
        use Instruction::*;
        match instr {
            Nop  => self.noop(),
            Stop => todo!(),
            Load8 { target, source } 
                 => self.load8(target, source),
            Halt => todo!(),
            Alu8(op, src)
                 => self.alu8(op, src),
            RotA(rot_kind)
                 => self.rot_a(rot_kind),
            Alu16(op, src)
                 => self.alu16(op, src),
            Control(op, cond)
                 => self.control(op, cond),
            Bit(op, src)
                 => self.bit_ops(op, src),
            Load16 { target, source }
                 => self.load16(target, source),
            DAA  => self.daa(),
            ComplA => self.compl_a(),
            CarryFlag { set }
                 => self.carry_flag(set),
            InterruptEnable { enable }
                 => self.interrupt_enable(enable),
            Pop(reg)  => self.pop_op(reg),
            Push(reg) => self.push_op(reg),
        }
    }

    fn pop(&mut self) -> u16 {
        let lo = self.read8(Src8::Reg16Addr(Reg16::SP));
        self.sp += 1;
        let hi = self.read8(Src8::Reg16Addr(Reg16::SP));
        self.sp += 1;
        u8u8_to_u16((lo, hi))
    }

    fn push(&mut self, val: u16) {
        let (lo, hi) = u16_to_u8u8(val);
        self.sp -= 1; 
        self.write8(Src8::Reg16Addr(Reg16::SP), hi);
        self.sp -= 1; 
        self.write8(Src8::Reg16Addr(Reg16::SP), lo);

    }

    fn set_flags(&mut self, z: FlagUpdate, n: FlagUpdate, h: FlagUpdate, c: FlagUpdate) {
        macro_rules! update_f {
            ($cond:ident, $flagname:expr, $aluflag:ident) => {
                let val = match $cond {
                    Set => true,
                    Reset => false,
                    Keep => self.get_flag($flagname),
                    AluSync => self.alu.$aluflag,
                };
                self.set_flag($flagname, val) 
            };
        }

        update_f!(z, Flag::Z, z_flag);
        update_f!(n, Flag::N, n_flag);
        update_f!(h, Flag::H, h_flag);
        update_f!(c, Flag::C, c_flag);        
    }

    fn noop(&mut self) {
        ()
    }

    fn alu8(&mut self, op: AluOp, src: Src8) {
        let lhs = self.read8(Reg8::A.into());
        let rhs = self.read8(src);
        let res = match op {
            AluOp::Add  => self.alu.add8(lhs, rhs),
            AluOp::AddC => self.alu.add8c(lhs, rhs, self.get_flag(Flag::C)),
            AluOp::Sub => self.alu.sub8(lhs, rhs),
            AluOp::SbC => self.alu.sub8c(lhs, rhs, self.get_flag(Flag::C)),
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
        self.set_flags(AluSync, AluSync, AluSync, if matches!(op, AluOp::Inc | AluOp::Dec) { Keep } else { AluSync });
    }

    fn rot_a(&mut self, rot_kind: RotKind) {
        let val = self.read8(Reg8::A.into());
        let carry = self.get_flag(Flag::C);
        let res = match rot_kind {
            RotKind::RotLeftCarry => (val << 1) | (carry as u8),
            RotKind::RotRightCarry => (val >> 1) | ((carry as u8) << 7),
            RotKind::RotLeft => val.rotate_left(1),
            RotKind::RotRight => val.rotate_right(1),
            _ => unreachable!(),
        };

        self.write8(Reg8::A.into(), res);
        match rot_kind {
            RotKind::RotLeftCarry
            | RotKind::RotLeft => self.set_flags(
                                        Reset,
                                        Reset,
                                        Reset,
                                        (val.bitand(1u8 << 7) != 0).into()),
            RotKind::RotRightCarry
            | RotKind::RotRight => self.set_flags(
                                        Reset,
                                        Reset,
                                        Reset,
                                        (val.bitand(1u8) != 0).into()),
            _ => unreachable!()
        };
    }

    fn load8(&mut self, target: Src8, source: Src8) {
        let val = self.read8(source);
        self.write8(target, val);
    }

    fn alu16(&mut self, op: AluOp, src: Src16) {
        let hl = self.read16(Reg16::HL.into());
        let val = self.read16(Reg16::HL.into());
        // `Add` corresponds to HL + src,
        // whereas `Inc` acts on src.
        let (res, dest) = match op {
            AluOp::Add => (self.alu.add16(hl, val), Reg16::HL.into()),
            AluOp::Inc => (val.wrapping_add(1), src),
            AluOp::Dec => (val.wrapping_sub(1), src),
            _ => unreachable!(),
        };
        self.write16(dest, res);
        match op {
            AluOp::Add => self.set_flags(Keep, Reset, AluSync, AluSync),
            AluOp::Inc | AluOp::Dec => (),
            _ => unreachable!()
        }
    }

    fn control(&mut self, op: ControlOp, cond: Condition) {
        let cond_val = match cond {
            Condition::NZ => !self.get_flag(Flag::Z),
            Condition::Z  =>  self.get_flag(Flag::Z),
            Condition::NC => !self.get_flag(Flag::C),
            Condition::C  =>  self.get_flag(Flag::C),
            Condition::None => true,
        };

        if cond_val {
            let next_pc = match op {
                ControlOp::Ret => self.pop(),
                ControlOp::RetI => self.pop(),
                ControlOp::Jump(dest) => dest,
                ControlOp::JumpHL => self.read16(Reg16::HL.into()),
                ControlOp::JumpRel(delta) => self.pc.wrapping_add(delta as u16),
                ControlOp::Call(addr) => {
                    self.push(self.pc);
                    addr
                },
                ControlOp::RST(i) => (0x08 * i) as u16,
            };
            self.pc = next_pc;
        };
    }

    fn bit_ops(&mut self, op: BitOp, src: Src8) {
        let val = self.read8(src);
        let carry = self.get_flag(Flag::C);
        let res = match op {
            BitOp::Rot(rot_kind) => match rot_kind {
                RotKind::RotLeftCarry => (val << 1) | (carry as u8),
                RotKind::RotRightCarry => (val >> 1) | ((carry as u8) << 7),
                RotKind::RotLeft => val.rotate_left(1),
                RotKind::RotRight => val.rotate_right(1),
                RotKind::ShiftLeftArith => val << 1,
                RotKind::ShiftRightArith => (val as i8 >> 1) as u8,
                RotKind::Swap => (val.bitand(0x0F) << 4) | (val.bitand(0xF0) >> 4),
                RotKind::ShiftRightLogic => val >> 1,
            },
            BitOp::GetBit(_) => val,
            BitOp::ResBit(i) => val.bitand(!(1u8 << i)),
            BitOp::SetBit(i) => val.bitor(1u8 << i),
        };

        self.write8(src, res);

        match op {
            BitOp::Rot(rot_kind) => match rot_kind {
                RotKind::RotLeftCarry
                | RotKind::RotLeft
                | RotKind::ShiftLeftArith => self.set_flags(
                                            (res == 0).into(),
                                            Reset,
                                            Reset,
                                            (val.bitand(1u8 << 7) != 0).into()),
                RotKind::RotRightCarry
                | RotKind::RotRight
                | RotKind::ShiftRightArith
                | RotKind::ShiftRightLogic => self.set_flags(
                                            (res == 0).into(),
                                            Reset,
                                            Reset,
                                            (val.bitand(1u8) != 0).into()),
                RotKind::Swap => self.set_flags(
                                            (res == 0).into(),
                                            Reset,
                                            Reset,
                                            Reset),
            },
            BitOp::GetBit(i) => self.set_flags(
                (val.bitand(1u8 << i) == 0).into(),
                Reset,
                Set,
                Keep),
            _ => (),
        }
    }

    fn load16(&mut self, target: Src16, source: Src16) {
        let val = self.read16(source);
        self.write16(target, val);
    }

    /// DAA: "Decimal adjust accumulator".
    /// Convert a number to binary-coded decimal after an operation.
    /// 
    fn daa(&mut self) {
        todo!();
        let val = self.a;
        let carry = self.get_flag(Flag::C);
        let mut next_carry = false;
        let res = if self.get_flag(Flag::N) {
            // Last op was a substraction
            0
        } else {
            // Last op was an add
            if carry || val > 0x99 {
                val + 0x60
            } else {
                val
            }
        };
        self.write8(Reg8::A.into(), res);

        self.set_flags((res == 0).into(), Keep, Reset, next_carry.into());
    }

    fn compl_a(&mut self) {
        let val = self.read8(Reg8::A.into());
        let res = !val;
        self.write8(Reg8::A.into(), res);
    
        self.set_flags(Keep, Set, Set, Keep);
    }

    fn carry_flag(&mut self, set: bool) {
        let val = if set { true.into() } else { self.get_flag(Flag::C).into() };
        self.set_flags(Keep, Reset, Reset, val);
    }

    fn interrupt_enable(&mut self, enable: bool) {
        eprintln!("Interrupts! {enable}");
    }

    fn pop_op(&mut self, reg: Reg16) {
        let val = self.pop();
        self.write16(reg.into(), val);
    }

    fn push_op(&mut self, reg: Reg16) {
        let val = self.read16(reg.into());
        self.push(val);
    }
}


#[cfg(test)]
mod execute_panic_tests;

#[cfg(test)]
mod execute_tests;
