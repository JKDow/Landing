pub const CIRCLE_SEGMENTS: u32 = 64;

pub struct Circle {
    pub vertices: Vec<[f32; 2]>, // 2D positions for the circle
    pub indices: Vec<u16>,       // Indices for the circle's triangles
}

impl Circle {
    pub fn new() -> Self {
        let mut vertices = vec![[0.0, 0.0]]; // Center vertex
        let mut indices = Vec::new();

        let angle_step = std::f32::consts::TAU / CIRCLE_SEGMENTS as f32;
        for i in 0..CIRCLE_SEGMENTS {
            let angle = i as f32 * angle_step;
            let x = angle.cos();
            let y = angle.sin();
            vertices.push([x, y]);
        }

        for i in 1..=CIRCLE_SEGMENTS {
            let next = if i == CIRCLE_SEGMENTS { 1 } else { i + 1 };
            indices.push(0); // Center vertex
            indices.push(i as u16);
            indices.push(next as u16);
        }

        Self { vertices, indices }
    }

    pub const ATTR: [wgpu::VertexAttribute; 1] = wgpu::vertex_attr_array![
        // Position
        0 => Float32x2
    ];

    pub fn desc() -> wgpu::VertexBufferLayout<'static> {
        wgpu::VertexBufferLayout {
            array_stride: std::mem::size_of::<[f32; 2]>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &Self::ATTR,
        }
    }
}
