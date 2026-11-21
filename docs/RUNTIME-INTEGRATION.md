# Runtime integration decision record

MacPlay does not bundle or download a Windows compatibility runtime yet. Before selecting a runtime, the project must review:
- license terms for the exact distribution and every bundled component;
- Intel x86_64 and Apple Silicon support separately;
- code-signing, notarization and redistribution obligations;
- graphics backend support per macOS version and GPU;
- update provenance, checksums/signatures and rollback behavior.

A runtime installation is valid only inside MacPlay's managed runtime directory. Runtime identifiers are data, not shell fragments. Launch plans use a program path and a vector of arguments; never construct a shell command string.

No process execution is enabled by this milestone. The future supervisor must require user intent, use an isolated per-game prefix, preserve logs, and support graceful termination.
