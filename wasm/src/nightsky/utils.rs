pub fn setup_logger() {
    console_log::init_with_level(log::Level::Info).expect("Couldn't initialize logger");
    std::panic::set_hook(Box::new(console_error_panic_hook::hook));
    log::info!("Started wasm logger");
}

pub fn hex_to_wgpu_color(hex: &str) -> Result<wgpu::Color, String> {
    // Remove the '#' prefix if present
    let hex = hex.strip_prefix('#').unwrap_or(hex);
    let parse_rgb = |hex: &str| -> Result<[u8; 3], String> {
        let mut vals: [u8; 3] = [0, 0, 0];
        vals[0] = u8::from_str_radix(&hex[0..2], 16).map_err(|_| "Invalid red component")?;
        vals[1] = u8::from_str_radix(&hex[2..4], 16).map_err(|_| "Invalid green component")?;
        vals[2] = u8::from_str_radix(&hex[4..6], 16).map_err(|_| "Invalid blue component")?;
        Ok(vals)
    };
    let make_color = |rgb: [u8; 3], a: u8| wgpu::Color {
        r: rgb[0] as f64 / 255.0,
        g: rgb[1] as f64 / 255.0,
        b: rgb[2] as f64 / 255.0,
        a: a as f64 / 255.0,
    };

    // Parse the string based on its length
    let rgb = parse_rgb(hex)?;
    match hex.len() {
        6 => Ok(make_color(rgb, 255)),
        8 => {
            let a = u8::from_str_radix(&hex[6..8], 16).map_err(|_| "Invalid alpha component")?;
            Ok(make_color(rgb, a))
        }
        _ => Err("Hex string must be 6 (RGB) or 8 (RGBA) characters long".into()),
    }
}

pub fn random_range(min: f32, max: f32) -> f32 {
    js_sys::Math::random() as f32 * (max - min) + min
}
