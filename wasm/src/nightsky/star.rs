use crate::utils::random_range;

const VEL_MOD: f32 = 0.005;
const STAR_COLORS: [[f32; 3]; 5] = [
    [0.8, 0.9, 1.0], // Blue-white
    [1.0, 1.0, 1.0], // White
    [1.0, 0.9, 0.8], // Yellow-white
    [1.0, 0.8, 0.6], // Light orange
    [1.0, 0.6, 0.4], // Reddish-orange
];

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
    pub color: [f32; 3], // RGB color of the star
    pub fade_speed: f32, // (0.0 to 1.0)
    pub velocity: [f32; 2],
}

impl Star {
    pub fn new(
        x: f32,
        y: f32,
        size: f32,
        brightness: f32,
        fade_speed: f32,
        x_vel: f32,
        y_vel: f32,
        color: [f32; 3],
    ) -> Self {
        Self {
            position: [x, y],
            size,
            brightness,
            color,
            fade_speed,
            velocity: [x_vel, y_vel],
        }
    }

    pub fn generate(count: usize, size: f32, dim: bool) -> Vec<Star> {
        let size = size.clamp(0.1, 4.0);
        let brightness = if dim { 0.0 } else { random_range(0.2, 1.0) };
        (0..count)
            .map(|_| {
                let base_color = STAR_COLORS[random_range(0.0, STAR_COLORS.len() as f32) as usize];
                let color_variation = |c: f32| (c + random_range(-0.1, 0.1)).clamp(0.0, 1.0);
                let color = [
                    color_variation(base_color[0]), // Red
                    color_variation(base_color[1]), // Green
                    color_variation(base_color[2]), // Blue
                ];
                Star::new(
                    random_range(-1.0, 1.0),
                    random_range(-1.0, 1.0),
                    random_range(0.001, 0.003) * size,
                    brightness,
                    random_range(0.08, 0.3),
                    random_range(-1.0, 1.0) * VEL_MOD,
                    random_range(-1.0, 1.0) * VEL_MOD,
                    color,
                )
            })
            .collect()
    }

    pub fn update(&mut self, delta_time: f32) {
        self.brightness += self.fade_speed * delta_time;
        if self.brightness >= 1.0 {
            self.brightness = 1.0;
            self.fade_speed = -self.fade_speed;
        } else if self.brightness <= 0.0 {
            self.brightness = 0.0;
            self.fade_speed = -self.fade_speed;
            self.refresh_position();
        }
        self.position[0] += self.velocity[0] * delta_time;
        self.position[1] += self.velocity[1] * delta_time;
    }

    fn refresh_position(&mut self) {
        self.position[0] = js_sys::Math::random() as f32 * 2.0 - 1.0;
        self.position[1] = js_sys::Math::random() as f32 * 2.0 - 1.0;
    }

    const ATTR: [wgpu::VertexAttribute; 4] = wgpu::vertex_attr_array![
        // Position
        1 => Float32x2,
        // Size
        2 => Float32,
        // Brightness
        3 => Float32,
        // Color
        4 => Float32x3,
    ];

    pub fn desc() -> wgpu::VertexBufferLayout<'static> {
        wgpu::VertexBufferLayout {
            array_stride: std::mem::size_of::<Self>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Instance,
            attributes: &Self::ATTR,
        }
    }
}
