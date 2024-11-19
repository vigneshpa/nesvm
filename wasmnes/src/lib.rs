use core::slice;

use nes_core::ppu::Pixel;
use nes_core::{ppu::VideoBackend, Emulator, Tick};

use js_sys::Uint8ClampedArray;
use wasm_bindgen::prelude::*;

// Type definition for the render funtion
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = js_sys::Function, typescript_type = "(fb: Uint8ClampedArray) => void")]
    pub type FBRenderFunction;
}

#[wasm_bindgen(raw_module = "../src/main.ts")]
extern "C" {
    pub fn panic_handler(info: &str);
}

// WASM initilization
#[wasm_bindgen(start)]
pub fn init() {
    std::panic::set_hook(Box::new(|info| {
        let info = info.to_string();
        // let stackTace = info.stack
        panic_handler(&info);
    }));
    console_log::init_with_level(log::Level::Trace).unwrap();
}

// Video backend implementation
struct WasmVideo(FBRenderFunction);
impl VideoBackend for WasmVideo {
    fn render(&mut self, fb: &[Pixel]) -> () {
        let view = unsafe {
            let slice =
                slice::from_raw_parts(fb.as_ptr() as *const u8, fb.len() * size_of::<Pixel>());
            Uint8ClampedArray::view(slice)
        };
        let this = JsValue::null();
        let arg1 = JsValue::from(view);
        // ffi_render(fb.as_ptr(), fb.len() * size_of::<Pixel>());
        self.0.call1(&this, &arg1).unwrap();
    }
}

// Exposing WasmNES

#[wasm_bindgen]
pub struct WasmNES(Emulator);

#[wasm_bindgen]
impl WasmNES {
    #[wasm_bindgen(constructor)]
    pub fn new(nes_file: &[u8], render_fn: FBRenderFunction) -> Self {
        let video = WasmVideo(render_fn);
        let emu = Emulator::new(nes_file, video);
        Self(emu)
    }

    pub fn step(&mut self) -> u8 {
        self.0.tick()
    }

    pub fn reset(&mut self) {
        self.0.reset()
    }
}
