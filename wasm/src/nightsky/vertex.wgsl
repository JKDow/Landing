struct VertexOutput {
    @builtin(position) position: vec4<f32>,  // Position in clip space
    @location(0) quad_position: vec2<f32>,  // Position within the star's quad
    @location(1) brightness: f32,           // Star brightness
    @location(2) size: f32                  // Star size
};

@vertex
fn main(
    @builtin(vertex_index) vertex_index: u32,
    @builtin(instance_index) instance_index: u32, // Index of the current instance
    @location(0) position: vec2<f32>,            // Star position (per instance)
    @location(1) size: f32,                      // Star size (per instance)
    @location(2) brightness: f32
) -> VertexOutput {
    // Define the four corners of the quad
    let offsets = array<vec2<f32>, 4>(
        vec2<f32>(-1.0, -1.0), // Bottom-left
        vec2<f32>( 1.0, -1.0), // Bottom-right
        vec2<f32>(-1.0,  1.0), // Top-left
        vec2<f32>( 1.0,  1.0)  // Top-right
    );

    var out: VertexOutput;
    let offset = offsets[vertex_index % 4u];

    // Map the quad's position and scale it by size
    out.position = vec4<f32>(position + offset * size, 0.0, 1.0);

    // Pass attributes to the fragment shader
    out.quad_position = offset;
    out.brightness = brightness;
    out.size = size; // Pass the size to the fragment shader
    return out;
}
