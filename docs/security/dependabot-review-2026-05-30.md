# Dependabot Security Review — 2026-05-30

Automated daily review of open Dependabot alerts across Censgate open-source repositories.

## Repositories Scanned

| Repository | Open Alerts |
|---|---|
| [censgate/redact](https://github.com/censgate/redact) | 1 → **0** (remediated) |
| [censgate/openclaw-redact](https://github.com/censgate/openclaw-redact) | 0 |
| [censgate/openclaw-redact-benchmark](https://github.com/censgate/openclaw-redact-benchmark) | 0 |

## Alert Details

### CVE-2026-45784 — `openssl` 0.10.79 → 0.10.80 (censgate/redact)

| Field | Value |
|---|---|
| GHSA | [GHSA-phqj-4mhp-q6mq](https://github.com/advisories/GHSA-phqj-4mhp-q6mq) |
| Severity | Medium |
| CVSS v4 | **5.1** |
| EPSS | Not yet published (FIRST.org returned no score) |
| Scope | Runtime (transitive) |
| Patched version | 0.10.80 |

**Advisory summary:** Out-of-bounds write in `CipherCtxRef::cipher_update_inplace` when used with AES key-wrap-with-padding ciphers (EVP_aes_{128,192,256}_wrap_pad). Only affects callers using those specific ciphers.

#### Risk Prioritization

- CVSS 5.1 — below the 7.0 high-priority threshold.
- EPSS unavailable; niche cipher API with no known in-the-wild exploitation data.
- **Production path:** yes — `openssl` is a transitive runtime dependency via `ort` → `ureq` → `native-tls` → `openssl` (ONNX model download TLS).
- **Reachable vulnerable API:** **no** — no application or dependency code calls `cipher_update_inplace` with AES-KW-PAD ciphers.

```
Dependency chain:
  redact-ner (ort) → ureq → native-tls → openssl 0.10.79
```

Ripgrep for `openssl`, `CipherCtx`, `cipher_update_inplace`, `wrap_pad` in `*.rs` / `Cargo.toml`: no direct usage.

#### Remediation

| Action | Status |
|---|---|
| Existing Dependabot PR [#76](https://github.com/censgate/redact/pull/76) | Merged 2026-05-30 |
| Dependabot alert [#18](https://github.com/censgate/redact/security/dependabot/18) | **Fixed** (`fixed_at: 2026-05-30T09:04:18Z`) |
| CI on merge commit `63b1d7e` | All checks green on PR #76 |

## Container Version Sync

| Source | Tag |
|---|---|
| GHCR `ghcr.io/censgate/redact` (latest full) | `0.8.3-full` / `full` (2026-04-19) |
| GHCR `ghcr.io/censgate/redact` (latest slim) | `0.8.3` / `latest` (2026-04-19) |
| Latest GitHub release | `v0.8.3` |
| openclaw-redact default (`src/config.ts`) | `ghcr.io/censgate/redact:full` |

**Result:** No container bump needed — openclaw-redact `:full` tag resolves to the current GHCR release.

## Summary

| Alert | Risk | Reachable | Fix PR | CI | Status |
|---|---|---|---|---|---|
| CVE-2026-45784 (`openssl`) | Medium (CVSS 5.1) | No (transitive TLS only) | [#76](https://github.com/censgate/redact/pull/76) merged | Green | **Fixed** |

**Open alerts remaining:** 0 across all Censgate public repos.
