# version-check

A lightweight, opt-in version checking library for Mozilla CLI tools.

## Features

- **Opt-in only**: Only checks when `MOZTOOLS_UPDATE_CHECK=1` is set
- **Non-blocking**: Runs in background thread, never delays program startup
- **Cached**: Checks at most once per 24 hours per tool
- **Shared cache**: All tools share `~/.mozbuild/tool-versions.json`
- **Silent failures**: Network errors don't affect program operation
- **Thread-safe**: Safe for concurrent access

## Usage

Add to your `Cargo.toml`:

```toml
[dependencies]
version-check = { path = "../foxtail/version-check" }
```

In your `main.rs`:

```rust
fn main() -> Result<()> {
    let version_checker = version_check::VersionChecker::new(
        "your-tool-name",
        env!("CARGO_PKG_VERSION"),
    );
    version_checker.check_async();

    let result = run();

    version_checker.print_warning();

    result
}

fn run() -> Result<()> {
    // Your actual program logic here
    Ok(())
}
```

## How It Works

1. At program startup, if `MOZTOOLS_UPDATE_CHECK=1` is set, spawn a background thread
2. The thread checks the cache file (`~/.mozbuild/tool-versions.json`)
3. If the cache is recent (< 24 hours), use cached data
4. Otherwise, query crates.io API: `https://crates.io/api/v1/crates/<name>`
5. Update the cache with the latest version info
6. At program exit, print a warning if a newer version is available

## Warning Format

When a newer version is available, users see:

```
Note: A newer version of socorro-cli is available (0.2.0 > 0.1.0)
      Run: cargo binstall socorro-cli
```

## Cache Format

The cache file at `~/.mozbuild/tool-versions.json` contains:

```json
{
  "socorro-cli": {
    "last_check": 1234567890,
    "latest": "0.2.0"
  },
  "treeherder-cli": {
    "last_check": 1234567890,
    "latest": "0.1.0"
  }
}
```

## Testing

To enable version checking:

```bash
export MOZTOOLS_UPDATE_CHECK=1
socorro-cli crash --help
```

To disable (default):

```bash
unset MOZTOOLS_UPDATE_CHECK
socorro-cli crash --help
```

## Configuration

- **Cache location**: `~/.mozbuild/tool-versions.json`
- **Cache validity**: 24 hours
- **Network timeout**: 5 seconds
- **User-Agent**: `{tool-name}/version-check`

## Implementation Details

- Uses `reqwest` with blocking client for HTTP requests
- Uses `serde_json` for cache file serialization
- Thread-safe via `Arc<Mutex<Option<String>>>`
- Silently fails on any error (network, I/O, parsing)
- Never blocks program execution
