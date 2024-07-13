#![no_std]

use core::{
    ffi::c_void,
    ptr::NonNull,
    sync::atomic::{AtomicBool, Ordering},
};

static EXISTS: AtomicBool = AtomicBool::new(false);

pub struct State {
    inner: NonNull<c_void>,
}

impl State {
    pub fn new() -> Option<State> {
        if EXISTS.swap(true, Ordering::Acquire) {
            None
        } else {
            let ptr = unsafe { perfect6502_sys::initAndResetChip() };
            Some(State {
                inner: NonNull::new(ptr).expect("ptr should not be null"),
            })
        }
    }

    pub fn step(&mut self) {
        unsafe { perfect6502_sys::step(self.inner.as_ptr()) };
    }

    pub fn pc(&mut self) -> u8 {
        unsafe { perfect6502_sys::readPC(self.inner.as_ptr()) as u8 }
    }

    pub fn a(&mut self) -> u8 {
        unsafe { perfect6502_sys::readA(self.inner.as_ptr()) as u8 }
    }

    pub fn x(&mut self) -> u8 {
        unsafe { perfect6502_sys::readX(self.inner.as_ptr()) as u8 }
    }

    pub fn y(&mut self) -> u8 {
        unsafe { perfect6502_sys::readY(self.inner.as_ptr()) as u8 }
    }

    pub fn s(&mut self) -> u8 {
        unsafe { perfect6502_sys::readSP(self.inner.as_ptr()) as u8 }
    }

    pub fn p(&mut self) -> u8 {
        unsafe { perfect6502_sys::readP(self.inner.as_ptr()) as u8 }
    }

    pub fn read_addr_bus(&mut self) -> u16 {
        unsafe { perfect6502_sys::readAddressBus(self.inner.as_ptr()) }
    }

    pub fn read_data_bus(&mut self) -> u8 {
        unsafe { perfect6502_sys::readDataBus(self.inner.as_ptr()) }
    }

    pub fn read(&mut self, addr: u16) -> u8 {
        unsafe { perfect6502_sys::memory[addr as usize] }
    }

    pub fn write(&mut self, addr: u16, data: u8) {
        unsafe { perfect6502_sys::memory[addr as usize] = data };
    }
}

impl Drop for State {
    fn drop(&mut self) {
        unsafe { perfect6502_sys::destroyChip(self.inner.as_ptr()) };
        EXISTS.store(false, Ordering::Release);
    }
}
