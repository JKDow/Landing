use super::utils::random_range;

const VEL_MOD: f32 = 0.005;


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
    pub fade_speed: f32, // (0.0 to 1.0)
    pub velocity: [f32; 2],
}

impl Star {
    pub fn new(x: f32, y: f32, size: f32, brightness: f32, fade_speed: f32, x_vel: f32, y_vel: f32) -> Self {
        Self {
            position: [x, y],
            size,
            brightness,
            fade_speed,
            velocity: [x_vel, y_vel],
        }
    }

    pub fn generate(count: usize) -> Vec<Star> {
        (0..count)
            .map(|_| {
                Star::new(
                    random_range(-1.0, 1.0),
                    random_range(-1.0, 1.0),
                    random_range(0.001, 0.003),
                    random_range(0.2, 1.0),
                    random_range(0.08, 0.3),
                    random_range(-1.0, 1.0) * VEL_MOD,
                    random_range(-1.0, 1.0) * VEL_MOD,
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

    const ATTR: [wgpu::VertexAttribute; 3] = wgpu::vertex_attr_array![
        // Position
        1 => Float32x2,
        // Size
        2 => Float32,
        // Brightness
        3 => Float32,
    ];

    pub fn desc() -> wgpu::VertexBufferLayout<'static> {
        wgpu::VertexBufferLayout {
            array_stride: std::mem::size_of::<Self>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Instance,
            attributes: &Self::ATTR,
        }
    }
}
