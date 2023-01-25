// use std::u8;

#[derive(Clone, Copy)]
#[repr(C)]
pub union Register {
    pub bytes: [u8; 2],
    pub short: u16
}

pub struct Cpu {
    // memory: Memory,
    memory: [u8; 42],
    /// Registers are stored as follows:
    /// `[bc, de, hl, af]`.
    /// The first letter describes the **higher** order byte,
    /// the second is the lower order byte.
    registers: [Register; 4],
    // af: u16,
    // bc: u16,
    // de: u16,
    // hl: u16,
    sp: u16,
    pc: u16
}

#[cfg(target_endian="little")]
impl Cpu {
    /// Compute the "r" table.
    /// See [here](http://www.z80.info/decoding.htm) for details.
    /// We use `(i+1)%2` because the system is little endian,
    /// hence the low byte is stored at index 0.
    pub fn r(&mut self, i: usize) -> &mut u8 {
        unsafe { 
            if i == 6 {
                &mut self.memory[self.registers[3].short as usize]
            }
            else { 
                &mut self.registers[i/2].bytes[(i+1)%2] 
            }
        }
    }
}

#[cfg(target_endian="big")]
impl Cpu {
    /// Compute the "r" table.
    /// See [here](http://www.z80.info/decoding.htm) for details.
    pub fn r(&mut self, i: usize) -> &mut u8 {
        unsafe { 
            if i == 6 {
                &mut self.memory[self.registers[3].short as usize]
            }
            else { 
                &mut self.registers[i/2].bytes[i%2] 
            }
        }
    }
}

impl Cpu {
    pub fn new() -> Self {
        Cpu {
            memory: [0; 42],
            registers: [Register {short: 0}; 4],
            sp: 0,
            pc: 0
        }
    }

    pub fn rp(&mut self, i: usize) -> &mut u16 {
        if i == 3 {
            &mut self.sp
        }
        else {
            unsafe {
                &mut self.registers[i].short
            }
        }
    }

    pub fn rp2(&mut self, i: usize) -> &mut u16 {
        unsafe {
            &mut self.registers[i].short
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Cpu;

    #[test]
    fn reg_bc() {
        let mut c = Cpu::new();
        *c.r(0) = 0x12;
        *c.r(1) = 0x34;
        unsafe {
            assert!(c.registers[0].short == 0x1234); 
        }
    }
}