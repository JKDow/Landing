use super::star::BasicStar;
use crate::star_render::StarRender;
use wasm_bindgen::JsCast;
use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement};

pub struct BasicSky {
    stars: Vec<BasicStar>, // Particles grouped by layers
    canvas: HtmlCanvasElement,
    ctx: CanvasRenderingContext2d,
    fade_speed: f64,
    star_size: f64,
}

impl BasicSky {
    pub fn new(canvas_id: &str, fade_speed: f64, star_count: u32, star_size: f32) -> Self {
        let document = web_sys::window().unwrap().document().unwrap();
        let canvas = document.get_element_by_id(canvas_id).unwrap();
        let canvas: HtmlCanvasElement = canvas.dyn_into::<HtmlCanvasElement>().unwrap();
        let ctx = canvas
            .get_context("2d")
            .unwrap()
            .unwrap()
            .dyn_into::<CanvasRenderingContext2d>()
            .unwrap();

        let star_count = star_count.clamp(0, 2000);
        let stars = (0..star_count)
            .map(|_| {
                BasicStar::new(
                    canvas.width() as f64,
                    canvas.height() as f64,
                    fade_speed,
                    js_sys::Math::random(),
                    "#FFFFFF".to_string(), // Customize colors per layer if needed
                    star_size.into(),
                )
            })
            .collect();

        Self {
            stars,
            canvas,
            ctx,
            fade_speed,
            star_size: star_size as f64,
        }
    }

    pub fn update_and_render(&mut self) {
        self.ctx.clear_rect(
            0.0,
            0.0,
            self.canvas.width() as f64,
            self.canvas.height() as f64,
        );

        for star in self.stars.iter_mut() {
            if star.active {
                star.update();
                star.draw(&self.ctx);
            } else {
                *star = BasicStar::new(
                    self.canvas.width() as f64,
                    self.canvas.height() as f64,
                    self.fade_speed,
                    0.0,
                    star.color.clone(),
                    self.star_size,
                );
            }
        }
    }
}

impl StarRender for BasicSky {
    fn update_and_render(&mut self, _delta_time: f32) {
        self.update_and_render();
    }

    fn resize(&mut self, canvas: HtmlCanvasElement) {
        self.canvas = canvas;
    }

    fn add_stars(&mut self, count: u32) {
        self.stars.extend((0..count).map(|_| {
            BasicStar::new(
                self.canvas.width() as f64,
                self.canvas.height() as f64,
                self.fade_speed,
                0.0,
                "#FFFFFF".to_string(),
                self.star_size,
            )
        }));
    }

    fn remove_stars(&mut self, count: u32) {
        self.stars.truncate(self.stars.len().saturating_sub(count as usize));
    }
}
