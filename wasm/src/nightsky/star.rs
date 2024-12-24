#[repr(C)]
#[derive(Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
/// Star struct
/// Position: X and Y coordinates
/// Size: Radius of the star
/// Brightness: Brightness of the star (0.0 to 1.0)
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
                    (2.0 + 2.0 * js_sys::Math::random()) as f32,
                    js_sys::Math::random() as f32,
                )
            })
            .collect()
    }
}
