pub fn hex_to_wgpu_color(hex: &str) -> Result<wgpu::Color, String> {
    // Remove the '#' prefix if present
    let hex = hex.strip_prefix('#').unwrap_or(hex);

    // Validate length
    if hex.len() != 6 && hex.len() != 8 {
        return Err("Hex string must be 6 (RGB) or 8 (RGBA) characters long".into());
    }

    // Parse the color components
    let r = u8::from_str_radix(&hex[0..2], 16).map_err(|_| "Invalid red component")?;
    let g = u8::from_str_radix(&hex[2..4], 16).map_err(|_| "Invalid green component")?;
    let b = u8::from_str_radix(&hex[4..6], 16).map_err(|_| "Invalid blue component")?;
    let a = if hex.len() == 8 {
        u8::from_str_radix(&hex[6..8], 16).map_err(|_| "Invalid alpha component")?
    } else {
        255
    };

    // Convert sRGB to linear space
    let srgb_to_linear = |c: u8| {
        let c = c as f64 / 255.0;
        if c <= 0.04045 {
            c / 12.92
        } else {
            ((c + 0.055) / 1.055).powf(2.4)
        }
    };

    Ok(wgpu::Color {
        r: srgb_to_linear(r),
        g: srgb_to_linear(g),
        b: srgb_to_linear(b),
        a: a as f64 / 255.0, // Alpha remains in the [0, 1] range
    })
}

