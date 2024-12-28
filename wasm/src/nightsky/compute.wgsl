@group(0) @binding(0) var<storage, read_write> stars: array<Star>;

struct Star {
    position: vec2<f32>,
    size: f32,
    brightness: f32,
    fade_speed: f32,
};

@compute @workgroup_size(64)
fn main(@builtin(global_invocation_id) id: vec3<u32>) {
    let index = id.x;
    if (index >= arrayLength(&stars)) {
        return;
    }

    // Access the star using a pointer
    let star = &stars[index];

    // Dereference pointer fields to modify
    (*star).brightness = (*star).brightness + (*star).fade_speed * 0.016; // delta_time = 0.016

    // Oscillate brightness between 0.0 and 1.0
    if ((*star).brightness > 1.0) {
        (*star).brightness = 1.0 - ((*star).brightness - 1.0);
        (*star).fade_speed = -(*star).fade_speed; // Reverse fade direction
    } else if ((*star).brightness < 0.0) {
        (*star).brightness = -(*star).brightness;
        (*star).fade_speed = -(*star).fade_speed; // Reverse fade direction
    }
}

