use std::alloc::{alloc, dealloc, Layout};

extern "C" {
    #[link_name = "render"]
    fn ffi_render(fb: *const u8, n: usize);
    #[link_name = "print"]
    fn ffi_print(bytes: *const u8, n: usize);
}


#[export_name = "alloc"]
pub extern "C" fn ffi_alloc(size: usize) -> *mut [u8] {
    let v = vec![0u8; size];
    let b = v.into_boxed_slice();
    Box::<[u8]>::into_raw(b)
}

#[export_name = "dealloc"]
pub extern "C" fn ffi_dealloc(ptr: *mut [u8]) {
    unsafe {
        let b = Box::<[u8]>::from_raw(ptr);
        drop(b);
    }
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

#[export_name = "step"]
pub extern "C" fn step() -> u8 {
    print("Stepping");
    0
}

#[export_name = "reset"]
pub extern "C" fn reset() {
    let empty = [];
    render(&empty);
}

