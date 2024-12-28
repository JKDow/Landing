use wasm_bindgen::prelude::*;
use web_sys::HtmlCanvasElement;

use crate::nightsky::{
    pipeline::{
        begin_render_pass, configure_surface, create_bind_group, create_compute_pipeline,
        create_instance, create_multisampled_frame, create_render_pipeline, create_star_buffer,
        create_surface, request_adapter, request_device_and_queue,
    },
    star::Star,
    utils::{hex_to_wgpu_color, setup_logger},
};

#[wasm_bindgen]
pub struct NightSky {
    _canvas: HtmlCanvasElement,
    _instance: wgpu::Instance,
    surface: wgpu::Surface<'static>,
    device: wgpu::Device,
    queue: wgpu::Queue,
    surface_config: wgpu::SurfaceConfiguration,
    clear_color: wgpu::Color,
    star_buffer: wgpu::Buffer,
    star_count: u32,
    render_pipeline: wgpu::RenderPipeline,
    multisampled_frame: wgpu::Texture,
    compute_pipeline: wgpu::ComputePipeline,
    bind_group: wgpu::BindGroup,
}

#[wasm_bindgen]
impl NightSky {
    #[wasm_bindgen(constructor)]
    /// Clear color is a hex string
    pub async fn new(canvas: HtmlCanvasElement, clear_color: String, star_count: u32) -> NightSky {
        setup_logger();
        let instance = create_instance();
        let surface = create_surface(&instance, &canvas);
        log::info!("Created instance and surface");
        let adapter = request_adapter(&instance, &surface).await;
        let (device, queue) = request_device_and_queue(&adapter).await;
        log::info!("Created adapter, device, and queue");
        let surface_config = configure_surface(&adapter, &surface, canvas.height(), canvas.width());
        surface.configure(&device, &surface_config);
        let clear_color = hex_to_wgpu_color(&clear_color).unwrap();
        log::info!("Created surface configuration and color: {:?}", clear_color);
        let stars = Star::generate(star_count as usize);
        let star_buffer = create_star_buffer(&device, &stars);
        let multisampled_frame = create_multisampled_frame(&device, &surface_config);

        let render_pipeline = create_render_pipeline(&device, &surface_config);
        let compute_pipeline = create_compute_pipeline(&device);
        let bind_group = create_bind_group(&device, &compute_pipeline, &star_buffer);

        NightSky {
            _canvas: canvas,
            _instance: instance,
            surface,
            device,
            queue,
            surface_config,
            clear_color,
            star_count,
            star_buffer,
            render_pipeline,
            multisampled_frame,
            compute_pipeline,
            bind_group,
        }
    }

    #[wasm_bindgen]
    pub fn update_and_render(&self) {
        self.update();
        self.render();
    }

    pub fn resize(&mut self, width: u32, height: u32) {
        self.surface_config.width = width;
        self.surface_config.height = height;
        self.surface.configure(&self.device, &self.surface_config);
        //log::info!("Resized surface to {}x{}", width, height);
    }

    fn update(&self) {
        let mut encoder = self
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                label: Some("Compute Encoder"),
            });

        // Dispatch compute shader
        {
            let mut compute_pass = encoder.begin_compute_pass(&wgpu::ComputePassDescriptor {
                label: Some("Compute Pass"),
                timestamp_writes: None,
            });
            compute_pass.set_pipeline(&self.compute_pipeline);
            compute_pass.set_bind_group(0, &self.bind_group, &[]);
            compute_pass.dispatch_workgroups((self.star_count + 63) / 64, 1, 1);
        }

        // Submit the compute commands
        self.queue.submit(std::iter::once(encoder.finish()));
    }

    fn render(&self) {
        let mut encoder = self.command_encoder();
        let frame = self.surface.get_current_texture().unwrap();
        let view = frame
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());

        // Create the multisampled texture view
        let multisampled_view = self
            .multisampled_frame
            .create_view(&wgpu::TextureViewDescriptor::default());
        // Create the render pass
        {
            let mut render_pass =
                begin_render_pass(&mut encoder, &view, &multisampled_view, self.clear_color);
            render_pass.set_pipeline(&self.render_pipeline);
            render_pass.set_vertex_buffer(0, self.star_buffer.slice(..)); // Buffer for star instances
            render_pass.draw(0..4, 0..self.star_count as u32); // Draw 4 vertices per instance
        }
        self.submit(encoder);
        frame.present();
    }

    fn command_encoder(&self) -> wgpu::CommandEncoder {
        self.device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                label: Some("Render Encoder"),
            })
    }

    fn submit(&self, encoder: wgpu::CommandEncoder) {
        self.queue.submit(std::iter::once(encoder.finish()));
    }
}
