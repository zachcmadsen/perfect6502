#![no_std]

use core::{
    ffi::c_void,
    sync::atomic::{AtomicBool, Ordering},
};

static EXISTS: AtomicBool = AtomicBool::new(false);

pub struct State(*mut c_void);

impl Drop for State {
    fn drop(&mut self) {
        unsafe { perfect6502_sys::destroyChip(self.0) };
        EXISTS.store(false, Ordering::Release);
    }
}

impl State {
    pub fn new() -> Option<State> {
        if EXISTS.swap(true, Ordering::Acquire) {
            None
        } else {
            let state = unsafe { perfect6502_sys::initAndResetChip() };
            Some(State(state))
        }
    }

    pub fn step(&mut self) {
        unsafe { perfect6502_sys::step(self.0) };
    }

    pub fn pc(&mut self) -> u8 {
        unsafe { perfect6502_sys::readPC(self.0) as u8 }
    }

    pub fn a(&mut self) -> u8 {
        unsafe { perfect6502_sys::readA(self.0) as u8 }
    }

    pub fn x(&mut self) -> u8 {
        unsafe { perfect6502_sys::readX(self.0) as u8 }
    }

    pub fn y(&mut self) -> u8 {
        unsafe { perfect6502_sys::readY(self.0) as u8 }
    }

    pub fn s(&mut self) -> u8 {
        unsafe { perfect6502_sys::readSP(self.0) as u8 }
    }

    pub fn p(&mut self) -> u8 {
        unsafe { perfect6502_sys::readP(self.0) as u8 }
    }

    pub fn read_addr_bus(&mut self) -> u16 {
        unsafe { perfect6502_sys::readAddressBus(self.0) }
    }

    pub fn read_data_bus(&mut self) -> u8 {
        unsafe { perfect6502_sys::readDataBus(self.0) }
    }

    pub fn read(&mut self, addr: u8) -> u8 {
        unsafe { perfect6502_sys::memory[addr as usize] }
    }

    pub fn write(&mut self, addr: u8, data: u8) {
        unsafe { perfect6502_sys::memory[addr as usize] = data };
    }
}
