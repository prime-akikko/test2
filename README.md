# test2 — Pure GPUI Desktop Shell

Slim Rust + GPUI desktop GUI workspace (German UI, Windows-primary).  
Panel layout inspired by [prime-akikko/test1](https://github.com/prime-akikko/test1) — **without** Modbus TCP, register catalog, poll/cyclic writes, or any protocol crates.

## Workspace

| Crate | Path | Role |
| --- | --- | --- |
| `starter_core` | `crates/core` | Nav selection + status stubs |
| `starter_mock` | `crates/mock` | Demo status copy for headless smoke |
| `starter_app` | `crates/app` | GPUI binary (optional `gui` feature) |

## Build / run

```bash
# Toolchain comes from rust-toolchain.toml (1.99.1)
cargo check --workspace
cargo test --workspace
./scripts/check.sh

# Headless smoke (default features = mock demo copy, no GPUI native link)
cargo run -p starter_app

# Optional GUI typecheck / Windows window
cargo check -p starter_app --features gui
cargo run -p starter_app --features gui   # Windows (or Linux with display + deps)
```

Set `RUN_TESTS=1 ./scripts/check.sh` to include tests in the check script.

## GPUI pin

- crates.io **`gpui = "=0.2.2"`** (workspace dep, lockfile committed)
- Do **not** float on zed/zed `main`
- Feature `gui` is optional so Cloud/Linux `cargo check --workspace` stays green

See [`crates/app/README.md`](crates/app/README.md) for Windows MSVC / D3D11 notes and the Linux `xattr` patch.

## Platform

| Target | Role |
| --- | --- |
| **Windows** | Primary GUI acceptance (MSVC + Windows SDK + D3D11) |
| Linux / Cloud | Headless `cargo run` / `cargo test`; `gui` compile is best-effort |

## Out of scope

Modbus TCP, Lambda EU13L catalog, register grids, poll/cyclic schedulers, device connect UI, `lambda_*` protocol crates.

## Agent notes

See [`AGENTS.md`](AGENTS.md). Keep this repo a **pure GUI skeleton**.
