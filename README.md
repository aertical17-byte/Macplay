# MacPlay

**A guided way to explore Windows game compatibility on Intel and Apple silicon Macs.**

> Early experimental scaffold — not yet a working game launcher. No compatibility, performance, or 8 GB RAM guarantee is implied.

MacPlay is planned as a hybrid platform: orchestrate established compatibility technologies first, then develop proprietary components only where technically justified and legally supportable.

## Goals
- One library for Steam titles and user-selected Windows executables.
- Separate, hardware-aware paths for Intel and Apple silicon.
- Isolated per-game environments, verified runtime updates, diagnostics, and rollback.
- A measured Smart Optimizer with reversible, per-game recommendations.
- Compatibility claims backed by reproducible test evidence.

## Repository
- `apps/macos`: native SwiftUI shell.
- `core`: Rust domain crates.
- `docs`: architecture, compatibility, optimizer, security, and roadmap.

## Build
Requirements: macOS 14+, Xcode 16+, Rust stable.

```sh
cd core && cargo test --workspace
cd ../apps/macos && swift build
```

## Important technical notes
Rosetta translates macOS Intel applications on Apple silicon; it is not a Windows game compatibility layer. Windows x86/x64 titles require a validated Wine/runtime and graphics translation path. Potential upstream components (Wine, DXVK, VKD3D-Proton, Apple's Game Porting Toolkit/D3DMetal) require per-component architecture, support, and license review before redistribution.

Steam credentials must stay with the official Steam client. Importing a library does not guarantee DRM, launcher, or anti-cheat compatibility. On 8 GB Macs, some games will remain unsuitable; MacPlay will not disable swap or claim to create memory.

See [Architecture](ARCHITECTURE.md), [Roadmap](docs/ROADMAP.md), and [Security](SECURITY.md).
