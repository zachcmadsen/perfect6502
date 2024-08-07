#![no_std]

use core::{
    ffi::c_void,
    ptr::{self, NonNull},
    sync::atomic::{AtomicBool, Ordering},
};

static EXISTS: AtomicBool = AtomicBool::new(false);

pub struct State {
    inner: NonNull<c_void>,
}

pub enum Cycle {
    Read,
    Write,
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

    pub fn pc(&mut self) -> u16 {
        unsafe { perfect6502_sys::readPC(self.inner.as_ptr()) }
    }

    pub fn a(&mut self) -> u8 {
        unsafe { perfect6502_sys::readA(self.inner.as_ptr()) }
    }

    pub fn x(&mut self) -> u8 {
        unsafe { perfect6502_sys::readX(self.inner.as_ptr()) }
    }

    pub fn y(&mut self) -> u8 {
        unsafe { perfect6502_sys::readY(self.inner.as_ptr()) }
    }

    pub fn sp(&mut self) -> u8 {
        unsafe { perfect6502_sys::readSP(self.inner.as_ptr()) }
    }

    pub fn p(&mut self) -> u8 {
        unsafe { perfect6502_sys::readP(self.inner.as_ptr()) }
    }

    pub fn rw(&mut self) -> Cycle {
        let rw = unsafe { perfect6502_sys::readRW(self.inner.as_ptr()) };
        match rw != 0 {
            true => Cycle::Read,
            false => Cycle::Write,
        }
    }

    pub fn address_bus(&mut self) -> u16 {
        unsafe { perfect6502_sys::readAddressBus(self.inner.as_ptr()) }
    }

    pub fn data_bus(&mut self) -> u8 {
        unsafe { perfect6502_sys::readDataBus(self.inner.as_ptr()) }
    }

    pub fn set_data_bus(&mut self, data: u8) {
        unsafe { perfect6502_sys::writeDataBus(self.inner.as_ptr(), data) };
    }

    pub fn ir(&mut self) -> u8 {
        unsafe { perfect6502_sys::readIR(self.inner.as_ptr()) }
    }

    pub fn memory(&self) -> &[u8] {
        unsafe { &*ptr::addr_of!(perfect6502_sys::memory) }
    }

    pub fn memory_mut(&mut self) -> &mut [u8] {
        unsafe { &mut *ptr::addr_of_mut!(perfect6502_sys::memory) }
    }

    pub fn cycles(&mut self) -> u32 {
        unsafe { perfect6502_sys::cycle }
    }
}

impl Drop for State {
    fn drop(&mut self) {
        unsafe { perfect6502_sys::destroyChip(self.inner.as_ptr()) };
        EXISTS.store(false, Ordering::Release);
    }
}
