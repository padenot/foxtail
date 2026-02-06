use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct StatusInput {
    pub cwd: String,
    pub model: Model,
    pub workspace: Workspace,
    pub cost: Cost,
    pub context_window: ContextWindow,
}

#[derive(Debug, Deserialize)]
pub struct Model {
    pub display_name: String,
}

#[derive(Debug, Deserialize)]
pub struct Workspace {
    pub current_dir: String,
}

#[derive(Debug, Deserialize)]
pub struct Cost {
    pub total_cost_usd: f64,
    pub total_duration_ms: u64,
    pub total_lines_added: u32,
    pub total_lines_removed: u32,
}

#[derive(Debug, Deserialize)]
pub struct ContextWindow {
    pub context_window_size: u32,
    pub used_percentage: f64,
    pub remaining_percentage: f64,
    pub current_usage: Option<CurrentUsage>,
}

#[derive(Debug, Deserialize)]
pub struct CurrentUsage {
    #[allow(dead_code)]
    pub input_tokens: u32,
    #[allow(dead_code)]
    pub output_tokens: u32,
    pub cache_creation_input_tokens: u32,
    pub cache_read_input_tokens: u32,
}
