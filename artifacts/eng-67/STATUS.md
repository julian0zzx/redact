# ENG-67 — redact-full container + `/healthz`

Worktree-local evidence for Linear ENG-67 (platform redact HTTP boundary).

## Acceptance

| # | Criterion | Status |
|---|-----------|--------|
| 1 | `Dockerfile.ner` produces a runnable **redact-full** image with **`GET /healthz` → HTTP 200** | **PASS** — smoke used locally built `redact-full:eng-67-local` |
| 2 | Run notes document required env vars, container port (`8080`), and **`PLATFORM_REDACT_API_URL`** usage | **PASS** — README “Full image” + REST quick note |
| 3 | Smoke transcript: `docker run` + `curl /healthz` | **PASS** — `commands.log`, `healthz-curl.log` |

## Alignment with seemath compose (profile `redact`)

Reference commit `5ff9342` (`agent/eng-67-compose-redact-profile`): service **`redact-full`** maps **`8081:8080`**. Set **`PLATFORM_REDACT_API_URL=http://localhost:8081`** when exercising from the host (origin only — no path).

## Evidence files

- `commands.log` — Docker build + run/remove transcript tail
- `healthz-curl.log` — verbose `curl` against `/healthz` (shows **HTTP/1.1 200**)
- `diff.patch` — patch vs `main` at capture time
