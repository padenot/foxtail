# Release a new version: just release 0.1.9
# Requires crates.io credentials (CARGO_REGISTRY_TOKEN or 'cargo login').
# Binaries are built and attached to the GitHub release by CI on tag push.
release version:
    #!/usr/bin/env bash
    set -euo pipefail

    # Fail before the irreversible commit/tag/push if we can't publish afterwards.
    if [ -z "${CARGO_REGISTRY_TOKEN:-}" ] && ! grep -q token "${CARGO_HOME:-$HOME/.cargo}/credentials.toml" 2>/dev/null; then
        echo "error: no crates.io credentials (set CARGO_REGISTRY_TOKEN or run 'cargo login')" >&2
        echo "       refusing to start a release that can't be published" >&2
        exit 1
    fi

    sed -i '' "s/^version = \".*\"/version = \"{{version}}\"/" Cargo.toml

    cargo fmt
    cargo clippy --all-targets --all-features -- -D warnings
    cargo build --release

    git add -A
    git commit -m "Bump version to {{version}}"
    git tag -a "v{{version}}" -m "Release v{{version}}"
    git push origin main
    git push origin "v{{version}}"

    cargo publish

    echo "CI will build binaries and attach them to the v{{version}} GitHub release."
