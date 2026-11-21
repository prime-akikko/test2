# GPUI Starter — Agent Instructions

Slim desktop GUI scaffold (Rust + GPUI). German UI strings. Windows-primary.

**Confirmed product scope:** pure basic GPUI shell only.  
**No** Modbus TCP, register catalog, poll/cyclic writes, or `lambda_modbus` / catalog crates.  
Panel layout may follow test1’s *look* (header / side panel / main / status); nothing protocol-related.

## Locked starter decisions

| Topic | Decision |
| --- | --- |
| UI language | **German** user-visible strings |
| Platform | **Windows** primary (MSVC + Windows SDK, D3D11); Linux/Cloud headless-first |
| Scope | Pure GUI shell — no device/protocol I/O |
| GPUI | Pin crates.io exact version (`=0.2.2`); commit lockfile; do not float on `main` |
| Features | Default `mock` (demo copy); optional `gui` for GPUI link |

## Crate layout

- `starter_core` — nav selection + status stubs
- `starter_mock` — demo status text for headless smoke
- `starter_app` — GPUI binary (`gui` feature) + headless smoke

## Hard rules

1. **German UI** — labels, navigation, status in German.
2. **GPUI pin** — crates.io or a known revision; lockfile committed; no floating `main`.
3. **No protocol** — do not add Modbus, catalog, TCP clients, Unit-ID/Port-502 connect bars, or register grids unless explicitly requested.
4. **Windows notes** — document MSVC / SDK / D3D11 in README; Linux GUI is best-effort.

## Hooks & scripts

- `scripts/check.sh` — `cargo fmt --check` + `cargo check --workspace` (`RUN_TESTS=1` for tests).

## Working style

- Prefer small, independently testable changes.
- When unsure whether to add product features, keep the skeleton and ask.
