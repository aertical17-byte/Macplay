# MacPlay roadmap

## Foundation — current
- [x] SwiftUI macOS shell and persistent local executable library
- [x] Physical Mac hardware inventory (including Apple Silicon under Rosetta)
- [x] Rust workspace and safety-first domain crates
- [x] Read-only Steam KeyValues manifest parser
- [x] Runtime launch-plan validation (no process execution)
- [x] Universal app bundle assembly script (unsigned)
- [ ] Green CI on supported macOS runners

## System integration
- [ ] Connect Steam manifest discovery to the UI, with detected/installed/tested states
- [ ] Persist Steam and executable entries in one library model
- [ ] Diagnostics export with user consent and path redaction

## Runtime and execution
- [ ] Choose Wine distribution after architecture, license and redistribution review
- [ ] Runtime download with signed manifest, hashes and atomic install
- [ ] Per-game Wine prefixes, isolated writable directories and cleanup
- [ ] User-consented process supervisor with logs and graceful stop
- [ ] First verified game on Intel and Apple Silicon separately

## Compatibility and graphics
- [ ] Versioned evidence-backed compatibility database
- [ ] Validate graphics backend selection per game and macOS version
- [ ] Document DRM, third-party launcher and anti-cheat restrictions

## Optimizer
- [ ] Observe memory pressure, thermal state and frame-time metrics
- [ ] Reversible per-game graphics recommendations
- [ ] Benchmarks before/after; realistic 8 GB warnings

## Launcher completion — final milestone
- [ ] Artwork, search, filters, install/import and game detail
- [ ] Runtime setup wizard and per-game settings
- [ ] Launch, stop, logs, diagnostics and update UX
- [ ] Signed/notarized universal macOS distribution

A milestone is complete only with implementation, tests and documented evidence.
