use crate::config::{ColorSpec, Colors};
use nu_ansi_term::Color::{self, Rgb};

pub fn hsl_to_rgb(h: f64, s: f64, l: f64) -> (u8, u8, u8) {
    let s = s / 100.0;
    let l = l / 100.0;
    let c = (1.0 - (2.0 * l - 1.0).abs()) * s;
    let h = h / 60.0;
    let x = c * (1.0 - ((h % 2.0) - 1.0).abs());
    let m = l - c / 2.0;
    let (r, g, b) = if h < 1.0 { (c, x, 0.0) } else if h < 2.0 { (x, c, 0.0) } else if h < 3.0 { (0.0, c, x) } else if h < 4.0 { (0.0, x, c) } else if h < 5.0 { (x, 0.0, c) } else { (c, 0.0, x) };
    (((r + m) * 255.0) as u8, ((g + m) * 255.0) as u8, ((b + m) * 255.0) as u8)
}

pub fn parse_color(spec: &ColorSpec) -> Option<(u8, u8, u8)> {
    match spec {
        ColorSpec::Rgb(rgb) => {
            if rgb.len() == 3 { Some((rgb[0], rgb[1], rgb[2])) } else { None }
        }
        ColorSpec::Hex(hex) => {
            let hex = hex.trim_start_matches('#');
            if hex.len() == 6 {
                let r = u8::from_str_radix(&hex[0..2], 16).ok()?;
                let g = u8::from_str_radix(&hex[2..4], 16).ok()?;
                let b = u8::from_str_radix(&hex[4..6], 16).ok()?;
                Some((r, g, b))
            } else if hex.len() == 3 {
                let r = u8::from_str_radix(&hex[0..1].repeat(2), 16).ok()?;
                let g = u8::from_str_radix(&hex[1..2].repeat(2), 16).ok()?;
                let b = u8::from_str_radix(&hex[2..3].repeat(2), 16).ok()?;
                Some((r, g, b))
            } else { None }
        }
        ColorSpec::Hsl(hsl_str) => {
            let hsl_str = hsl_str.trim();
            if !hsl_str.starts_with("hsl(") || !hsl_str.ends_with(')') { return None; }
            let inner = &hsl_str[4..hsl_str.len() - 1];
            let parts: Vec<&str> = inner.split(',').map(|s| s.trim()).collect();
            if parts.len() != 3 { return None; }
            let h_raw = parts[0];
            let s_raw = parts[1].trim_end_matches('%');
            let l_raw = parts[2].trim_end_matches('%');
            let h: f64 = h_raw.parse().ok()?;
            let s: f64 = s_raw.parse().ok()?;
            let l: f64 = l_raw.parse().ok()?;
            Some(hsl_to_rgb(h, s, l))
        }
    }
}

pub fn color_from_spec(spec: &ColorSpec) -> Color {
    let (r, g, b) = parse_color(spec).expect("Invalid color in palette");
    Rgb(r, g, b)
}

pub fn gradient_green_to_red(percentage: f64, _bg: (u8, u8, u8), is_light_bg: bool) -> Color {
    let percentage = percentage.clamp(0.0, 100.0);
    let (green, yellow, red) = if is_light_bg {
        ((0.0, 150.0, 0.0), (200.0, 180.0, 0.0), (180.0, 0.0, 0.0))
    } else {
        ((80.0, 255.0, 80.0), (255.0, 240.0, 80.0), (255.0, 80.0, 80.0))
    };
    let (r, g, b) = if percentage <= 50.0 {
        let t = percentage / 50.0;
        (green.0 + (yellow.0 - green.0) * t, green.1 + (yellow.1 - green.1) * t, green.2 + (yellow.2 - green.2) * t)
    } else {
        let t = (percentage - 50.0) / 50.0;
        (yellow.0 + (red.0 - yellow.0) * t, yellow.1 + (red.1 - yellow.1) * t, yellow.2 + (red.2 - yellow.2) * t)
    };
    Rgb(r as u8, g as u8, b as u8)
}

pub fn get_colors(config_colors: Option<&Colors>, is_light_bg: bool) -> Colors {
    match config_colors {
        Some(colors) => colors.clone(),
        None => {
            let text_color = if is_light_bg { ColorSpec::Rgb(vec![0, 0, 0]) } else { ColorSpec::Rgb(vec![255, 255, 255]) };
            Colors {
                background: ColorSpec::Rgb(vec![240, 160, 80]),
                model: text_color.clone(),
                cwd: text_color.clone(),
                time: text_color.clone(),
                git_clean: text_color.clone(),
                delta: text_color.clone(),
                cost: text_color.clone(),
                cache: text_color.clone(),
                separator: ColorSpec::Rgb(vec![220, 100, 0]),
            }
        }
    }
}

pub fn validate_colors(colors: &Colors) -> Result<(), String> {
    let specs = [&colors.background, &colors.model, &colors.cwd, &colors.time, &colors.git_clean, &colors.delta, &colors.cost, &colors.cache, &colors.separator];
    for spec in specs {
        if parse_color(spec).is_none() { return Err(format!("Invalid color specification: {:?}", spec)); }
    }
    Ok(())
}

fn brightness(rgb: (u8, u8, u8)) -> f64 { 0.2126 * (rgb.0 as f64) + 0.7152 * (rgb.1 as f64) + 0.0722 * (rgb.2 as f64) }

pub fn adjust_colors_for_background(mut colors: Colors, is_light_bg: bool) -> Colors {
    let adjust = |spec: &ColorSpec, light_bg: bool| -> ColorSpec {
        if let Some(rgb) = parse_color(spec) {
            let b = brightness(rgb);
            if !light_bg && b < 80.0 { return ColorSpec::Rgb(vec![255, 255, 255]); }
            if light_bg && b > 175.0 { return ColorSpec::Rgb(vec![0, 0, 0]); }
        }
        spec.clone()
    };
    colors.model = adjust(&colors.model, is_light_bg);
    colors.cwd = adjust(&colors.cwd, is_light_bg);
    colors.time = adjust(&colors.time, is_light_bg);
    colors.git_clean = adjust(&colors.git_clean, is_light_bg);
    colors.delta = adjust(&colors.delta, is_light_bg);
    colors.cost = adjust(&colors.cost, is_light_bg);
    colors.cache = adjust(&colors.cache, is_light_bg);
    colors.separator = adjust(&colors.separator, is_light_bg);
    colors
}

