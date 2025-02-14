use web_sys::HtmlCanvasElement;

use crate::{
    nightsky::{
        pipeline::*, screen::create_screen_size_buffer, star::Star, utils::hex_to_wgpu_color,
    },
    star_render::StarRender,
};

pub struct NightSky {
    star_size: f32,
    _instance: wgpu::Instance,
    surface: wgpu::Surface<'static>,
    device: wgpu::Device,
    queue: wgpu::Queue,
    surface_config: wgpu::SurfaceConfiguration,
    clear_color: wgpu::Color,
    stars: Vec<Star>,
    star_buffer: wgpu::Buffer,
    circle_vertex_buffer: wgpu::Buffer,
    circle_index_buffer: wgpu::Buffer,
    index_count: u32,
    render_pipeline: wgpu::RenderPipeline,
    multisampled_frame: wgpu::Texture,
    bind_group: wgpu::BindGroup,
}

impl NightSky {
    /// Clear color is a hex string
    pub async fn new(
        canvas: &HtmlCanvasElement,
        clear_color: String,
        star_count: u32,
        star_size: f32,
    ) -> Result<NightSky, String> {
        let instance = create_instance();
        let surface = create_surface(&instance, canvas).map_err(|e| e.to_string())?;
        log::info!("Created instance and surface");
        let adapter = request_adapter(&instance, &surface)
            .await
            .ok_or("No adapter found")?;
        let limits = adapter.limits();
        log::info!("Adapter limits: {:#?}", limits);
        let (device, queue) = request_device_and_queue(&adapter)
            .await
            .map_err(|e| e.to_string())?;
        log::info!("Created adapter, device, and queue");
        let surface_config = configure_surface(&adapter, &surface, canvas.height(), canvas.width());
        surface.configure(&device, &surface_config);
        let clear_color = hex_to_wgpu_color(&clear_color).unwrap();
        log::info!("Created surface configuration and color: {:?}", clear_color);
        let stars = Star::generate(star_count as usize, star_size, false);
        let (circle_vertex_buffer, circle_index_buffer, index_count) =
            create_circle_buffer(&device);
        let star_buffer = create_star_buffer(&device, &stars);
        let multisampled_frame = create_multisampled_frame(&device, &surface_config);

        let screen_buffer =
            create_screen_size_buffer(&device, canvas.width() as f32, canvas.height() as f32);
        let (bind_group_layout, bind_group) = create_bind_group(&device, &screen_buffer);

        let render_pipeline = create_render_pipeline(&device, &surface_config, &bind_group_layout);

        Ok(NightSky {
            star_size,
            _instance: instance,
            surface,
            device,
            queue,
            surface_config,
            clear_color,
            stars,
            star_buffer,
            circle_vertex_buffer,
            circle_index_buffer,
            index_count,
            render_pipeline,
            multisampled_frame,
            bind_group,
        })
    }

    pub fn update_and_render(&mut self, delta_time: f32) {
        self.update(delta_time);
        self.render();
    }

    pub fn resize(&mut self, canvas: HtmlCanvasElement) -> Result<(), String> {
        self.surface_config.width = canvas.width();
        self.surface_config.height = canvas.height();
        self.surface.configure(&self.device, &self.surface_config);
        self.multisampled_frame = create_multisampled_frame(&self.device, &self.surface_config);
        Ok(())
    }

    fn update(&mut self, delta_time: f32) {
        self.stars
            .iter_mut()
            .for_each(|star| star.update(delta_time));
        self.queue
            .write_buffer(&self.star_buffer, 0, bytemuck::cast_slice(&self.stars));
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
            if !self.stars.is_empty() {
                render_pass.set_pipeline(&self.render_pipeline);
                render_pass.set_vertex_buffer(0, self.circle_vertex_buffer.slice(..));
                render_pass.set_vertex_buffer(1, self.star_buffer.slice(..)); // Instance buffer
                render_pass.set_index_buffer(
                    self.circle_index_buffer.slice(..),
                    wgpu::IndexFormat::Uint16,
                );
                render_pass.set_bind_group(0, &self.bind_group, &[]);
                render_pass.draw_indexed(0..self.index_count, 0, 0..self.stars.len() as u32);
            }
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

impl StarRender for NightSky {
    fn update_and_render(&mut self, delta_time: f32) {
        NightSky::update_and_render(self, delta_time);
    }

    fn resize(&mut self, canvas: HtmlCanvasElement) {
        NightSky::resize(self, canvas).unwrap();
    }

    fn add_stars(&mut self, count: u32) {
        self.stars
            .extend(Star::generate(count as usize, self.star_size, true));
        self.star_buffer = create_star_buffer(&self.device, &self.stars);
    }

    fn remove_stars(&mut self, count: u32) {
        self.stars
            .truncate(self.stars.len().saturating_sub(count as usize));
        self.star_buffer = create_star_buffer(&self.device, &self.stars);
    }

    fn get_num_stars(&self) -> u32 {
        self.stars.len() as u32
    }
}
