use wgpu::util::DeviceExt;

#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
pub struct ScreenSize {
    pub width: f32,
    pub height: f32,
    _padding: [u32; 2],
}

// Create a uniform buffer for the screen size
pub fn create_screen_size_buffer(device: &wgpu::Device, width: f32, height: f32) -> wgpu::Buffer {
    let screen_size = ScreenSize { width, height, _padding: [0; 2] };
    device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
        label: Some("Screen Size Buffer"),
        contents: bytemuck::cast_slice(&[screen_size]),
        usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
    })
}

