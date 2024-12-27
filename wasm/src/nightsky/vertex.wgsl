struct VertexOutput {
    @builtin(position) position: vec4<f32>,  // Position in clip space
    @location(0) quad_position: vec2<f32>,  // Position within the star's quad
    @location(1) brightness: f32           // Star brightness
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

    let ndc_position = position * 2.0 - vec2<f32>(1.0, 1.0);

    let scaled_size = size * 0.01;

    // Scale the offset by size and map the star's position to clip space
    //out.position = vec4<f32>((position * 2.0 - 1.0) + offset * size, 0.0, 1.0);
    out.position = vec4<f32>(ndc_position + offset * scaled_size, 0.0, 1.0);

    // Pass the relative position within the quad to the fragment shader
    out.quad_position = offset;
    //out.quad_position = offset * 0.5 + vec2<f32>(0.5, 0.5);

    // Pass brightness through
    out.brightness = brightness;
    return out;
}

