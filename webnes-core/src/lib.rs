use std::panic;
use allocator::WidePointer;
use nes_core::{Core, Tick};

mod allocator;

static mut CORE_PTR: usize = 0;

extern "C" {
    #[link_name = "render"]
    fn ffi_render(fb: *const u8, n: usize);
    #[link_name = "print"]
    fn ffi_print(bytes: *const u8, n: usize);
    #[link_name = "error"]
    fn ffi_error(bytes: *const u8, n: usize);
}

fn render(fb: &[u8]) {
    unsafe {
        ffi_render(fb.as_ptr(), fb.len());
    }
}

fn print(text:&str) {
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
}


#[export_name = "load"]
pub extern "C" fn load(nes_file: *mut WidePointer) {
    let nes_file = unsafe {
        let nes_file = &*nes_file;
        std::slice::from_raw_parts(nes_file.buf, nes_file.len)
    };

    let core = Core::new(nes_file);
    let core = Box::new(core);
    unsafe {
        if CORE_PTR != 0 {
            panic!("ROM already loaded!");
        }
        CORE_PTR = Box::into_raw(core) as usize;
    }
}

#[export_name = "step"]
pub extern "C" fn step() -> u8 {
    unsafe {
        if CORE_PTR == 0 {
            panic!("Cannot step before loading a ROM!");
        }
        let core = CORE_PTR as *mut Core;
        let core = &mut *core;
        print("Stepping");
        core.tick()
    }
}

#[export_name = "reset"]
pub extern "C" fn reset() {
    let empty = [];
    render(&empty);
}