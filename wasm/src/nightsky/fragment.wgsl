@fragment
fn main(
    @location(0) quad_position: vec2<f32>, // Position in the star's quad
    @location(1) brightness: f32,          // Star brightness
    @location(2) size: f32                 // Star size
) -> @location(0) vec4<f32> {
    // Calculate the distance from the center of the quad
    let dist = length(quad_position);

    // Adjust smoothstep range based on size
    let edge_start = 1.0 - size * 0.5; // Inner edge based on size
    let edge_end = 1.0;                // Outer edge (always the same)

    // Smoothly blend the edges based on distance
    let alpha = 1.0 - smoothstep(edge_start, edge_end, dist);

    // Render the star with brightness and smooth edges
    return vec4<f32>(1.0, 1.0, 1.0, brightness * alpha);
}

