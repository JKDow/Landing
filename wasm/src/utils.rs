use web_sys::HtmlCanvasElement;

use crate::star_render::StarRender;

pub fn setup_logger(level: &str) {
    let log_level = match level {
        "debug" => log::Level::Debug,
        "info" => log::Level::Info,
        "warn" => log::Level::Warn,
        "error" => log::Level::Error,
        _ => log::Level::Warn,
    };
    console_log::init_with_level(log_level).expect("Couldn't initialize logger");
    std::panic::set_hook(Box::new(console_error_panic_hook::hook));
    log::info!("Started wasm logger");
}

pub fn random_range(min: f32, max: f32) -> f32 {
    js_sys::Math::random() as f32 * (max - min) + min
}

pub struct EmptySky {}
impl StarRender for EmptySky {
    fn update_and_render(&mut self, _delta_time: f32) {}
    fn resize(&mut self, _canvas: HtmlCanvasElement) {}
    fn add_stars(&mut self, _count: u32) {}
    fn remove_stars(&mut self, _count: u32) {}
}
