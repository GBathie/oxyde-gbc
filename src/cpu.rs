
use crate::memory::Memory;

mod instruction;
mod alu;

pub struct Cpu {
    memory: Memory,
    // Registers
    a: u8,
    f: u8,
    b: u8,
    c: u8,
    d: u8,
    e: u8,
    h: u8,
    l: u8,
    sp: u16,
    pc: u16,
    im: InterruptMode,
}


impl Cpu {
    pub fn new() -> Self {
        Cpu {
            memory: Memory::new(),
            // See [Pandocs](https://gbdev.io/pandocs/Power_Up_Sequence.html#cpu-registers)
            // for initial values of registers.
            a: 0x01,
            f: 0x00, // Not sure about this one, chose the value of DMG0
            b: 0xFF,
            c: 0x13,
            d: 0x00,
            e: 0xC1,
            h: 0x84,
            l: 0x03,
            sp: 0xFFFE,
            pc: 0x0100,
            //
            im: InterruptMode::Todo,
        }
    }

    pub fn run(&mut self) {
        loop {
            self.fetch_decode();
        }
    }

    fn fetch(&mut self) -> u8 {
        let val = self.memory[self.pc];
        self.pc += 1;
        val
    }
}


pub enum InterruptMode {
    Todo,
}


const OFFSET : usize = 0xFF00;
/// Interface to read from and write to the 8 bits/16bits registers
impl Cpu {
    fn read8(&self, src: Src8) -> u8 {
        match src {
            Src8::Register(reg) => {
                match reg {
                    Reg8::B => self.b,
                    Reg8::C => self.c,
                    Reg8::D => self.d,
                    Reg8::E => self.e,
                    Reg8::F => self.f,
                    Reg8::H => self.h,
                    Reg8::L => self.l,
                    Reg8::A => self.a,
                }
            }
            Src8::Const(val) => val,
            Src8::OffsetAddr(addr) => self.memory[OFFSET + addr as usize],
            Src8::ConstAddr(addr) => self.memory[addr],
            Src8::Reg16Addr(reg) => self.memory[self.read16(reg)],
        }
    }

    fn write8(&mut self, src: Src8, val: u8) {
        match src {
            Src8::Register(reg) => {
                match reg {
                    Reg8::B => self.b = val,
                    Reg8::C => self.c = val,
                    Reg8::D => self.d = val,
                    Reg8::E => self.e = val,
                    Reg8::F => self.f = val,
                    Reg8::H => self.h = val,
                    Reg8::L => self.l = val,
                    Reg8::A => self.a = val,
                }
            }
            Src8::Const(_) => panic!("Attempting to write a value into a constant"),
            Src8::OffsetAddr(addr) => self.memory[OFFSET + addr as usize] = val,
            Src8::ConstAddr(addr) => self.memory[addr] = val,
            Src8::Reg16Addr(reg) => {
                let addr = self.read16(reg);
                self.memory[addr] = val;
            },
        }
    }

    fn read16(&self, reg: Reg16) -> u16 {
        macro_rules! word_from {
            ($hi:ident, $lo:ident) => {
                ((self.$hi as u16) << 8) | (self.$lo as u16)
            };
        }        
        
        match reg {
            Reg16::BC => word_from!(b, c),
            Reg16::DE => word_from!(d, e),
            Reg16::HL => word_from!(h, l),
            Reg16::SP => self.sp,
            Reg16::AF => word_from!(a, f),
        }
    }

    fn write16(&mut self, reg: Reg16, val: u16) {
        macro_rules! to_word {
            ($hi:ident, $lo:ident) => {{
                self.$hi = (val >> 8) as u8; self.$lo = (val & 0xFF) as u8;
            }};
        }        
        
        match reg {
            Reg16::BC => to_word!(b, c),
            Reg16::DE => to_word!(d, e),
            Reg16::HL => to_word!(h, l),
            Reg16::SP => self.sp = val,
            Reg16::AF => to_word!(a, f),
        };
    }

    fn get_flag(&self, flag: Flag) -> bool {
        match flag {
            Flag::Z => (self.f & 0x80) != 0,
            Flag::H => (self.f & 0x40) != 0,
            Flag::N => (self.f & 0x20) != 0,
            Flag::C => (self.f & 0x10) != 0,
        }
    }

    fn set_flag(&mut self, flag: Flag, val: bool) {
        macro_rules! set_f {
            ($mask:literal) => {
                if val { self.f |= $mask } else { self.f &= !$mask }
            };
        }
        match flag {
            Flag::Z => set_f!(0x80),
            Flag::H => set_f!(0x40),
            Flag::N => set_f!(0x20),
            Flag::C => set_f!(0x10),
        };
    }

}


#[derive(Clone, Copy, Debug)]
enum Src8 {
    Register(Reg8),
    Const(u8),
    OffsetAddr(u8),
    ConstAddr(u16),
    Reg16Addr(Reg16),
}

impl From<Reg8> for Src8 {
    fn from(value: Reg8) -> Self {
        Self::Register(value)
    }
}

#[derive(Clone, Copy, Debug)]
enum Reg8 {
    B,
    C,
    D,
    E,
    F,
    H,
    L,
    A,
}

#[derive(Clone, Copy, Debug)]
enum Src16 {
    Register(Reg16),
    Const(u16),
    ConstAddr(u16),
    OffsetSP(i8),
}

#[derive(Clone, Copy, Debug)]
enum Reg16 {
    BC,
    DE,
    HL,
    SP,
    AF,
}



///
/// 
#[derive(Clone, Copy, Debug)]
enum ControlOp {
    Ret,
    Jump(u16),
    JumpRel(i8),
    Call(u16),
}

#[derive(Clone, Copy, Debug)]
enum Condition {
    NZ,
    Z,
    NC,
    C,
    None,
}

#[derive(Clone, Copy, Debug)]
enum Flag {
    Z,
    H,
    N,
    C
}


#[cfg(test)]
mod decode_tests;