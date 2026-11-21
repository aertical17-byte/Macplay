# Security policy

Treat selected executables, archives, and runtime payloads as untrusted.

Before runtime execution ships: verify signed manifests and hashes; defend archive extraction against traversal and symlinks; isolate per-game writable data; pass process arguments without shell interpolation; request explicit consent for launch/network access; redact secrets and personal paths from diagnostics; sign and notarize releases.

Never collect Steam passwords or session cookies. No privileged helper or system-wide modification in the MVP. Until a private security contact is configured, use GitHub private vulnerability reporting if enabled.
