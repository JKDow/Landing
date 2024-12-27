@fragment
fn main(
    @location(0) quad_position: vec2<f32>, // Position in the star's quad
    @location(1) brightness: f32           // Star brightness
) -> @location(0) vec4<f32> {
    let dist = length(quad_position);

    // Smoothly blend the edges
    let alpha = 1.0 - smoothstep(0.95, 1.0, dist);

    // Render the star with brightness and smooth edges
    return vec4<f32>(1.0, 1.0, 1.0, brightness * alpha);
}
