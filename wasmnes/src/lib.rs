use std::mem::size_of;
use std::panic;
use allocator::WidePointer;
use nes_core::ppu2c02::Pixel;
use nes_core::{ppu2c02::VideoBackend, Emulator, Tick};
use nes_core::log;

mod allocator;

static mut EMU_PTR: usize = 0;

extern "C" {
    #[link_name = "render"]
    fn ffi_render(fb: *const Pixel, n: usize);
    #[link_name = "print"]
    fn ffi_print(bytes: *const u8, n: usize);
    #[link_name = "error"]
    fn ffi_error(bytes: *const u8, n: usize);
}

fn render(fb: &[Pixel]) {
    unsafe {
        ffi_render(fb.as_ptr(), fb.len() * size_of::<Pixel>());
    }
}

pub fn print_wasm(text:&str) {
    unsafe {
        ffi_print(text.as_ptr(), text.len());
    }
}

fn error(text:&str) {
    unsafe {
        ffi_error(text.as_ptr(), text.len());
    }
}

#[export_name = "init"]
pub fn init() {
    panic::set_hook(Box::new(|info| {
        let mesg = info.to_string();
        error(&mesg);
    }));
    nes_core::logger::register_logger(print_wasm);
}

struct FFIVideo;
impl VideoBackend for FFIVideo {
    fn render(&mut self, fb: &[Pixel]) -> () {
        render(fb)
    }
}


#[export_name = "load"]
pub extern "C" fn load(nes_file: *mut WidePointer) {
    let nes_file = unsafe {
        let nes_file = &*nes_file;
        std::slice::from_raw_parts(nes_file.buf, nes_file.len)
    };

    let mut emu = Emulator::new(nes_file, FFIVideo);
    emu.reset();
    let emu = Box::new(emu);
    unsafe {
        if EMU_PTR != 0 {
            panic!("ROM already loaded!");
        }
        EMU_PTR = Box::into_raw(emu) as usize;
    }
    log!("Loaded new Game ROM");
}

#[export_name = "step"]
pub extern "C" fn step() -> u8 {
    unsafe {
        if EMU_PTR == 0 {
            panic!("Cannot step before loading a ROM!");
        }
        let emu = EMU_PTR as *mut Emulator;
        let emu = &mut *emu;
        log!("Stepping");
        emu.tick()
    }
}

#[export_name = "reset"]
pub extern "C" fn reset() {
    unsafe {
        if EMU_PTR == 0 {
            panic!("Cannot reset before loading a ROM!");
        }
        let emu = EMU_PTR as *mut Emulator;
        let emu = &mut *emu;
        log!("Resetting");
        emu.reset();
    }
}