---
name: release
description: Tag a new version, make a new release
---

# Release Process

Run:

```
CARGO_REGISTRY_TOKEN=<token> just release X.Y.Z
```

This bumps the version, runs fmt/clippy/build, commits, tags, pushes, and
publishes to crates.io.

Pushing the tag triggers `.github/workflows/release.yml`, which builds binaries
for all supported targets and attaches them to the GitHub release. That release
must exist with those assets or `cargo binstall foxtail` will fail, so check
`gh release view vX.Y.Z` afterwards.
