use clap::{Parser, ValueEnum};

#[derive(Debug, Clone, Copy, ValueEnum)]
pub enum ColorMode {
    Auto,
    Always,
    Never,
}

#[derive(Debug, Clone, Copy, ValueEnum)]
pub enum BackgroundMode {
    Auto,
    Light,
    Dark,
}

#[derive(Debug, Parser)]
#[command(name = "foxtail", version, about = "Status line renderer for foxtail", disable_help_subcommand = true)]
pub struct Args {
    #[arg(long = "print-config")]
    pub print_config: bool,

    #[arg(long = "detect-background")]
    pub detect_background: bool,

    #[arg(long = "config")]
    pub config_path: Option<String>,

    #[arg(long = "color", value_enum, default_value_t = ColorMode::Auto)]
    pub color: ColorMode,

    #[arg(long = "background", value_enum, default_value_t = BackgroundMode::Auto)]
    pub background: BackgroundMode,

    #[arg(long = "no-git")]
    pub no_git: bool,

    // ASCII flag removed: always render the fun fox art when possible
}
