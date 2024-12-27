/// Star struct
/// Position: X and Y coordinates
/// Size: Radius of the star
/// Brightness: Brightness of the star (0.0 to 1.0)
#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
pub struct Star {
    /// X and Y position (x, y) coordinates
    pub position: [f32; 2],
    /// Radius of the star
    pub size: f32,
    /// Brightness of the star (0.0 to 1.0)
    pub brightness: f32,
}

impl Star {
    pub fn new(x: f32, y: f32, size: f32, brightness: f32) -> Self {
        Self {
            position: [x, y],
            size,
            brightness,
        }
    }

    pub fn generate(count: usize) -> Vec<Star> {
        (0..count)
            .map(|_| {
                Star::new(
                    js_sys::Math::random() as f32,
                    js_sys::Math::random() as f32,
                    js_sys::Math::random() as f32,
                    js_sys::Math::random() as f32,
                )
            })
            .collect()
    }

    const ATTR: [wgpu::VertexAttribute; 3] = wgpu::vertex_attr_array![
        // Position
        0 => Float32x2,
        // Size
        1 => Float32,
        // Brightness
        2 => Float32
    ];

    pub fn desc() -> wgpu::VertexBufferLayout<'static> {
        wgpu::VertexBufferLayout {
            array_stride: std::mem::size_of::<Self>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Instance,
            attributes: &Self::ATTR,
        }
    }
}
