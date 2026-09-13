<p align="center">
  <img src="assets/app-icon-composed.png" alt="AiMaMi" width="128" height="128" />
</p>

<h1 align="center">AiMaMi</h1>

<p align="center">
  <strong>A native desktop companion for OpenAI Codex — manage accounts, routing, sessions, and local configuration in one place.</strong>
</p>

<p align="center">
  English · <a href="./README-cn.md">简体中文</a>
</p>

---

## Overview

Codex stores accounts, sessions, MCP entries, Skills, smart-router settings, and relay configuration across multiple files under `~/.codex`. Multi-account switching, quota exhaustion, third-party model setup, session cleanup, and config drift quickly turn day-to-day work into hand-editing TOML, JSON, and SQLite.

AiMaMi is built with **Tauri 2, React, and Rust**. It consolidates these high-frequency workflows — including smart routing and relay management — into a single desktop app that reads and writes Codex data locally, reducing the risk of manual file edits.

---

## Core Capabilities

| Module | Pain point addressed |
| --- | --- |
| **Account management** | Switching accounts by editing `auth.json`; scattered quota views; cumbersome import/export |
| **Auto-switch** | Work stops when 5-hour or weekly quota runs out; need automatic fallback and Codex restart |
| **Smart router** | Use relay models inside Codex Desktop while keeping historical threads resumable |
| **Relay management** | Provider setup, connectivity tests, import/export, and router diagnostics |
| **Session management** | Safely inspect, analyze, and bulk-clean local threads from the real index |
| **MCP / Skills** | Manage MCP entries and Skills lifecycle in the UI, with backup and restore |
| **Plugins** | Unified toggles for built-in extensions (e.g. web tools, image support) |
| **Custom instructions** | Manage only the AiMaMi-managed block in `~/.codex/AGENTS.md`, with preview and rollback |
| **System maintenance** | Diagnose, clean, rebuild registry, force-quit Codex, fix common config issues |
| **Settings & runtime** | Theme, language, quota refresh, API proxy, update checks; tray and macOS notch quota display |

**Smart router note:** Relay models are forwarded through AiMaMi's local proxy. Keep AiMaMi running while Codex uses relay models.

<p align="center">
  <img src="assets/console.png" alt="AiMaMi" width="1200" height="812" />
</p>
<p align="center">
  <img src="assets/aimami-qun.png" alt="AiMaMi community QR code" width="400" height="300" />
</p>

---

## Platform Support

| Platform | Notes |
| --- | --- |
| macOS | Universal (Apple Silicon + Intel), macOS 12+ |
| Windows | x64, NSIS installer |
| Linux | Best-effort support for some workflows |

---

## Tech Stack

Tauri 2 · React 18 · TypeScript · Vite 6 · Tailwind CSS · shadcn/ui · Rust

---

## Quick Start

**Requirements:** Node.js · pnpm · Rust · [Tauri prerequisites](https://v2.tauri.app/start/prerequisites/)

```bash
git clone https://github.com/ypyik0669/AiMaMi.git
cd AiMaMi
corepack enable
pnpm install --frozen-lockfile
pnpm tauri dev
```

```bash
pnpm build                                        # Frontend build check
cargo check --manifest-path src-tauri/Cargo.toml  # Rust check
pnpm tauri build                                  # Production build
```

### Windows installation

The Windows release is packaged as an NSIS installer. Install the WebView2
Runtime if it is not already present, then run the generated installer from:

```text
src-tauri/target/release/bundle/nsis/
```

For a clean build on another Windows computer:

```powershell
git clone https://github.com/ypyik0669/AiMaMi.git
cd AiMaMi
corepack enable
pnpm install --frozen-lockfile
pnpm tauri build
```

### Resource usage

AiMaMi uses lazy-loaded pages and unloads inactive pages after navigation.
It does not prewarm every feature page at startup. WebView2 still creates
separate renderer, GPU, network, and storage processes as part of its normal
desktop-app architecture, so the Task Manager process count is expected.

If the app becomes unresponsive during startup, update Microsoft Edge WebView2
and the graphics driver, then collect the AiMaMi log before deleting any local
Codex data. Do not disable WebView2 hardware acceleration as a first step:
that can move the workload from GPU to CPU and make low-end systems slower.

### Verification

Before publishing a build, run:

```bash
pnpm build
cargo test --manifest-path src-tauri/Cargo.toml
cargo build --manifest-path src-tauri/Cargo.toml --release
```

---

## Project Structure

```text
src/           React frontend
src-tauri/     Tauri shell and Rust backend
src/locales/   i18n (en / zh)
scripts/       Build and release scripts
assets/        Branding and documentation assets
```

---

## Architecture

```text
React UI ── invoke() ──▶ Tauri commands ──▶ core/
                                              ├── ~/.codex          (Codex native)
                                              └── ~/.codex/codexmate/ (AiMaMi app data)
                         platform/            macOS / Windows implementations
```

---

## Contributing

Issues and pull requests are welcome. Please read [CONTRIBUTING.md](CONTRIBUTING.md) for development setup, validation steps, and local Codex data-safety guidance. For larger changes, open an issue first so the approach can be discussed early.

---

## License

[Apache License 2.0](LICENSE)

---

## Disclaimer

AiMaMi is an independent tool for local Codex workflows. It is not affiliated with, endorsed by, or sponsored by OpenAI. Use third-party relay services at your own risk and comply with their terms of service.
