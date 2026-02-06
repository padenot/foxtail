use crate::colors::{color_from_spec, gradient_green_to_red};
use crate::config::{get_symbol, ColorSpec, Colors, Config};
use crate::git::{commit_warning_color, format_duration, format_duration_ms, get_git_info};
use crate::types::StatusInput;
use nu_ansi_term::Color::{self, Rgb};
use nu_ansi_term::Style;

use std::path::{Component, Path};

pub fn abbreviate_path(path: &str) -> String {
    let p = Path::new(path);
    if path.is_empty() {
        return String::new();
    }
    let mut parts: Vec<String> = Vec::new();
    let mut is_abs = false;
    for comp in p.components() {
        match comp {
            Component::RootDir => {
                is_abs = true;
            }
            Component::Normal(os) => {
                parts.push(os.to_string_lossy().to_string());
            }
            _ => {}
        }
    }
    if parts.is_empty() {
        return if is_abs {
            std::path::MAIN_SEPARATOR.to_string()
        } else {
            String::new()
        };
    }
    let last_idx = parts.len() - 1;
    let mut abbreviated: Vec<String> = Vec::with_capacity(parts.len());
    for (i, component) in parts.iter().enumerate() {
        if i == last_idx {
            abbreviated.push(component.to_string());
        } else if component.starts_with('.') {
            if component.len() > 1 {
                let first_char = component.chars().nth(1).unwrap_or('.');
                abbreviated.push(format!(".{}", first_char));
            } else {
                abbreviated.push(component.to_string());
            }
        } else {
            let first_char = component.chars().next().unwrap_or('?');
            abbreviated.push(first_char.to_string());
        }
    }
    let joined = abbreviated.join(std::path::MAIN_SEPARATOR_STR);
    if is_abs {
        if joined.is_empty() {
            std::path::MAIN_SEPARATOR.to_string()
        } else {
            format!("{}{}", std::path::MAIN_SEPARATOR, joined)
        }
    } else {
        joined
    }
}

pub fn fox_head(color_enabled: bool) -> String {
    if !color_enabled {
        return ">>".to_string();
    }
    format!(
        "{}{}{}{}",
        Rgb(0, 0, 0).paint("🭈"),
        Rgb(250, 248, 230).paint("🭄"),
        Rgb(255, 180, 40).paint("█"),
        Rgb(255, 140, 0).paint("█")
    )
}

pub fn fox_tail(bg: Option<Color>, color_enabled: bool) -> String {
    if !color_enabled {
        return "<<".to_string();
    }
    format!(
        "{}{}{}{}{}{}{}{}{}{}{}{}",
        bg.map(|b| Style::new().on(b).paint(" "))
            .unwrap_or_else(|| Style::new().paint(" ")),
        Rgb(255, 140, 0).paint("🭝"),
        Rgb(255, 160, 20).paint("🭓"),
        Rgb(255, 245, 200).paint("▇"),
        Rgb(255, 160, 20).paint("▅"),
        Rgb(255, 245, 200).paint("▆"),
        Rgb(255, 160, 20).paint("▄"),
        Rgb(255, 245, 200).paint("▃"),
        Rgb(255, 160, 20).paint("▄"),
        Rgb(255, 245, 200).paint("▃"),
        Rgb(250, 248, 230).paint("▂"),
        Rgb(255, 245, 200).paint("🬽")
    )
}

pub fn render_ctx(
    pct: f64,
    size: u32,
    sym: &str,
    bg: Option<Color>,
    bg_rgb: (u8, u8, u8),
    light: bool,
    color_enabled: bool,
) -> String {
    let color = gradient_green_to_red(pct, bg_rgb, light);
    // pct is the USED percentage; compute used tokens for display
    let used = size as f64 * pct / 100.0;
    let text = format!("{}{:.0}% ({:.0}k)", sym, pct, used / 1000.0);
    if !color_enabled {
        return text;
    }
    let s = Style::new().fg(color).bold();
    (if let Some(b) = bg { s.on(b) } else { s })
        .paint(text)
        .to_string()
}

pub fn render_git(
    cwd: &str,
    cfg: &Config,
    col: &ColorSpec,
    bg: Option<Color>,
    bg_rgb: (u8, u8, u8),
    light: bool,
    color_enabled: bool,
) -> String {
    get_git_info(cwd, cfg)
        .map(|(t, a, r)| {
            let dur = format_duration(t);
            if a + r > 0 {
                let text = format!("{} (+{} -{}){}", dur, a, r, get_symbol("git_warning", cfg));
                if !color_enabled {
                    return text;
                }
                let c = commit_warning_color(a + r, bg_rgb, light);
                let s = Style::new().fg(c).bold();
                (if let Some(b) = bg { s.on(b) } else { s })
                    .paint(text)
                    .to_string()
            } else {
                let text = format!("{}{}", get_symbol("git_clean", cfg), dur);
                if !color_enabled {
                    return text;
                }
                let s = color_from_spec(col).bold();
                (if let Some(b) = bg { s.on(b) } else { s })
                    .paint(text)
                    .to_string()
            }
        })
        .unwrap_or_default()
}

pub fn paint_with(col: &ColorSpec, text: String, bg: Option<Color>, color_enabled: bool) -> String {
    if !color_enabled {
        return text;
    }
    let s = crate::colors::color_from_spec(col).bold();
    (if let Some(b) = bg { s.on(b) } else { s })
        .paint(text)
        .to_string()
}

#[rustfmt::skip]
pub fn build_replacements<'a>(
    d: &'a StatusInput,
    cfg: &'a Config,
    col: &'a Colors,
    bg: Option<Color>,
    bg_rgb: (u8, u8, u8),
    light: bool,
    color_enabled: bool,
) -> [(&'static str, String); 11] {
    let sym = |k| get_symbol(k, cfg);
    [
        ("head",        cfg.visual.as_ref().and_then(|v| v.head.clone()).unwrap_or_else(|| fox_head(color_enabled))),
        ("tail",        cfg.visual.as_ref().and_then(|v| v.tail.clone()).unwrap_or_else(|| fox_tail(bg, color_enabled))),
        ("model",       paint_with(&col.model, d.model.display_name.clone(), bg, color_enabled)),
        ("cwd",         paint_with(&col.cwd, format!("{}{}", sym("cwd"), d.workspace.current_dir), bg, color_enabled)),
        ("cwdcompact",  paint_with(&col.cwd, format!("{}{}", sym("cwd"), abbreviate_path(&d.workspace.current_dir)), bg, color_enabled)),
        ("duration",    paint_with(&col.time, format!("{}{}", sym("time"), format_duration_ms(d.cost.total_duration_ms)), bg, color_enabled)),
        ("ctx",         render_ctx(d.context_window.used_percentage, d.context_window.context_window_size, &sym("context"), bg, bg_rgb, light, color_enabled)),
        ("claudedelta", paint_with(&col.delta, format!("{}+{} -{}", sym("delta"), d.cost.total_lines_added, d.cost.total_lines_removed), bg, color_enabled)),
        ("gitdelta",    render_git(&d.cwd, cfg, &col.git_clean, bg, bg_rgb, light, color_enabled)),
        ("cost",        paint_with(&col.cost, format!("{}{:.3}", sym("cost"), d.cost.total_cost_usd), bg, color_enabled)),
        ("cache",       d.context_window.current_usage.as_ref()
            .map(|u| paint_with(&col.cache, format!("{}r:{:.0}k w:{:.0}k", sym("cache"), u.cache_read_input_tokens as f64 / 1000.0, u.cache_creation_input_tokens as f64 / 1000.0), bg, color_enabled))
            .unwrap_or_default()),
    ]
}

pub fn render_warning(
    d: &StatusInput,
    cfg: &Config,
    bg_rgb: (u8, u8, u8),
    light: bool,
    color_enabled: bool,
) -> String {
    let used_tokens = (d.context_window.context_window_size as f64
        * d.context_window.used_percentage
        / 100.0) as u32;
    let used_k = used_tokens as f64 / 1000.0;
    let total_k = d.context_window.context_window_size as f64 / 1000.0;
    let mut text = cfg.warning_message.clone().unwrap_or_else(||
        "⚠ Context warning: {used_pct:.1}% in first minute ({used_k:.0}k/{total_k:.0}k tokens) ⚠".to_string()
    );
    text = text.replace(
        "{used_pct:.1}",
        &format!("{:.1}", d.context_window.used_percentage),
    );
    text = text.replace(
        "{used_pct}",
        &format!("{:.1}", d.context_window.used_percentage),
    );
    text = text.replace("{used_k:.0}", &format!("{:.0}", used_k));
    text = text.replace("{used_k}", &format!("{:.0}", used_k));
    text = text.replace("{total_k:.0}", &format!("{:.0}", total_k));
    text = text.replace("{total_k}", &format!("{:.0}", total_k));
    if !color_enabled {
        return text;
    }
    let bg = Rgb(bg_rgb.0, bg_rgb.1, bg_rgb.2);
    let s = Style::new()
        .fg(gradient_green_to_red(100.0, bg_rgb, light))
        .bold();
    (if cfg.enable_background { s.on(bg) } else { s })
        .paint(text)
        .to_string()
}

pub fn render_formatted(
    fmt: &str,
    d: &StatusInput,
    cfg: &Config,
    col: &Colors,
    bg_rgb: (u8, u8, u8),
    light: bool,
    color_enabled: bool,
) -> String {
    let bg = if cfg.enable_background && color_enabled {
        Some(Rgb(bg_rgb.0, bg_rgb.1, bg_rgb.2))
    } else {
        None
    };
    let mut s = fmt.to_string();
    for (k, v) in build_replacements(d, cfg, col, bg, bg_rgb, light, color_enabled) {
        s = s.replace(&format!("{{{}}}", k), &v);
    }
    let sep_str = cfg
        .visual
        .as_ref()
        .and_then(|v| v.separator.clone())
        .unwrap_or_else(|| " | ".to_string());
    let joiner = if color_enabled {
        let c = color_from_spec(&col.separator);
        let sty = Style::new().fg(c);
        (if let Some(b) = bg { sty.on(b) } else { sty })
            .paint(sep_str.clone())
            .to_string()
    } else {
        sep_str.clone()
    };
    s.split('|')
        .map(|s| s.trim())
        .filter(|s| !s.is_empty())
        .collect::<Vec<_>>()
        .join(&joiner)
}
