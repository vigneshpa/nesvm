use core::slice;

use nes_core::ppu2c02::Pixel;
use nes_core::{ppu2c02::VideoBackend, Emulator, Tick};
use wasm_bindgen::prelude::*;

#[wasm_bindgen(start)]
pub fn init() {
    std::panic::set_hook(Box::new(console_error_panic_hook::hook));
    console_log::init_with_level(log::Level::Debug).unwrap();
}

struct FFIVideo(js_sys::Function);
impl VideoBackend for FFIVideo {
    fn render(&mut self, fb: &[Pixel]) -> () {

        let view = unsafe {
            let slice = slice::from_raw_parts(fb.as_ptr() as *const u8, fb.len() * size_of::<Pixel>()) ;
            js_sys::Uint8Array::view(slice)
        };
        let this = JsValue::null();
        let arg1 = JsValue::from(view);
        // ffi_render(fb.as_ptr(), fb.len() * size_of::<Pixel>());
        self.0.call1(&this, &arg1).unwrap();
    }
}

#[wasm_bindgen]
pub struct WasmNES(Emulator);

#[wasm_bindgen]
impl WasmNES {
    #[wasm_bindgen(constructor)]
    pub fn new(nes_file: &[u8], render_fn: js_sys::Function) -> Self {
        let video = FFIVideo(render_fn);
        let emu = Emulator::new(nes_file, video);
        log::info!("Loaded new Game ROM");
        Self(emu)
    }

    pub fn step(&mut self) -> u8 {
        log::info!("Stepping");
        self.0.tick()
    }

    pub fn reset(&mut self) {
        log::info!("Resetting");
        self.0.reset()
    }

    pub fn drop(self) -> () {}
}
