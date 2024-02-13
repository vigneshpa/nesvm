use std::slice;

#[repr(C)]
pub struct WidePointer {
    pub buf: *mut u8,
    pub len: usize,
}

/// Allocates new memory and return a thin pointer to the fat pointer of the allocated memory
/// Indirect thim pointer is returned because a fat pointer cannot be returned with "C" abi and modified in JS
/// As JS do not have access to unexported globals. Calling a function that reutrns a structs requires acces to the global 0, which is not exported
#[export_name = "alloc"]
pub extern "C" fn ffi_alloc(size: usize) -> *mut WidePointer {
    let v = vec![0u8; size];
    let buf = v.into_boxed_slice();
    let len = buf.len();
    let buf = Box::into_raw(buf);
    let buf = buf as *mut u8;
    let ptr = WidePointer { buf, len };
    let b = Box::new(ptr);
    Box::into_raw(b)
}

#[export_name = "free"]
pub extern "C" fn ffi_dealloc(ptr: *mut WidePointer) {
    unsafe {
        let ptr = Box::from_raw(ptr);
        let b = Box::from_raw(slice::from_raw_parts_mut(ptr.buf as *mut u8, ptr.len));
        drop(b);
        drop(ptr);
    }
}