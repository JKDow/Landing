@fragment
fn main(
    @location(0) quad_position: vec2<f32>, // Position in the star's quad
    @location(1) brightness: f32           // Star brightness
) -> @location(0) vec4<f32> {
    // Calculate distance from the center of the quad
    let dist = length(quad_position);

    // Only render pixels inside the circle
    if (dist > 1.0) {
        discard;
    }

    // Render the star with brightness applied
    return vec4<f32>(1.0, 1.0, 1.0, brightness);
}
