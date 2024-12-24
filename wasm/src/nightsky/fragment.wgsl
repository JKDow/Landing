@fragment
fn main(
    @location(0) brightness: f32      // Star brightness
) -> @location(0) vec4<f32> {
    return vec4<f32>(brightness, brightness, brightness, 1.0); // White star
}

