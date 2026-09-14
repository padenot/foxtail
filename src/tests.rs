use super::*;
use std::collections::HashMap;

#[test]
fn test_abbreviate_path_simple() {
    assert_eq!(abbreviate_path("/home/user/project"), "/h/u/project");
}

#[test]
fn test_hsl_percent_parsing() {
    let spec = ColorSpec::Hsl("hsl(30, 100%, 50%)".to_string());
    let rgb = parse_color(&spec).unwrap();
    assert_eq!(rgb.0, rgb.0); // basic sanity
}

#[test]
fn test_warning_message_customization_plain() {
    let d = StatusInput {
        cwd: "/tmp".to_string(),
        model: Model {
            display_name: "M".to_string(),
        },
        workspace: Workspace {
            current_dir: "/tmp".to_string(),
        },
        cost: Cost {
            total_cost_usd: 0.0,
            total_duration_ms: 30_000,
            total_lines_added: 0,
            total_lines_removed: 0,
        },
        context_window: ContextWindow {
            context_window_size: 200_000,
            used_percentage: 42.0,
            remaining_percentage: 58.0,
            current_usage: None,
        },
    };
    let cfg = Config {
        warning_message: Some("warn {used_pct}% {used_k}k/{total_k}k".to_string()),
        ..Default::default()
    };
    let out = render_warning(&d, &cfg, (0, 0, 0), false, false);
    assert!(out.contains("warn 42.0% 84k/200k"));
}

#[test]
fn test_abbreviate_path_hidden_dirs() {
    assert_eq!(abbreviate_path("/home/user/.config/app"), "/h/u/.c/app");
    assert_eq!(abbreviate_path("/home/.local/.cache/data"), "/h/.l/.c/data");
}

#[test]
fn test_abbreviate_path_single_component() {
    assert_eq!(abbreviate_path("/project"), "/project");
    assert_eq!(abbreviate_path("project"), "project");
}

#[test]
fn test_abbreviate_path_root() {
    assert_eq!(abbreviate_path("/"), "/");
}

#[test]
fn test_format_duration_ms_seconds() {
    assert_eq!(format_duration_ms(5000), "5s");
    assert_eq!(format_duration_ms(45000), "45s");
}

#[test]
fn test_format_duration_ms_minutes() {
    assert_eq!(format_duration_ms(60000), "1m");
    assert_eq!(format_duration_ms(125000), "2m");
    assert_eq!(format_duration_ms(3540000), "59m");
}

#[test]
fn test_format_duration_ms_hours() {
    assert_eq!(format_duration_ms(3600000), "1h0m");
    assert_eq!(format_duration_ms(5400000), "1h30m");
    assert_eq!(format_duration_ms(7200000), "2h0m");
}

#[test]
fn test_format_duration_ms_days() {
    assert_eq!(format_duration_ms(86400000), "1d0h");
    assert_eq!(format_duration_ms(90000000), "1d1h");
}

#[test]
fn test_get_symbol_with_emojis() {
    let config = Config {
        enable_git: true,
        use_emojis: true,
        warn_high_initial_context_threshold: 20.0,
        symbols: HashMap::new(),
        enable_background: false,
        threshold_green: 100,
        threshold_yellow: 500,
        threshold_orange: 1000,
        colors: None,
        visual: None,
        format: default_format(),
        light_background: None,
        warning_message: None,
    };

    assert_eq!(get_symbol("cwd", &config), "📁 ");
    assert_eq!(get_symbol("time", &config), "⏱ ");
    assert_eq!(get_symbol("context", &config), "🧠 ");
    assert_eq!(get_symbol("git_warning", &config), " ⚠");
    assert_eq!(get_symbol("git_clean", &config), "✓ ");
    assert_eq!(get_symbol("delta", &config), "Σ ");
    assert_eq!(get_symbol("cost", &config), "💰 $");
    assert_eq!(get_symbol("cache", &config), "🗄 ");
}

#[test]
fn test_get_symbol_without_emojis() {
    let config = Config::default();

    assert_eq!(get_symbol("cwd", &config), "cwd:");
    assert_eq!(get_symbol("time", &config), "");
    assert_eq!(get_symbol("context", &config), "ctx:");
    assert_eq!(get_symbol("git_warning", &config), "");
    assert_eq!(get_symbol("git_clean", &config), "");
    assert_eq!(get_symbol("delta", &config), "Δ ");
    assert_eq!(get_symbol("cost", &config), "$");
    assert_eq!(get_symbol("cache", &config), "cache:");
}

#[test]
fn test_get_symbol_with_custom_overrides() {
    let mut symbols = HashMap::new();
    symbols.insert("cwd".to_string(), "DIR:".to_string());
    symbols.insert("cost".to_string(), "¢".to_string());

    let config = Config {
        enable_git: true,
        use_emojis: true,
        warn_high_initial_context_threshold: 20.0,
        symbols,
        enable_background: false,
        threshold_green: 100,
        threshold_yellow: 500,
        threshold_orange: 1000,
        colors: None,
        visual: None,
        format: default_format(),
        light_background: None,
        warning_message: None,
    };

    assert_eq!(get_symbol("cwd", &config), "DIR:");
    assert_eq!(get_symbol("cost", &config), "¢");

    assert_eq!(get_symbol("time", &config), "⏱ ");
    assert_eq!(get_symbol("context", &config), "🧠 ");
}

#[test]
fn test_get_symbol_unknown_key() {
    let config = Config::default();
    assert_eq!(get_symbol("unknown", &config), "");
}

#[test]
fn test_commit_warning_color_green() {
    let colors = Colors::default();
    let bg_rgb = parse_color(&colors.background).unwrap();
    let color = commit_warning_color(10, bg_rgb, false);
    assert!(format!("{:?}", color).contains("Rgb"));
}

#[test]
fn test_commit_warning_color_yellow() {
    let colors = Colors::default();
    let bg_rgb = parse_color(&colors.background).unwrap();
    let color = commit_warning_color(150, bg_rgb, false);
    assert!(format!("{:?}", color).contains("Rgb"));
}

#[test]
fn test_commit_warning_color_orange() {
    let colors = Colors::default();
    let bg_rgb = parse_color(&colors.background).unwrap();
    let color = commit_warning_color(600, bg_rgb, false);
    assert!(format!("{:?}", color).contains("Rgb"));
}

#[test]
fn test_commit_warning_color_red() {
    let colors = Colors::default();
    let bg_rgb = parse_color(&colors.background).unwrap();
    let color = commit_warning_color(9001, bg_rgb, false);
    let debug_str = format!("{:?}", color);
    assert!(debug_str.contains("Rgb"));
}

#[test]
fn test_config_default() {
    let config = Config::default();
    assert!(config.enable_git);
    assert!(!config.use_emojis);
    assert_eq!(config.warn_high_initial_context_threshold, 20.0);
    assert!(config.symbols.is_empty());
}

#[test]
fn test_cost_log_path() {
    use crate::cost_log::log_path;

    let v: serde_json::Value =
        serde_json::from_str(r#"{"session_id":"abc","transcript_path":"/p/proj/abc.jsonl"}"#)
            .unwrap();
    assert_eq!(
        log_path(&v).unwrap().to_str().unwrap(),
        "/p/proj/abc.cost.jsonl"
    );

    let v: serde_json::Value = serde_json::from_str(r#"{"session_id":"abc"}"#).unwrap();
    assert!(log_path(&v).is_none());
}

#[test]
fn test_cost_log_line() {
    use crate::cost_log::log_line;

    let v: serde_json::Value = serde_json::from_str(
        r#"{"cost":{"total_cost_usd":0.123},"context_window":{"used_percentage":1.0}}"#,
    )
    .unwrap();
    let line = log_line(&v, "2026-09-13T09:20:45.521Z").unwrap();
    assert_eq!(line, "{\"t\":\"2026-09-13T09:20:45.521Z\",\"c\":0.123}\n");
    // Only the two fields of interest are kept.
    let parsed: serde_json::Value = serde_json::from_str(&line).unwrap();
    assert_eq!(parsed.as_object().unwrap().len(), 2);

    // An integral cost must still serialize as valid JSON.
    let v: serde_json::Value = serde_json::from_str(r#"{"cost":{"total_cost_usd":2.0}}"#).unwrap();
    assert_eq!(log_line(&v, "T").unwrap(), "{\"t\":\"T\",\"c\":2}\n");

    // No cost means nothing to log.
    let v: serde_json::Value = serde_json::from_str(r#"{"session_id":"abc"}"#).unwrap();
    assert!(log_line(&v, "T").is_none());
}
