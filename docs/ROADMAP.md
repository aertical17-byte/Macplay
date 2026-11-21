# MacPlay roadmap

## Foundation
- [x] SwiftUI macOS app shell
- [x] Rust workspace and safety-first domain crates
- [x] Persistent local game-library UI
- [x] Hardware inventory in the macOS UI
- [x] Runtime launch-plan validation (no execution yet)
- [ ] Green CI on supported macOS runners

## Runtime and execution
- [ ] Choose Wine distribution after architecture, license and redistribution review
- [ ] Runtime download with signed manifest, hashes and atomic install
- [ ] Per-game Wine prefixes, isolated writable directories and cleanup
- [ ] User-consented process supervisor with logs and graceful stop
- [ ] First verified game on Intel and Apple Silicon separately

## Compatibility and Steam
- [ ] Versioned evidence-backed compatibility database
- [ ] Steam client detection/integration; credentials stay in official client
- [ ] Document DRM, third-party launcher and anti-cheat restrictions

## Optimizer
- [ ] Observe memory pressure, thermal state and frame-time metrics
- [ ] Reversible per-game graphics recommendations
- [ ] Benchmarks before/after; realistic 8 GB warnings

## Launcher completion (last milestone)
- [ ] Artwork, search, filters, install/import and game detail
- [ ] Runtime setup wizard and per-game settings
- [ ] Launch, stop, logs, diagnostics and update UX
- [ ] Signed/notarized universal macOS distribution

A milestone is complete only with implementation, tests and documented evidence.
