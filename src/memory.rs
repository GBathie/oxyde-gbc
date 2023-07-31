use std::ops::{Index, IndexMut};

pub struct Memory {
    data: [u8; 80000]
}

impl Memory {
    pub fn new() -> Self {
        Memory {
            data: [0; 80000]
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