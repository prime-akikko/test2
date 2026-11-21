# GPUI Starter — Agent Instructions

Slim desktop GUI scaffold (Rust + GPUI). German UI strings. Windows-primary. Mock-first.

This repo is intentionally **not** a full Modbus product. Prefer patterns from
`prime-akikko/test1` only for workspace/shell shape — do not copy catalog, TCP,
schedulers, or register grids unless a later task asks for them.

## Locked starter decisions

| Topic | Decision |
| --- | --- |
| UI language | **German** user-visible strings |
| Platform | **Windows** primary (MSVC + Windows SDK, D3D11); Linux/Cloud mock-first |
| Tests | **Mock-first**; no live device from CI/Cloud |
| GPUI | Pin crates.io exact version (`=0.2.2`); commit lockfile; do not float on `main` |
| Features | Default `mock`; optional `gui` for GPUI link |

## Crate layout

- `starter_core` — connection settings + status stubs
- `starter_mock` — in-memory connect/disconnect
- `starter_app` — GPUI binary (`gui` feature) + headless smoke

Dependency direction: `starter_app` → `starter_core`; `starter_mock` → `starter_core`;
`starter_mock` only via feature / tests.

## Hard rules

1. **German UI** — labels, buttons, status, errors in German.
2. **GPUI pin** — crates.io or a known revision; lockfile committed; no floating `main`.
3. **Mock-first** — Cloud/CI never needs a display or live device for default builds.
4. **Stay slim** — no Modbus TCP, official register catalog, poll/cyclic schedulers, or write-confirm product UX unless explicitly requested.
5. **Windows notes** — document MSVC / SDK / D3D11 in README; Linux GUI is best-effort.

## Hooks & scripts

- `scripts/check.sh` — `cargo fmt --check` + `cargo check --workspace` (`RUN_TESTS=1` for tests).

## Working style

- Prefer small, independently testable changes.
- When unsure whether to add product features, keep the skeleton and ask.
