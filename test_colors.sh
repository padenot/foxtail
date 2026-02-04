#!/bin/bash

# Build the project first
cargo build --release 2>/dev/null

echo "=== Testing all color variations ==="
echo ""

# Context green (>50% remaining)
echo "1. Context GREEN (60% remaining):"
echo '{
  "cwd": "/home/user/project",
  "model": {"display_name": "Sonnet 4.5"},
  "workspace": {"current_dir": "/home/user/my-project"},
  "cost": {
    "total_cost_usd": 0.123,
    "total_duration_ms": 45000,
    "total_lines_added": 50,
    "total_lines_removed": 30
  },
  "context_window": {
    "context_window_size": 200000,
    "used_percentage": 40.0,
    "remaining_percentage": 60.0,
    "current_usage": {
      "input_tokens": 50000,
      "output_tokens": 30000,
      "cache_creation_input_tokens": 10000,
      "cache_read_input_tokens": 20000
    }
  }
}' | ./target/release/foxtail
echo ""

# Context yellow (20-50% remaining)
echo "2. Context YELLOW (35% remaining):"
echo '{
  "cwd": "/home/user/project",
  "model": {"display_name": "Sonnet 4.5"},
  "workspace": {"current_dir": "/home/user/my-project"},
  "cost": {
    "total_cost_usd": 0.456,
    "total_duration_ms": 120000,
    "total_lines_added": 150,
    "total_lines_removed": 80
  },
  "context_window": {
    "context_window_size": 200000,
    "used_percentage": 65.0,
    "remaining_percentage": 35.0,
    "current_usage": null
  }
}' | ./target/release/foxtail
echo ""

# Context red (<20% remaining)
echo "3. Context RED (10% remaining):"
echo '{
  "cwd": "/home/user/project",
  "model": {"display_name": "Sonnet 4.5"},
  "workspace": {"current_dir": "/home/user/my-project"},
  "cost": {
    "total_cost_usd": 0.789,
    "total_duration_ms": 180000,
    "total_lines_added": 300,
    "total_lines_removed": 200
  },
  "context_window": {
    "context_window_size": 200000,
    "used_percentage": 90.0,
    "remaining_percentage": 10.0,
    "current_usage": null
  }
}' | ./target/release/foxtail
echo ""

# Git clean
echo "4. Git CLEAN (no changes):"
echo '{
  "cwd": "'$(pwd)'",
  "model": {"display_name": "Sonnet 4.5"},
  "workspace": {"current_dir": "'$(pwd)'"},
  "cost": {
    "total_cost_usd": 0.050,
    "total_duration_ms": 30000,
    "total_lines_added": 0,
    "total_lines_removed": 0
  },
  "context_window": {
    "context_window_size": 200000,
    "used_percentage": 25.0,
    "remaining_percentage": 75.0,
    "current_usage": null
  }
}' | ./target/release/foxtail
echo ""

# Git warning green (small changes)
echo "5. Git warning GREEN (50 lines changed):"
mkdir -p /tmp/test_git_repo && cd /tmp/test_git_repo
git init -q 2>/dev/null
echo "test" > test.txt
git add test.txt
git commit -q -m "initial" 2>/dev/null
# Create small diff
for i in {1..25}; do echo "line $i" >> test.txt; done
cd - >/dev/null

echo '{
  "cwd": "/tmp/test_git_repo",
  "model": {"display_name": "Sonnet 4.5"},
  "workspace": {"current_dir": "/tmp/test_git_repo"},
  "cost": {
    "total_cost_usd": 0.100,
    "total_duration_ms": 60000,
    "total_lines_added": 100,
    "total_lines_removed": 50
  },
  "context_window": {
    "context_window_size": 200000,
    "used_percentage": 30.0,
    "remaining_percentage": 70.0,
    "current_usage": null
  }
}' | ./target/release/foxtail
echo ""

# Git warning yellow (medium changes)
cd /tmp/test_git_repo
# Create medium diff
for i in {1..250}; do echo "line $i" >> test.txt; done
cd - >/dev/null

echo "6. Git warning YELLOW (275 lines changed):"
echo '{
  "cwd": "/tmp/test_git_repo",
  "model": {"display_name": "Sonnet 4.5"},
  "workspace": {"current_dir": "/tmp/test_git_repo"},
  "cost": {
    "total_cost_usd": 0.200,
    "total_duration_ms": 90000,
    "total_lines_added": 200,
    "total_lines_removed": 100
  },
  "context_window": {
    "context_window_size": 200000,
    "used_percentage": 45.0,
    "remaining_percentage": 55.0,
    "current_usage": null
  }
}' | ./target/release/foxtail
echo ""

# Git warning orange (large changes)
cd /tmp/test_git_repo
# Create large diff
for i in {1..500}; do echo "line $i" >> test.txt; done
cd - >/dev/null

echo "7. Git warning ORANGE (775 lines changed):"
echo '{
  "cwd": "/tmp/test_git_repo",
  "model": {"display_name": "Sonnet 4.5"},
  "workspace": {"current_dir": "/tmp/test_git_repo"},
  "cost": {
    "total_cost_usd": 0.350,
    "total_duration_ms": 150000,
    "total_lines_added": 400,
    "total_lines_removed": 150
  },
  "context_window": {
    "context_window_size": 200000,
    "used_percentage": 50.0,
    "remaining_percentage": 50.0,
    "current_usage": null
  }
}' | ./target/release/foxtail
echo ""

# Git warning red (huge changes)
cd /tmp/test_git_repo
# Create huge diff
for i in {1..5000}; do echo "line $i" >> test.txt; done
cd - >/dev/null

echo "8. Git warning RED (5775 lines changed):"
echo '{
  "cwd": "/tmp/test_git_repo",
  "model": {"display_name": "Sonnet 4.5"},
  "workspace": {"current_dir": "/tmp/test_git_repo"},
  "cost": {
    "total_cost_usd": 0.500,
    "total_duration_ms": 200000,
    "total_lines_added": 800,
    "total_lines_removed": 300
  },
  "context_window": {
    "context_window_size": 200000,
    "used_percentage": 85.0,
    "remaining_percentage": 15.0,
    "current_usage": null
  }
}' | ./target/release/foxtail
echo ""

# Cleanup
rm -rf /tmp/test_git_repo

echo "=== All color variations displayed ==="
