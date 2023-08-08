use std::{ops::{Index, IndexMut}, fs::{self}};

pub struct Memory {
    data: [u8; 0x10000]
}

impl Memory {
    pub fn new() -> Self {
        Memory {
            data: [0; 0x10000]
        }
    }

    pub fn from_file(filepath: &str) -> Memory {
        let bytes = fs::read(filepath).expect("Couldn't open file");
        let mut data = [0; 0x10000];
        data.clone_from_slice(&bytes.as_slice()[..0x10000]);
        Memory {
            data: data,
        }
    }
    // pub fn load(&mut self, addr: u16) -> &mut u8 {
    //     &mut self.data[addr as usize]
    // }
}


impl<Idx : Into<usize>> Index<Idx> for Memory {
    type Output = u8;

    fn index(&self, index: Idx) -> &Self::Output {
        &self.data[index.into()]
    }
}

impl<Idx : Into<usize>> IndexMut<Idx> for Memory {
    fn index_mut(&mut self, index: Idx) -> &mut Self::Output {
        &mut self.data[index.into()]
    }
}