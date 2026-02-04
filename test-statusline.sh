#!/bin/bash
# Test script to demonstrate the statusline output

cat <<'EOF' | ./target/release/cc-statusline
{
  "hook_event_name": "Status",
  "session_id": "test-session-123",
  "transcript_path": "/tmp/transcript.json",
  "cwd": "/home/padenot/src/repositories/cc-statusline",
  "model": {
    "id": "claude-sonnet-4-5",
    "display_name": "Sonnet"
  },
  "workspace": {
    "current_dir": "/home/padenot/src/repositories/cc-statusline",
    "project_dir": "/home/padenot/src/repositories/cc-statusline"
  },
  "version": "1.0.80",
  "output_style": {
    "name": "default"
  },
  "cost": {
    "total_cost_usd": 0.123,
    "total_duration_ms": 45000,
    "total_api_duration_ms": 2300,
    "total_lines_added": 256,
    "total_lines_removed": 43
  },
  "context_window": {
    "total_input_tokens": 25234,
    "total_output_tokens": 8521,
    "context_window_size": 200000,
    "used_percentage": 16.8775,
    "remaining_percentage": 83.1225,
    "current_usage": {
      "input_tokens": 15500,
      "output_tokens": 2500,
      "cache_creation_input_tokens": 12000,
      "cache_read_input_tokens": 8000
    }
  }
}
EOF
