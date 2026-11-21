# Hardware inventory

The macOS shell reads only basic host facts:
- physical CPU family (via `hw.optional.arm64`, so Rosetta process translation does not mislabel the host);
- logical CPU count;
- physical memory capacity;
- macOS version.

These are descriptive facts, not a performance score. They do not predict whether a particular Windows game is playable. Memory pressure, thermal state, frame time and graphics capabilities must be measured separately before optimizer advice is enabled.
