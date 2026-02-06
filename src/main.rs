mod cli;
mod colors;
mod config;
mod git;
mod render;
mod types;

use clap::Parser;
use std::io::{self, Read};

// Re-exports for tests and external use
pub use cli::{Args, BackgroundMode, ColorMode};
pub use colors::{adjust_colors_for_background, parse_color};
pub use config::{
    default_format, dump_config, get_symbol, load_config_with_path_override, ColorSpec, Colors,
    Config, Visual,
};
pub use git::{commit_warning_color, format_duration_ms};
pub use render::{abbreviate_path, render_formatted, render_warning};
pub use types::{ContextWindow, Cost, CurrentUsage, Model, StatusInput, Workspace};

#[cfg(test)]
mod tests;

fn main() {
    let args = Args::parse();

    let version_checker =
        moz_cli_version_check::VersionChecker::new("foxtail", env!("CARGO_PKG_VERSION"));
    version_checker.check_async();

    if args.print_config {
        dump_config();
        return;
    }

    // Determine color enablement
    let color_enabled = match args.color {
        ColorMode::Always => true,
        ColorMode::Never => false,
        ColorMode::Auto => {
            if std::env::var_os("NO_COLOR").is_some() {
                false
            } else if let Ok(v) = std::env::var("CLICOLOR_FORCE") {
                v == "1"
            } else if let Ok(v) = std::env::var("CLICOLOR") {
                v != "0"
            } else {
                #[allow(deprecated)]
                {
                    use std::io::IsTerminal;
                    std::io::stdout().is_terminal()
                }
            }
        }
    };

    let config_file_override = args.config_path.clone();

    let mut input = String::new();
    io::stdin()
        .read_to_string(&mut input)
        .expect("Failed to read stdin");

    let data: StatusInput = match serde_json::from_str(&input) {
        Ok(d) => d,
        Err(e) => {
            eprintln!("Failed to parse JSON: {}", e);
            std::process::exit(1);
        }
    };

    let mut config = load_config_with_path_override(config_file_override.as_deref()).0;
    if args.no_git {
        config.enable_git = false;
    }
    let is_light_bg = match args.background {
        BackgroundMode::Light => true,
        BackgroundMode::Dark => false,
        BackgroundMode::Auto => config.light_background.unwrap_or(false),
    };
    let mut colors = colors::get_colors(config.colors.as_ref(), is_light_bg);
    if config.colors.is_some() {
        colors = adjust_colors_for_background(colors, is_light_bg);
    }
    if let Err(e) = colors::validate_colors(&colors) {
        eprintln!("{}", e);
        std::process::exit(2);
    }
    let bg_rgb = match parse_color(&colors.background) {
        Some(rgb) => rgb,
        None => {
            eprintln!("Invalid background color in configuration");
            std::process::exit(2);
        }
    };

    if data.cost.total_duration_ms / 1000 < 60
        && data.context_window.used_percentage > config.warn_high_initial_context_threshold
    {
        println!(
            "{}",
            render_warning(&data, &config, bg_rgb, is_light_bg, color_enabled)
        );
    } else {
        println!(
            "{}",
            render_formatted(
                &config.format,
                &data,
                &config,
                &colors,
                bg_rgb,
                is_light_bg,
                color_enabled,
            ),
        );
    }

    version_checker.print_warning();
}
