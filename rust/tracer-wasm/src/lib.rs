use std::sync::{Mutex, OnceLock};
use tracer::Renderer;
struct State {
    renderer: Option<Renderer>,
    framebuffer: Vec<u8>,
}
static RENDERER: OnceLock<Mutex<State>> = OnceLock::new();
fn state() -> &'static Mutex<State> {
    RENDERER.get_or_init(|| {
        Mutex::new(State {
            renderer: None,
            framebuffer: Vec::new(),
        })
    })
}
#[no_mangle]
pub extern "C" fn init(width: u32, height: u32, scene: u32) {
    let mut s = state().lock().unwrap();
    let renderer = Renderer::new(width.max(1), height.max(1), scene);
    s.framebuffer = vec![0; (renderer.width * renderer.height * 4) as usize];
    renderer.write_rgba(&mut s.framebuffer);
    s.renderer = Some(renderer);
}
#[no_mangle]
pub extern "C" fn reset() {
    let mut s = state().lock().unwrap();
    let mut framebuffer = std::mem::take(&mut s.framebuffer);
    if let Some(r) = s.renderer.as_mut() {
        r.reset();
        r.write_rgba(&mut framebuffer);
    }
    s.framebuffer = framebuffer;
}
#[no_mangle]
pub extern "C" fn set_orbit(yaw: f32, pitch: f32) {
    let mut s = state().lock().unwrap();
    let mut framebuffer = std::mem::take(&mut s.framebuffer);
    if let Some(r) = s.renderer.as_mut() {
        r.set_orbit(yaw, pitch);
        r.write_rgba(&mut framebuffer);
    }
    s.framebuffer = framebuffer;
}
#[no_mangle]
pub extern "C" fn render_pass() -> u32 {
    let mut s = state().lock().unwrap();
    let mut framebuffer = std::mem::take(&mut s.framebuffer);
    if let Some(r) = s.renderer.as_mut() {
        r.render_pass(0xfeed_beef);
        let samples = r.samples;
        r.write_rgba(&mut framebuffer);
        s.framebuffer = framebuffer;
        samples
    } else {
        s.framebuffer = framebuffer;
        0
    }
}
#[no_mangle]
pub extern "C" fn sample_count() -> u32 {
    state()
        .lock()
        .unwrap()
        .renderer
        .as_ref()
        .map_or(0, |r| r.samples)
}
#[no_mangle]
pub extern "C" fn ray_count() -> u32 {
    state()
        .lock()
        .unwrap()
        .renderer
        .as_ref()
        .map_or(0, |r| r.last_pass_rays.min(u32::MAX as u64) as u32)
}
#[no_mangle]
pub extern "C" fn framebuffer_ptr() -> *const u8 {
    state().lock().unwrap().framebuffer.as_ptr()
}
