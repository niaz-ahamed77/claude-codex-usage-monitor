# Audited fork policy

Baseline upstream source: 1b3ea766d2047a183ffdbb321bf13b5a93d853dd (v1.3.1).

Security posture:
- Usage polling remains limited to Anthropic and ChatGPT/Codex endpoints.
- Claude/Codex credentials remain read from the official CLI credential stores.
- Automatic executable self-updates are disabled.
- Upgrades require a deliberate source review, merge, rebuild, and binary hash check.
- Do not install upstream release binaries directly over an audited build.
