# Optimizer safety contract

The optimizer begins as an advice-only subsystem. It may recommend closing unnecessary apps, lowering in-game texture quality, or stopping a game under critical pressure.

It must not:
- kill user processes automatically;
- disable macOS swap or SIP;
- claim to free unified memory with a magic cleanup;
- alter global settings without explicit consent and a rollback path;
- infer performance from RAM capacity alone.

On 8 GB Macs, present realistic warnings and measure frame time, memory pressure and thermal behavior before making per-game recommendations.
