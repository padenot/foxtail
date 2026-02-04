use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;

#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(untagged)]
pub enum ColorSpec {
    Hsl(String),  // "hsl(120, 60, 30)"
    Rgb(Vec<u8>), // [255, 140, 0]
    Hex(String),  // "#ff8c00" or "#f80"
}

#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(default)]
pub struct Colors {
    pub background: ColorSpec,
    pub model: ColorSpec,
    pub cwd: ColorSpec,
    pub time: ColorSpec,
    pub git_clean: ColorSpec,
    pub delta: ColorSpec,
    pub cost: ColorSpec,
    pub cache: ColorSpec,
    pub separator: ColorSpec,
}

impl Default for Colors {
    fn default() -> Self {
        Colors {
            background: ColorSpec::Rgb(vec![240, 160, 80]),
            model: ColorSpec::Rgb(vec![0, 0, 0]),
            cwd: ColorSpec::Rgb(vec![0, 0, 0]),
            time: ColorSpec::Rgb(vec![0, 0, 0]),
            git_clean: ColorSpec::Rgb(vec![0, 0, 0]),
            delta: ColorSpec::Rgb(vec![0, 0, 0]),
            cost: ColorSpec::Rgb(vec![0, 0, 0]),
            cache: ColorSpec::Rgb(vec![0, 0, 0]),
            separator: ColorSpec::Rgb(vec![220, 100, 0]),
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
#[allow(dead_code)]
pub struct Visual {
    #[serde(default)]
    pub head: Option<String>,
    #[serde(default)]
    pub tail: Option<String>,
    #[serde(default)]
    pub separator: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Config {
    #[serde(default = "default_enable_git")]
    pub enable_git: bool,
    #[serde(default = "default_use_emojis")]
    pub use_emojis: bool,
    #[serde(default)]
    pub enable_background: bool,
    #[serde(default = "default_warn_threshold")]
    pub warn_high_initial_context_threshold: f64,
    #[serde(default)]
    pub symbols: HashMap<String, String>,
    #[serde(default = "default_threshold_green")]
    #[allow(dead_code)]
    pub threshold_green: u64,
    #[serde(default = "default_threshold_yellow")]
    #[allow(dead_code)]
    pub threshold_yellow: u64,
    #[serde(default = "default_threshold_orange")]
    #[allow(dead_code)]
    pub threshold_orange: u64,
    #[serde(default)]
    pub colors: Option<Colors>,
    #[serde(default)]
    #[allow(dead_code)]
    pub visual: Option<Visual>,
    #[serde(default = "default_format")]
    pub format: String,
    #[serde(default)]
    pub light_background: Option<bool>,
    #[serde(default)]
    pub warning_message: Option<String>,
}

pub fn default_warn_threshold() -> f64 { 20.0 }
pub fn default_enable_git() -> bool { true }
pub fn default_use_emojis() -> bool { false }
pub fn default_threshold_green() -> u64 { 100 }
pub fn default_threshold_yellow() -> u64 { 500 }
pub fn default_threshold_orange() -> u64 { 1000 }

pub fn default_format() -> String {
    "{head} | {model} | {cwdcompact} | {duration} | {ctx} | {gitdelta} | {claudedelta} | {cost} | {cache} | {tail}".to_string()
}

impl Default for Config {
    fn default() -> Self {
        Config {
            enable_git: true,
            use_emojis: false,
            enable_background: false,
            warn_high_initial_context_threshold: default_warn_threshold(),
            symbols: HashMap::new(),
            threshold_green: 100,
            threshold_yellow: 500,
            threshold_orange: 1000,
            colors: None,
            visual: None,
            format: default_format(),
            light_background: None,
            warning_message: None,
        }
    }
}

pub fn get_symbol(key: &str, config: &Config) -> String {
    if let Some(custom) = config.symbols.get(key) { return custom.clone(); }
    let (emoji, text) = match key {
        "cwd" => ("📁 ", "cwd:"),
        "time" => ("⏱ ", ""),
        "context" => ("🧠 ", "ctx:"),
        "git_warning" => (" ⚠", ""),
        "git_clean" => ("✓ ", ""),
        "delta" => ("Σ ", "Δ "),
        "cost" => ("💰 $", "$"),
        "cache" => ("🗄 ", "cache:"),
        _ => ("", ""),
    };
    if config.use_emojis { emoji.to_string() } else { text.to_string() }
}

pub fn load_config_with_path_override(override_path: Option<&str>) -> (Config, Option<String>) {
    if let Some(path) = override_path {
        if let Ok(contents) = fs::read_to_string(path) {
            if let Ok(config) = toml::from_str(&contents) {
                return (config, Some(path.to_string()));
            }
        }
    }

    let config_paths = [
        dirs::home_dir().map(|mut p| { p.push(".claude/statusline.toml"); p }),
        dirs::home_dir().map(|mut p| { p.push(".mozbuild/foxtail.toml"); p }),
        dirs::config_dir().map(|mut p| { p.push("foxtail/config.toml"); p }),
    ];

    for path in config_paths.iter().flatten() {
        if let Ok(contents) = fs::read_to_string(path) {
            if let Ok(config) = toml::from_str(&contents) {
                return (config, Some(path.to_string_lossy().to_string()));
            }
        }
    }
    (Config::default(), None)
}

pub fn load_config_with_path() -> (Config, Option<String>) {
    load_config_with_path_override(None)
}

pub fn dump_config() {
    let (config, config_path) = load_config_with_path();
    println!("Configuration:\n==============\n");
    if let Some(path) = config_path.as_ref() { println!("Loaded from: {}\n", path); } else {
        println!("No config file found. Using defaults.\n");
        println!("Checked locations:");
        if let Some(mut p) = dirs::home_dir() { p.push(".claude/statusline.toml"); println!("  - {}", p.display()); }
        if let Some(mut p) = dirs::home_dir() { p.push(".mozbuild/foxtail.toml"); println!("  - {}", p.display()); }
        if let Some(mut p) = dirs::config_dir() { p.push("foxtail/config.toml"); println!("  - {}", p.display()); }
        println!();
    }
    println!("Current configuration:\n---------------------");
    match toml::to_string(&config) { Ok(t) => println!("{}", t), Err(e) => eprintln!("Error serializing config: {}", e) }
    println!("\nComplete example configuration with all options:\n-----------------------------------------------");
    let mut example_symbols = HashMap::new();
    example_symbols.insert("cwd".to_string(), "📁 ".to_string());
    example_symbols.insert("time".to_string(), "⏱ ".to_string());
    let example_config = Config {
        enable_git: true,
        use_emojis: false,
        enable_background: false,
        warn_high_initial_context_threshold: 20.0,
        symbols: example_symbols,
        threshold_green: 100,
        threshold_yellow: 500,
        threshold_orange: 1000,
        colors: Some(Colors::default()),
        visual: Some(Visual { head: Some("🦊".to_string()), tail: Some("🦊".to_string()), separator: Some(" | ".to_string()) }),
        format: default_format(),
        light_background: Some(true),
        warning_message: Some("⚠ Context warning: {used_pct}% used in first minute ({used_k}k/{total_k}k) ⚠".to_string()),
    };
    match toml::to_string(&example_config) {
        Ok(t) => {
            println!("{}", t);
            println!("\nNote: Colors can be specified as:\n  - RGB arrays: [255, 140, 0]\n  - Hex strings: \"#ff8c00\" or \"#f80\"\n  - HSL strings: \"hsl(30, 100, 50)\"\n\nFormat placeholders:\n  {{head}} {{tail}} {{model}} {{cwd}} {{cwdcompact}} {{duration}}\n  {{ctx}} {{gitdelta}} {{claudedelta}} {{cost}} {{cache}}");
        }
        Err(e) => eprintln!("Error serializing example: {}", e),
    }
}
