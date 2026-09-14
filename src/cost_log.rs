use std::fs::OpenOptions;
use std::io::Write;
use std::path::{Path, PathBuf};

/// Append one line per received status payload to a cost log file sitting next
/// to the session transcript:
///
/// ```text
/// {"t":"2026-09-13T09:20:45.521Z","c":0.123}
/// ```
///
/// `t` is the time the payload was received, in the same format as the
/// timestamps in Claude Code's session .jsonl logs so both streams can be
/// correlated directly. `c` is `cost.total_cost_usd` as given to the status
/// line. Everything else in the payload is static, derivable, or already
/// reproduced by the consumer, so it is dropped.
///
/// Every sample is written, including ones identical to the previous line: a
/// flat ledger and a gap in sampling must stay distinguishable.
///
/// Failures are non-fatal: the status line must keep working even if logging
/// fails. They go to stderr, which Claude Code discards when running a status
/// line, so they are only visible when invoking foxtail from a shell.
pub fn log_cost(raw: &str) {
    let value: serde_json::Value = match serde_json::from_str(raw) {
        Ok(v) => v,
        Err(e) => {
            eprintln!("foxtail: --log-cost: invalid JSON: {}", e);
            return;
        }
    };

    let path = match log_path(&value) {
        Some(p) => p,
        None => {
            eprintln!("foxtail: --log-cost: no transcript_path/session_id in input");
            return;
        }
    };

    let timestamp = chrono::Utc::now()
        .format("%Y-%m-%dT%H:%M:%S%.3fZ")
        .to_string();
    let line = match log_line(&value, &timestamp) {
        Some(l) => l,
        None => {
            eprintln!("foxtail: --log-cost: no cost.total_cost_usd in input");
            return;
        }
    };

    let res = OpenOptions::new()
        .create(true)
        .append(true)
        .open(&path)
        .and_then(|mut f| f.write_all(line.as_bytes()));
    if let Err(e) = res {
        eprintln!("foxtail: --log-cost: {}: {}", path.display(), e);
    }
}

/// One `{"t":...,"c":...}` line, newline included, or `None` when the payload
/// carries no cost.
pub(crate) fn log_line(value: &serde_json::Value, timestamp: &str) -> Option<String> {
    let cost = value
        .get("cost")?
        .get("total_cost_usd")?
        .as_f64()
        .filter(|c| c.is_finite())?;
    Some(format!("{{\"t\":\"{}\",\"c\":{}}}\n", timestamp, cost))
}

/// The cost log path, next to the session transcript.
pub(crate) fn log_path(value: &serde_json::Value) -> Option<PathBuf> {
    let dir = Path::new(value.get("transcript_path")?.as_str()?).parent()?;
    let session_id = value.get("session_id")?.as_str()?;
    Some(dir.join(format!("{}.cost.jsonl", session_id)))
}
