struct FragmentInput {
    @location(0) brightness: f32,      // Brightness from the vertex shader
    @location(1) unit_circle_position: vec2<f32>, // Normalized position within the unit circle
};

@fragment
fn main(input: FragmentInput) -> @location(0) vec4<f32> {
    // Calculate the distance from the center of the unit circle
    let dist = length(input.unit_circle_position);

    // Smoothly fade the edges of the circle
    let alpha = 1.0 - smoothstep(0.8, 1.0, dist); // Adjust the edge smoothness

    // Set the color to white with brightness controlling alpha
    return vec4<f32>(1.0, 1.0, 1.0, input.brightness * alpha);
}

