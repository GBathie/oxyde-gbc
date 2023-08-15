use std::fs;


mod timer;
use timer::Timers;

pub struct Memory {
    data: [u8; 0x10000],
    timers: Timers
}

impl Memory {
    pub fn new() -> Self {
        Memory {
            data: [0; 0x10000],
            timers: Timers::new()
        }
    }

    pub fn from_file(filepath: &str) -> Memory {
        let bytes = fs::read(filepath).expect("Couldn't open file");
        let mut data = [0; 0x10000];
        data.clone_from_slice(&bytes.as_slice()[..0x10000]);
        Memory {
            data: data,
            timers: Timers::new()
        }
    }

    pub fn write<T: Into<usize>>(&mut self, addr: T, val: u8) {
        // self.data[addr.into()] = val;
        let index = addr.into();
        match index {
            0x0000..=0x3FFF => self.data[index] = val,
            0xFF04 => self.timers.set_div(val),
            0xFF05 => self.timers.tima = val,
            0xFF06 => self.timers.tma = val,
            0xFF07 => self.timers.set_tac(val),
            _ => self.data[index] = val
        };
    }

    pub fn read<T: Into<usize>>(&mut self, addr: T) -> u8 {
        let index: usize = addr.into();
        match index {
            0x0000..=0x3FFF => self.data[index],
            0xFF04 => self.timers.get_div(),
            0xFF05 => self.timers.tima,
            0xFF06 => self.timers.tma,
            0xFF07 => self.timers.get_tac(),
            _ => self.data[index]
        }
    }

}