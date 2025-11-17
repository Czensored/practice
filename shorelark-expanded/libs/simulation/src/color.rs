#[derive(Debug, Clone, Copy)]
pub struct Rgba {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

#[inline]
pub fn rgba_from_bytes(bytes: &[u8]) -> Rgba {
    if bytes.is_empty() {
        return Rgba {
            r: 255,
            g: 255,
            b: 255,
            a: 255,
        };
    }

    // Smooth hue from circular projection
    let w = 0.61803398875_f32; // golden-ratio-ish frequency
    let (mut sx, mut sy) = (0.0_f32, 0.0_f32);
    let (mut mn, mut mx, mut sum) = (u8::MAX, u8::MIN, 0u32);

    for (i, &b) in bytes.iter().enumerate() {
        let bf = b as f32;
        let t = i as f32 * w;
        sx += bf * t.cos();
        sy += bf * t.sin();
        mn = mn.min(b);
        mx = mx.max(b);
        sum += b as u32;
    }

    let hue_deg = sy.atan2(sx).to_degrees().rem_euclid(360.0);
    let mean = (sum as f32) / (bytes.len() as f32); // 0..255
    let spread = (mx - mn) as f32 / 255.0; // 0..1
    let s = (0.6 + 0.35 * spread).min(1.0); // vivid but stable
    let v = (0.6 + 0.4 * (mean / 255.0)).min(1.0); // brightness from mean

    let (r, g, b) = hsv_to_rgb(hue_deg, s, v);
    Rgba {
        r: (r * 255.0).round() as u8,
        g: (g * 255.0).round() as u8,
        b: (b * 255.0).round() as u8,
        a: 255,
    }
}

#[inline]
fn hsv_to_rgb(h_deg: f32, s: f32, v: f32) -> (f32, f32, f32) {
    let h6 = (h_deg.rem_euclid(360.0)) / 60.0;
    let i = h6.floor();
    let f = h6 - i;
    let p = v * (1.0 - s);
    let q = v * (1.0 - s * f);
    let t = v * (1.0 - s * (1.0 - f));
    match i as i32 {
        0 => (v, t, p),
        1 => (q, v, p),
        2 => (p, v, t),
        3 => (p, q, v),
        4 => (t, p, v),
        _ => (v, p, q),
    }
}
