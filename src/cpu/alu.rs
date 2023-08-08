use std::ops::{BitXor, BitOr, BitAnd};

use super::{u16_to_u8u8, u8u8_to_u16};

pub(super) struct Alu {
    pub(super) z_flag: bool,
    pub(super) n_flag: bool,
    pub(super) h_flag: bool,
    pub(super) c_flag: bool,
}

impl Alu {
    fn set_zf(&mut self, val: bool) {
        self.z_flag = val;
    }
    
    fn set_nf(&mut self, val: bool) {
        self.n_flag = val;
    }

    fn set_hf(&mut self, val: bool) {
        self.h_flag = val;
    }

    fn set_cf(&mut self, val: bool) {
        self.c_flag = val;
    }

    pub(super) fn new() -> Alu {
        Alu {
            z_flag: false,
            n_flag: false,
            h_flag: false,
            c_flag: false
        }
    }
}

impl Alu {
    /// 3-way add.
    /// Can be used to implement all operations:
    /// add, add with carry, sub, sub with carry,
    /// as well as add/sub on 16 bits
    fn add8c_in(&mut self, lhs: u8, rhs: u8, carry_in: u8) -> u8 {
        let (tmp, carry) = lhs.overflowing_add(rhs);
        let (res, carry2) = tmp.overflowing_add(carry_in);

        let half_carry = (lhs & 0xF) + (rhs & 0xF) + carry_in > 0xF;
        self.set_hf(half_carry);
        self.set_cf(carry || carry2);
        self.set_zf(res == 0);
        res
    }

    pub(super) fn add8c(&mut self, lhs: u8, rhs: u8, carry: bool) -> u8 {
        self.set_nf(false);
        self.add8c_in(lhs, rhs.wrapping_neg(), carry as u8)
    }

    pub(super) fn add8(&mut self, lhs: u8, rhs: u8) -> u8 {
        self.set_nf(false);
        self.add8c_in(lhs, rhs, 0)
    }

    /// Sub
    pub(super) fn sub8(&mut self, lhs: u8, rhs: u8) -> u8 {
        self.set_nf(true);
        self.add8c_in(lhs, rhs.wrapping_neg(), 0)
    }

    /// Subc
    pub(super) fn sub8c(&mut self, lhs: u8, rhs: u8, carry: bool) -> u8 {
        self.set_nf(true);
        self.add8c_in(lhs, rhs.wrapping_neg(), (carry as u8).wrapping_neg())
    }

    pub(super) fn add16(&mut self, lhs: u16, rhs: u16) -> u16 {
        let (l_lo, l_hi) = u16_to_u8u8(lhs);
        let (r_lo, r_hi) = u16_to_u8u8(rhs);
        let res_lo = self.add8(l_lo, r_lo);
        let res_hi = self.add8c(l_hi, r_hi, self.c_flag);
        u8u8_to_u16((res_lo, res_hi))
    }

    pub(super) fn and8(&mut self, lhs: u8, rhs: u8) -> u8 {
        let res = lhs.bitand(rhs);
        self.set_zf(res == 0);
        self.set_nf(false);
        self.set_hf(true);
        self.set_cf(false);

        res
    }

    pub(super) fn or8(&mut self, lhs: u8, rhs: u8) -> u8 {
        let res = lhs.bitor(rhs);
        self.set_zf(res == 0);
        self.set_nf(false);
        self.set_hf(false);
        self.set_cf(false);

        res
    }

    pub(super) fn xor8(&mut self, lhs: u8, rhs: u8) -> u8 {
        let res = lhs.bitxor(rhs);
        self.set_zf(res == 0);
        self.set_nf(false);
        self.set_hf(false);
        self.set_cf(false);

        res
    }
}
