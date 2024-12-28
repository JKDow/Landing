struct VertexInput {
    @location(0) position: vec2<f32>,         // Circle vertex position
    @location(1) instance_position: vec2<f32>, // Star instance position
    @location(2) size: f32,                  // Star instance size
    @location(3) brightness: f32,           // Star brightness
};

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>, // Position in clip space
    @location(0) brightness: f32,               // Brightness passed to fragment shader
    @location(1) unit_circle_position: vec2<f32>, // Position in the unit circle
};

@group(0) @binding(0)
var<uniform> screen_size: vec4<f32>;

@vertex
fn main(input: VertexInput) -> VertexOutput {
    var output: VertexOutput;

    // Calculate clip-space position
    output.clip_position = vec4<f32>(
        input.position * input.size + input.instance_position,
        0.0,
        1.0,
    );

    // Convert position to screen space and normalize
    let screen_dimensions = screen_size.xy;
    let screen_position = input.position * input.size;
    output.unit_circle_position = screen_position / screen_dimensions;

    // Pass brightness
    output.brightness = input.brightness;

    return output;
}
