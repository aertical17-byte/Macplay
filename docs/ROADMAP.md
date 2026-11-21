# MacPlay roadmap

## Phase 0 — foundation (current)
- [x] SwiftUI macOS shell and Rust workspace
- [x] Conservative compatibility and optimizer models
- [x] CI for Rust tests and Swift build
- [ ] Run CI on GitHub and resolve platform/compiler issues

## Phase 1 — real system inventory
- [ ] Detect Intel vs Apple Silicon, macOS version, memory and CPU count
- [ ] Persist a local game library with stable IDs
- [ ] Import .exe metadata and hashes without executing files
- [ ] Diagnostics export with user consent

## Phase 2 — runtime foundation
- [ ] Evaluate Wine distributions, architecture support, licenses and redistribution
- [ ] Install runtimes with checksum/signature verification
- [ ] Per-game prefixes and isolated writable directories
- [ ] Explicit user approval before launching executable content

## Phase 3 — compatibility and Steam
- [ ] Evidence-backed compatibility records and test environment metadata
- [ ] Steam client integration without collecting credentials
- [ ] Document DRM, launcher and anti-cheat limitations

## Phase 4 — launcher
- [ ] Game cards, install/import flow, runtime selection and launch/stop controls
- [ ] Logs, crash reports and per-game settings
- [ ] First verified playable game on Intel and Apple Silicon separately

## Phase 5 — optimizer
- [ ] Observe memory pressure, thermal state and frame-time metrics
- [ ] Reversible per-game graphics recommendations
- [ ] Benchmarks before/after; never promise all games run on 8 GB

A phase is complete only when implementation, tests and documented evidence exist.
