use web_sys::CanvasRenderingContext2d;

use crate::utils::random_range;

pub struct BasicStar {
    pub x: f64,
    pub y: f64,
    pub size: f64,
    pub opacity: f64,
    pub fade_speed: f64,
    pub active: bool,
    pub color: String,
    pub velocity_x: f64,
    pub velocity_y: f64,
}

impl BasicStar {
    pub fn new(width: f64, height: f64, fade_speed: f64, opacity: f64, color: String, size: f64) -> Self {
        let size = size.clamp(0.1, 4.0);
        Self {
            x: js_sys::Math::random() * width,
            y: js_sys::Math::random() * height,
            size: random_range(0.5, 2.0) as f64 * size,
            opacity,
            fade_speed: (fade_speed / 2.0) + (fade_speed * js_sys::Math::random()),
            active: true,
            color,
            velocity_x: (js_sys::Math::random() - 0.5) * 0.08,
            velocity_y: (js_sys::Math::random() - 0.5) * 0.08,
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

