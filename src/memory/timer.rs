pub(super) struct Timers {
    /// DIV - Divider timer: increased at a rate of 16384Hz,
    /// (every 256 clock cycles ? Div is the high byte of counter ?)
    /// except while in stop mode.
    /// Writing to DIV resets it to 0x00,
    /// and also sets the whole "clock counter" to 0.
    counter: u16,
    /// TIMA - Controlled timer.  
    /// This timer is increased at a rate
    /// determined by the TAC register.
    /// When it overflows,
    /// a timer interrupt is raised (IF bit 2, 0-indexed)
    /// and TIMA is reset to the value in TMA. 
    pub tima: u8,
    pub tma: u8,
    tac: u8,
    tima_enabled: bool,
    tima_period: usize,
}
impl Timers {
    pub(crate) fn new() -> Timers {
        Timers {
            // div: 0,
            counter: 0,
            tima: 0,
            tma: 0,
            tac: 0,
            tima_enabled: false,
            tima_period: 1024,
        }
    }

    pub(super) fn get_div(&self) -> u8 {
        (self.counter >> 8) as u8
    }
    
    pub(super) fn set_div(&mut self, val: u8) {
        self.tima_enabled = (val & 0b100) != 0;
        self.tima_period = match val & 0b11 {
            0b00 => 1024,
            0b01 => 16,
            0b10 => 64,
            0b11 => 256,
            _ => unreachable!(),
        }
    }

    pub(super) fn get_tac(&self) -> u8 {
        self.tac
    }

    pub(super) fn set_tac(&mut self, val: u8) {
        self.tima_enabled = (val & 0b100) != 0;
        self.tima_period = match val & 0b11 {
            0b00 => 1024,
            0b01 => 16,
            0b10 => 64,
            0b11 => 256,
            _ => unreachable!(),
        }
    }

    pub fn tick(&mut self, t: usize) {
        self.counter = self.counter.wrapping_add(t as u16);

        // if self.counter % self.tima_period
    }
}