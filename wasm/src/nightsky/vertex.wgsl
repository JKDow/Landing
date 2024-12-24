struct VertexOutput {
    @builtin(position) position: vec4<f32>;
    @location(0) brightness: f32; // Pass brightness to the fragment shader
};

@vertex
fn main(
    @location(0) position: vec2<f32>, // Star position
    @location(1) brightness: f32     // Star brightness
) -> VertexOutput {
    let adjusted_position = vec2<f32>(
        position.x * 2.0 - 1.0, // Convert to normalized device coordinates
        position.y * -2.0 + 1.0
    );

    var out: VertexOutput;
    out.position = vec4<f32>(adjusted_position, 0.0, 1.0);
    out.brightness = brightness; // Pass brightness through
    return out;
}
