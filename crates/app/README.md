# `starter_app` — GPUI Starter Shell

Windows-primary desktop binary: thin German UI shell (header, Verbindungspanel, Inhalt, Statusleiste).

## GPUI pin

| Item | Value |
| --- | --- |
| Source | crates.io |
| Crate / version | `gpui = "=0.2.2"` (exact pin in workspace `Cargo.toml`) |
| Why not Git `main` | Pre-1.0 churn; lockfile must be reproducible |
| Rust | ≥ **1.85** (gpui 0.2.2 uses edition 2024); repo pins `rust-toolchain.toml` → **1.99.1** |
| Feature flag | `gui` — optional so Linux/Cloud `cargo check --workspace` stays green without native GUI libs |

```bash
# Headless / Cloud (default: mock, no GPUI link)
cargo run -p starter_app

# Windows GUI (keep mock for in-memory connect smoke)
cargo run -p starter_app --features gui
```

## Shell layout (German UI)

1. **Header** — App-Titel + Unterzeile  
2. **Verbindung** — Host / Port / Unit-ID (Platzhalter), **Verbinden** / **Trennen** (Mock)  
3. **Inhalt** — leerer Hauptbereich als Platzhalter  
4. **Status** — Verbindungsstatus, Backend-Label, Meldung  

No Modbus TCP, catalog, register grid, or cyclic-write product features in this starter.

## Windows build prerequisites (v1 target)

1. [rustup](https://rustup.rs/) target `x86_64-pc-windows-msvc`  
2. Visual Studio Build Tools with **MSVC** C++ toolchain  
3. Windows SDK  
4. Direct3D 11 Feature Level **≥ 11.0** GPU/driver  

```bash
cargo build -p starter_app --release --features gui
```

Artefact: `target/release/starter_app.exe`.

## Cloud / Linux notes

| Path | Expectation |
| --- | --- |
| `cargo check --workspace` / `cargo test --workspace` | Green without `gui` |
| `cargo check -p starter_app --features gui` | Compiles GPUI on Linux when system GUI deps + vendored `xattr` patch are present |
| Full window smoke | Local **Windows** acceptance; Wayland/X11 may work on Linux but are not v1 AC |

Workspace `[patch.crates-io] xattr` (see `vendor/xattr`) fixes a `libc::ENOATTR` breakage on recent Linux toolchains.
