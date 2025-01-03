use crate::{
    basic::sky::BasicSky,
    nightsky::sky::NightSky,
    star_render::StarRender,
    system_options::StarSystemOptions,
    utils::{setup_logger, EmptySky},
};
use wasm_bindgen::prelude::*;
use web_sys::HtmlCanvasElement;

/// A star system that renders stars on a canvas
/// It binds to the input canvas and on the update_and_render call
/// it will handle rendering the stars
/// # Rendering
/// The rendering can be done either with the GPU or with the CPU
/// By default, the GPU rendering is used
/// If the use_advanced option is set to false, the CPU rendering is used
/// GPU rendering is more efficient and can render more stars
#[wasm_bindgen]
struct StarSystem {
    canvas: HtmlCanvasElement,
    options: StarSystemOptions,
    sky: Box<dyn StarRender>,
    using_advanced: bool,
}

#[wasm_bindgen]
impl StarSystem {
    /// Create a new star system with the given canvas and options
    /// # Description
    /// This does not setup the system, just creates the object
    /// Call `init` to setup the system
    /// # Arguments
    /// - `canvas` - The canvas to render the stars on
    /// - `options` - The options for the star system
    /// # Options
    /// - `log_level` - The log level to use (default: "warn")
    /// - `star_count` - The number of stars to render (default: 1000)
    /// - `clear_color` - The color to clear the screen with (default: "#000000")
    /// - `fade_speed` - The speed at which the stars fade (default: 0.001)
    /// - `use_advanced` - Use the advanced GPU rendering system (default: true)
    /// - `star_size` - Modifier for star size between 1.0-4.0 (default: 1.0)
    #[wasm_bindgen(constructor)]
    pub fn new(canvas: HtmlCanvasElement, options: js_sys::Object) -> Self {
        let options: StarSystemOptions =
            serde_wasm_bindgen::from_value(options.into()).unwrap_or(StarSystemOptions::default());
        setup_logger(&options.log_level);
        Self {
            canvas,
            options,
            sky: Box::new(EmptySky {}),
            using_advanced: false,
        }
    }

    /// Initialize the star system
    /// This needs to be called or update_and_render will not work
    /// Ensure this is awaited on before calling update_and_render
    pub async fn init(&mut self) {
        log::info!("Initializing star system");
        if self.options.use_advanced {
            match self.init_advanced().await {
                Ok(()) => {
                    log::info!("Advanced initialization succeeded");
                    self.using_advanced = true;
                    return;
                }
                Err(e) => log::warn!("Advanced initialization failed: {}", e),
            }
        }
        self.init_basic();
        log::info!("Basic initialization succeeded");
    }

    /// Update and render the stars
    pub fn update_and_render(&mut self, delta_time: f32) {
        self.sky.update_and_render(delta_time);
    }

    /// Resize the star system to new canvas dimensions
    pub fn resize(&mut self, canvas: HtmlCanvasElement) {
        self.sky.resize(canvas);
    }

    /// Check if the stars are rendered with the GPU
    pub fn is_advanced(&self) -> bool {
        self.using_advanced
    }

    /// Add new stars to the system
    pub fn add_stars(&mut self, count: u32) {
        self.sky.add_stars(count);
    }

    /// Remove stars from the system
    pub fn remove_stars(&mut self, count: u32) {
        self.sky.remove_stars(count);
    }

    async fn init_advanced(&mut self) -> Result<(), String> {
        let sky = NightSky::new(
            &self.canvas,
            self.options.clear_color.clone(),
            self.options.star_count,
            self.options.star_size,
        )
        .await?;
        self.sky = Box::new(sky);
        return Ok(());
    }

    fn init_basic(&mut self) {
        let canvas_id = self.canvas.id();
        let sky = BasicSky::new(
            &canvas_id,
            self.options.fade_speed,
            self.options.star_count,
            self.options.star_size.into(),
        );
        self.sky = Box::new(sky);
    }
}
