use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement};

#[wasm_bindgen]
pub struct Particle {
    x: f64,
    y: f64,
    size: f64,
    opacity: f64,
    fade_speed: f64,
    active: bool,
    color: String,
    velocity_x: f64,
    velocity_y: f64,
}

#[wasm_bindgen]
impl Particle {
    pub fn new(width: f64, height: f64, fade_speed: f64, opacity: f64, color: String) -> Particle {
        Particle {
            x: js_sys::Math::random() * width,
            y: js_sys::Math::random() * height,
            size: 2.0 + 2.0 * js_sys::Math::random(),
            opacity,
            fade_speed: (fade_speed / 2.0) + (fade_speed * js_sys::Math::random()),
            active: true,
            color,
            velocity_x: (js_sys::Math::random() - 0.5) * 0.1,
            velocity_y: (js_sys::Math::random() - 0.5) * 0.1,
        }
    }

    pub fn update(&mut self) {
        if !self.active {
            return;
        }
        self.x += self.velocity_x;
        self.y += self.velocity_y;
        self.opacity += self.fade_speed;
        if self.opacity > 1.0 {
            self.fade_speed = -self.fade_speed;
        } else if self.opacity < 0.0 {
            self.active = false;
        }
    }

    pub fn draw(&self, ctx: &CanvasRenderingContext2d) {
        if !self.active {
            return;
        }
        ctx.set_fill_style_str(&self.color);
        ctx.set_global_alpha(self.opacity);
        ctx.begin_path();
        ctx.arc(self.x, self.y, self.size, 0.0, std::f64::consts::PI * 2.0)
            .unwrap();
        ctx.fill();
    }
}

#[wasm_bindgen]
pub struct ParticleSystem {
    layers: Vec<Vec<Particle>>, // Particles grouped by layers
    canvas: HtmlCanvasElement,
    ctx: CanvasRenderingContext2d,
    fade_speed: f64,
}

#[wasm_bindgen]
impl ParticleSystem {
    #[wasm_bindgen(constructor)]
    pub fn new(
        canvas_id: &str,
        fade_speed: f64,
        layer_count: u32,
        particles_per_layer: u32,
    ) -> ParticleSystem {
        let document = web_sys::window().unwrap().document().unwrap();
        let canvas = document.get_element_by_id(canvas_id).unwrap();
        let canvas: HtmlCanvasElement = canvas.dyn_into::<HtmlCanvasElement>().unwrap();
        let ctx = canvas
            .get_context("2d")
            .unwrap()
            .unwrap()
            .dyn_into::<CanvasRenderingContext2d>()
            .unwrap();

        let layers = (0..layer_count)
            .map(|layer_index| {
                (0..particles_per_layer)
                    .map(|_| {
                        Particle::new(
                            canvas.width() as f64,
                            canvas.height() as f64,
                            fade_speed * (1.0 + layer_index as f64 * 0.2),
                            js_sys::Math::random(),
                            "#FFFFFF".to_string(), // Customize colors per layer if needed
                        )
                    })
                    .collect()
            })
            .collect();

        ParticleSystem {
            layers,
            canvas,
            ctx,
            fade_speed,
        }
    }

    pub fn update_and_render(&mut self) {
        self.ctx.clear_rect(
            0.0,
            0.0,
            self.canvas.width() as f64,
            self.canvas.height() as f64,
        );

        for layer in &mut self.layers {
            for particle in layer.iter_mut() {
                if particle.active {
                    particle.update();
                    particle.draw(&self.ctx);
                } else {
                    *particle = Particle::new(
                        self.canvas.width() as f64,
                        self.canvas.height() as f64,
                        self.fade_speed,
                        0.0,
                        particle.color.clone(),
                    );
                }
            }
        }
    }
}
