# MacPlay

**A guided way to explore Windows game compatibility on Intel and Apple silicon Macs.**

> Early experimental scaffold — not yet a working Windows game launcher. No compatibility, performance, or 8 GB RAM guarantee is implied.

MacPlay is planned as a hybrid platform: orchestrate established compatibility technologies first, then develop proprietary components only where technically justified and legally supportable.

## Current implementation
- Native SwiftUI macOS shell with persistent local executable library.
- Physical Mac architecture, logical CPU count, memory capacity and macOS version display.
- Rust workspace for hardware facts, compatibility evidence, game-library validation, runtime launch planning and conservative optimizer recommendations.
- Read-only Steam KeyValues parser for local app manifests and library folder metadata (no credentials or Steam session access).
- Script to assemble a local universal arm64 + x86_64 .app bundle.
- GitHub Actions workflow for Rust tests and macOS app packaging.

The current UI imports and removes executable references. It does not yet connect the Steam parser to the interface, install Wine, execute Windows files, or claim games are playable.

## Build
Requirements: macOS 14+, Xcode 16+, Rust stable.

```sh
cd core && cargo test --workspace
cd ../apps/macos && swift build
```

Create a local universal .app bundle:

```sh
./scripts/package-macos-app.sh
```

The package script does not sign or notarize the app. Configure Developer ID signing and notarization before distributing it.

## Important technical notes
Rosetta translates macOS Intel applications on Apple silicon; it is not a Windows game compatibility layer. Windows x86/x64 titles require a validated Wine/runtime and graphics translation path. Potential upstream components (Wine, DXVK, VKD3D-Proton, Apple's Game Porting Toolkit/D3DMetal) require per-component architecture, support, and license review before redistribution.

Steam credentials must stay with the official Steam client. Importing a library does not guarantee DRM, launcher, or anti-cheat compatibility. On 8 GB Macs, some games will remain unsuitable; MacPlay will not disable swap or claim to create memory.

See [Architecture](ARCHITECTURE.md), [Roadmap](docs/ROADMAP.md), [Steam discovery](docs/STEAM-DISCOVERY.md), [Runtime integration](docs/RUNTIME-INTEGRATION.md), and [Security](SECURITY.md).
