mod ffi_cpu;

use ffi_cpu::FFICPU;
use nes_core::Tick;

static mut CPU: FFICPU = ffi_cpu::construct();

pub extern "C" fn step() -> u8 {
    unsafe {
        CPU.tick()
    }
}