pub mod gpu;
pub mod night_sky;
pub mod run;
pub mod star;

use std::sync::Arc;

use night_sky::NightSky;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement};
use winit::dpi::PhysicalSize;
use winit::event::ElementState;
use winit::event::Event;
use winit::event::KeyEvent;
use winit::event::WindowEvent;
use winit::event_loop::EventLoop;
use winit::keyboard::KeyCode;
use winit::keyboard::PhysicalKey;
use winit::platform::web::EventLoopExtWebSys;
use winit::platform::web::WindowBuilderExtWebSys;
use winit::window::WindowBuilder;

#[wasm_bindgen]
pub async fn run_stars(canvas_id: String) {
    console_log::init_with_level(log::Level::Info).expect("Couldn't initialize logger");
    std::panic::set_hook(Box::new(console_error_panic_hook::hook));
    log::info!("Started wasm logger");
    let window = web_sys::window().ok_or("No window available").unwrap();
    let document = window.document().ok_or("No document available").unwrap();
    let canvas = document
        .get_element_by_id(&canvas_id)
        .ok_or("Canvas not found")
        .unwrap();
    let canvas: web_sys::HtmlCanvasElement = canvas
        .dyn_into()
        .map_err(|_| "Failed to cast to HtmlCanvasElement")
        .unwrap();

    let event_loop = EventLoop::new().expect("Failed to create event loop");
    // Use `winit` to wrap the canvas in a `Window`:
    let window = Arc::new(
        WindowBuilder::new()
            .with_inner_size(PhysicalSize::new(800, 600)) // Default size
            .with_canvas(Some(canvas)) // Attach to the existing canvas
            .build(&event_loop)
            .map_err(|e| JsValue::from_str(&format!("Failed to create window: {:?}", e)))
            .unwrap(),
    );

    log::info!("Setup window and linked canvas");

    // Initialize NightSky
    let mut sky = NightSky::new(window).await;
    let mut surface_configured = false;
    log::info!("Running event loop");
    event_loop.spawn(move |event, control_flow| match event {
        Event::Resumed => {
            log::debug!("Resumed");
        }
        Event::WindowEvent {
            ref event,
            window_id,
        } if window_id == sky.window().id() => {
            if !sky.input(event) {
                match event {
                    WindowEvent::CloseRequested
                    | WindowEvent::KeyboardInput {
                        event:
                            KeyEvent {
                                state: ElementState::Pressed,
                                physical_key: PhysicalKey::Code(KeyCode::Escape),
                                ..
                            },
                        ..
                    } => control_flow.exit(),
                    WindowEvent::Resized(physical_size) => {
                        log::info!("Resized {:?}", physical_size);
                        surface_configured = true;
                        sky.resize(*physical_size);
                    }
                    WindowEvent::RedrawRequested => {
                        // This tells winit that we want another frame after this one
                        sky.window().request_redraw();
                        if !surface_configured {
                            return;
                        }
                        sky.update();
                        match sky.render() {
                            Ok(_) => {}
                            // Reconfigure the surface if it's lost or outdated
                            Err(wgpu::SurfaceError::Lost | wgpu::SurfaceError::Outdated) => {
                                sky.resize(sky.size())
                            }
                            // The system is out of memory, we should probably quit
                            Err(wgpu::SurfaceError::OutOfMemory) => {
                                log::error!("OutOfMemory");
                                control_flow.exit();
                            }
                            // This happens when the a frame takes too long to present
                            Err(wgpu::SurfaceError::Timeout) => {
                                log::warn!("Surface timeout")
                            }
                        }
                    }
                    _ => {}
                }
            }
        }
        _ => {}
    });
}

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
