// Vertex shader
@group(0) @binding(0) var<uniform> screen_size: vec2<f32>;

struct VertexInput {
    @location(0) position: vec2<f32>;
    @location(1) color: vec4<f32>;
    @location(2) size: f32;
};

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>;
    @location(0) frag_color: vec4<f32>;
};

@vertex
fn vs_main(input: VertexInput) -> VertexOutput {
    var output: VertexOutput;
    // Convert position to clip space
    output.clip_position = vec4(input.position / screen_size * 2.0 - vec2(1.0, 1.0), 0.0, 1.0);
    output.frag_color = input.color;
    return output;
}

// Fragment shader
@fragment
fn fs_main(input: VertexOutput) -> @location(0) vec4<f32> {
    return input.frag_color;
}

