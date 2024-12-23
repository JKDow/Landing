use std::sync::Arc;

use winit::{
    dpi::PhysicalSize,
    event::*,
    event_loop::EventLoop,
    keyboard::{KeyCode, PhysicalKey},
    platform::web::{EventLoopExtWebSys, WindowExtWebSys},
    window::WindowBuilder,
};

use crate::gpu::state::State;
use wasm_bindgen::prelude::*;

#[cfg_attr(target_arch = "wasm32", wasm_bindgen(start))]
pub async fn run() {
    // check if there is a logger
    console_log::init_with_level(log::Level::Info).expect("Couldn't initialize logger");
    std::panic::set_hook(Box::new(console_error_panic_hook::hook));
    log::info!("Started wasm logger");
    let event_loop = EventLoop::new().expect("Failed to create event loop");
    let window = Arc::new(
        WindowBuilder::new()
            .with_inner_size(PhysicalSize::new(450, 450))
            .build(&event_loop)
            .expect("Failed to create window"),
    );

    web_sys::window()
        .and_then(|win| win.document())
        .and_then(|doc| {
            let dst = doc.get_element_by_id("wasm-window")?;
            let canvas = web_sys::Element::from(window.canvas()?);
            dst.append_child(&canvas).ok().unwrap();
            Some(())
        })
        .expect("Couldn't append canvas to document body.");

    let mut state = State::new(window).await;
    let mut surface_configured = false;

    event_loop.spawn(move |event, control_flow| match event {
        Event::Resumed => {
            log::debug!("Resumed");
        }
        Event::WindowEvent {
            ref event,
            window_id,
        } if window_id == state.window().id() => {
            if !state.input(event) {
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
                        state.resize(*physical_size);
                    }
                    WindowEvent::RedrawRequested => {
                        // This tells winit that we want another frame after this one
                        state.window().request_redraw();
                        if !surface_configured {
                            return;
                        }
                        state.update();
                        match state.render() {
                            Ok(_) => {}
                            // Reconfigure the surface if it's lost or outdated
                            Err(wgpu::SurfaceError::Lost | wgpu::SurfaceError::Outdated) => {
                                state.resize(state.size())
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
