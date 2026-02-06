use crate::config::Config;
use chrono::{DateTime, Duration, Utc};
use std::path::Path;
use std::process::Command;

pub fn format_duration_ms(duration_ms: u64) -> String {
    let secs = duration_ms / 1000;
    if secs < 60 {
        format!("{}s", secs)
    } else if secs < 3600 {
        format!("{}m", secs / 60)
    } else if secs < 86400 {
        format!("{}h{}m", secs / 3600, (secs % 3600) / 60)
    } else {
        format!("{}d{}h", secs / 86400, (secs % 86400) / 3600)
    }
}

pub fn format_duration(duration: Duration) -> String {
    let secs = duration.num_seconds().max(0);
    if secs < 60 {
        format!("{}s", secs)
    } else if secs < 3600 {
        format!("{}m", secs / 60)
    } else if secs < 86400 {
        format!("{}h{}m", secs / 3600, (secs % 3600) / 60)
    } else {
        format!("{}d{}h", secs / 86400, (secs % 86400) / 3600)
    }
}

pub fn get_git_info(cwd: &str, config: &Config) -> Option<(Duration, u64, u64)> {
    if !config.enable_git {
        return None;
    }
    let jj_path = Path::new(cwd).join(".jj");
    if jj_path.exists() {
        return None;
    }

    Command::new("git")
        .args(["-C", cwd, "rev-parse", "--git-dir"])
        .output()
        .ok()?
        .status
        .success()
        .then_some(())?;

    let output = Command::new("git")
        .args(["-C", cwd, "log", "-1", "--format=%ct"])
        .output()
        .ok()?;
    if !output.status.success() {
        return None;
    }
    let timestamp_str = String::from_utf8_lossy(&output.stdout);
    let timestamp: i64 = timestamp_str.trim().parse().ok()?;
    let last_commit_time = DateTime::from_timestamp(timestamp, 0)?;
    let time_since_commit = Utc::now().signed_duration_since(last_commit_time);

    let status_output = Command::new("git")
        .args(["-C", cwd, "status", "--porcelain", "-uno"])
        .output()
        .ok()?;
    let (mut added, mut removed) = (0u64, 0u64);
    if status_output.status.success() && !status_output.stdout.is_empty() {
        let diff_output = Command::new("git")
            .args(["-C", cwd, "diff", "--numstat"])
            .output()
            .ok()?;
        if !diff_output.status.success() {
            return Some((time_since_commit, 0, 0));
        }
        let diff_str = String::from_utf8_lossy(&diff_output.stdout);
        for line in diff_str.lines() {
            let parts: Vec<&str> = line.split_whitespace().collect();
            if parts.len() >= 2 {
                if let Ok(a) = parts[0].parse::<u64>() {
                    added += a;
                }
                if let Ok(r) = parts[1].parse::<u64>() {
                    removed += r;
                }
            }
        }
    }
    Some((time_since_commit, added, removed))
}

pub fn commit_warning_color(
    diff_lines: u64,
    bg_rgb: (u8, u8, u8),
    is_light_bg: bool,
) -> nu_ansi_term::Color {
    let max_lines = 1000.0;
    let percentage = (diff_lines as f64 / max_lines * 100.0).min(100.0);
    crate::colors::gradient_green_to_red(percentage, bg_rgb, is_light_bg)
}
