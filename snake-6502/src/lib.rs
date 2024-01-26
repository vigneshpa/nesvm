mod game;
use game::{Game, GameBus};

extern "C" {
    /// Imported function to read random number generator
    #[link_name = "rng"]
    fn ffi_rng() -> u8;
    /// Imported function to read button input
    #[link_name = "btn"]
    fn ffi_btn() -> u8;
    /// Imported function to render graphics
    #[link_name = "render"]
    fn ffi_render(buffer: *const u8, length: usize);
    /// Imported function to log out
    #[link_name = "flog"]
    fn ffi_flog(data:f32);
}

#[export_name = "create"]
pub extern "C" fn ffi_create() -> *mut Game {
    let bus = GameBus::new(rng, btn, render);
    let game = Game::new(bus);
    Box::into_raw(game)
}

#[export_name = "destroy"]
pub extern "C" fn ffi_destroy(p: *mut Game) {
    unsafe {
        let game = Box::from_raw(p);
        drop(game);
    }
}

#[export_name = "step"]
pub extern "C" fn ffi_step(p: *mut Game) {
    unsafe {
        (*p).step();
    }
}

fn rng() -> u8 {
    unsafe { ffi_rng() }
}

fn btn() -> u8 {
    unsafe { ffi_btn() }
}

#[allow(dead_code)]
fn flog(data: f32) {
    unsafe { ffi_flog(data) }
}

fn render(buffer: &[u8]) {
    unsafe {
        ffi_render(buffer.as_ptr(), buffer.len());
    }
}