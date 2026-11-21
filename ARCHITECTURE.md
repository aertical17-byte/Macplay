# Architecture

SwiftUI is the native macOS control plane; Rust owns testable domain logic. Keep UI thin and select runtime/backend per game and host.

## Host paths
- Intel: x86_64 macOS app and separately validated x86_64 Windows compatibility stack.
- Apple silicon: arm64 app plus a validated Windows x86/x64 translation path. Rosetta is not that path.

Potential building blocks include Wine, DXVK, VKD3D-Proton, and Apple's Game Porting Toolkit/D3DMetal. Review support and licenses before packaging. Steam authentication stays in the official client. Anti-cheat and DRM are title-specific unknowns.

## Core boundaries
- hardware: read-only host facts
- compatibility: explainable status backed by evidence
- runtime manager: verified, isolated runtime lifecycle (not implemented yet)
- optimizer: reversible recommendations; user consent required

Game files are untrusted. No silent execution, privileged installer, or unverified downloads.
