use std::ops::{BitXor, BitOr, BitAnd};

use super::{Cpu, Reg8, Flag};

pub(super) struct Alu {
    h_flag: bool,
    c_flag: bool,
}

impl Alu {
    pub(super) fn flags(&self) -> u8 {
        let mut res = 0;
        if self.h_flag { res |= 0x20 };
        if self.c_flag { res |= 0x10 };
        res
    }

    fn get_hf(&self) -> bool {
        self.h_flag
    }
    
    fn get_cf(&self) -> bool {
        self.c_flag
    }

    fn set_hf(&mut self, val: bool) {
        self.h_flag = val;
    }

    fn set_cf(&mut self, val: bool) {
        self.c_flag = val;
    }
}


#[derive(Clone, Copy, Debug)]
pub(super) enum AluOp {
    Add,
    AddC,
    Sub,
    SbC,
    And,
    Xor,
    Or,
    Cmp,
    Inc,
    Dec,
}

impl Alu {
    /// 3-way add.
    /// Can be used to implement all operations:
    /// add, add with carry, sub, sub with carry,
    /// as well as add/sub on 16 bits
    fn add8c_in(&mut self, lhs: u8, rhs: u8, carry_in: u8) -> u8 {
        let (tmp, carry) = lhs.overflowing_add(rhs);
        let (res, carry2) = tmp.overflowing_add(carry_in);

        // self.set_flag(Flag::Z, res == 0);
        // self.set_flag(Flag::N, false);
        let half_carry = (lhs & 0xF) + (rhs & 0xF) + carry_in > 0xF;
        self.set_hf(half_carry);
        self.set_cf(carry || carry2);
        res
    }

    pub(super) fn add8c(&mut self, lhs: u8, rhs: u8) -> u8 {
        self.add8c_in(lhs, rhs.wrapping_neg(), self.c_flag as u8)
    }

    pub(super) fn add8(&mut self, lhs: u8, rhs: u8) -> u8 {
        self.add8c_in(lhs, rhs, 0)
    }

    /// Sub
    pub(super) fn sub8(&mut self, lhs: u8, rhs: u8) -> u8 {
        self.add8c_in(lhs, rhs.wrapping_neg(), 0)
        // self.set_flag(Flag::N, true);
    }

    /// Subc
    pub(super) fn sub8c(&mut self, lhs: u8, rhs: u8) -> u8 {
        self.add8c_in(lhs, rhs.wrapping_neg(), (self.c_flag as u8).wrapping_neg())
        // self.set_flag(Flag::N, true);
        // res
    }

    pub(super) fn add16(&mut self, (l_hi, l_lo): (u8, u8), (r_hi, r_lo): (u8, u8)) -> (u8, u8) {
        let res_lo = self.add8(l_lo, r_lo);
        let res_hi = self.add8c(l_hi, r_hi);
        (res_hi, res_lo)
    }

    pub(super) fn and8(&mut self, lhs: u8, rhs: u8) -> u8 {
        self.set_hf(true);
        self.set_cf(false);
        lhs.bitand(rhs)
    }

    pub(super) fn or8(&mut self, lhs: u8, rhs: u8) -> u8 {
        self.set_hf(false);
        self.set_cf(false);
        lhs.bitor(rhs)
    }

    pub(super) fn xor8(&mut self, lhs: u8, rhs: u8) -> u8 {
        self.set_hf(false);
        self.set_cf(false);
        lhs.bitxor(rhs)
    }
}


#[derive(Clone, Copy, Debug)]
pub(super) enum BitOp {
    Rot(RotKind),
    GetBit(usize),
    ResBit(usize),
    SetBit(usize),
}

#[derive(Clone, Copy, Debug)]
pub(super) enum RotKind {
    RotLeftCarry,
    RotRightCarry,
    RotLeft,
    RotRight,
    ShiftLeftArith,
    ShiftRightArith,
    ShiftLeftLogic,
    ShiftRightLogic,
}
