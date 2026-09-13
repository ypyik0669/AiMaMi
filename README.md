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

> **Current source branch:** The visible pages are local status, MCP, Skills,
> custom instructions, maintenance, and settings. Account management, intelligent
> routing, relay management, and sessions have not been fully migrated from 1.2.6.
> The original product feature list and screenshots below are not an implementation
> checklist for this branch. Keep the original app if you depend on its relay,
> and use the isolated test script documented below.

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

**Requirements:** Node.js 24.12+ (24.x recommended; see `engines` in
`package.json` for other supported versions) · pnpm 11.7.0 · Rust stable ·
[Tauri prerequisites](https://v2.tauri.app/start/prerequisites/).
Install Corepack separately if your Node installation does not provide it.

```bash
git clone https://github.com/ypyik0669/AiMaMi.git
cd AiMaMi
corepack enable
corepack pnpm --version
pnpm install --frozen-lockfile
pnpm tauri dev
```

`packageManager` pins pnpm to **11.7.0**. If `pnpm --version` reports a
different version because another installation takes precedence on PATH,
use `corepack pnpm` instead of `pnpm` in the commands below.

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

Install Visual Studio Build Tools with **Desktop development with C++** and
a Windows SDK, plus the Rust MSVC toolchain. Reopen PowerShell after installing
the prerequisites. Building from source is CPU/RAM intensive; to avoid compiling
on a low-end laptop, build the installer on your development PC and transfer only
the resulting NSIS installer. The destination PC needs WebView2, not Node or Rust.

```powershell
git clone https://github.com/ypyik0669/AiMaMi.git
cd AiMaMi
corepack enable
corepack pnpm --version  # Expected: 11.7.0
corepack pnpm install --frozen-lockfile
if ($LASTEXITCODE -ne 0) { throw "Dependency installation failed; stop here." }
corepack pnpm tauri build
```

### Installation troubleshooting

`ERR_PNPM_MINIMUM_RELEASE_AGE_VIOLATION` means a locked dependency is newer
than the active pnpm policy permits. The reported failure affected Rollup
4.63.2 (published September 12, 2026) and its platform packages. This repository pins Rollup
to 4.63.1 in `pnpm-workspace.yaml` and the lockfile; it does not disable the
release-age or integrity checks.

For an existing checkout, update it before retrying:

```powershell
git pull --ff-only
if ($LASTEXITCODE -ne 0) { throw "Update failed; resolve local changes first." }
corepack pnpm --version
corepack pnpm install --frozen-lockfile
if ($LASTEXITCODE -ne 0) { throw "Dependency installation failed; stop here." }
corepack pnpm tauri build
```

If a stricter local policy still rejects a package, check the named version,
publication time, cutoff, and your system clock. Wait for its required age or
report the error so the lockfile can be reviewed. Do not delete the lockfile,
set `minimumReleaseAge=0`, or keep building after installation fails.

**Tauri version mismatch:** JavaScript packages and their Rust crates must
share a major/minor version. This fork pins API 2.10.x, dialog 2.6.x, updater
2.10.x, and process/shell 2.3.x on both sides. Pull the repository and rerun the
frozen install above; do not independently run `pnpm update` or `cargo update`
to fix this error. `cargo test` checks the version contract.

**App exits immediately with `PluginInitialization("updater", ...)`:** Older
builds registered the updater even when `plugins.updater` was absent. Source
builds now start without that plugin unless update endpoints and a signing
public key are configured. Automatic checks stay quiet; a manual check reports
that automatic updates are not configured, rather than claiming you are up to
date. Install newer builds manually. Release maintainers can enable the existing
updater by supplying their own signed feed in `plugins.updater`; do not invent a
public key or reuse another project's feed.

**Blank dashboard or broken logo:** The old overview route rendered nothing,
and the `load_snapshot` command was missing. The dashboard now shows read-only
local file status and navigation, with a bundled logo. Reading status does not
sync accounts, rewrite relay configuration, or repair background services.
This local status page is not the complete 1.2.6 dashboard. It does not establish
feature parity for accounts, relay routing, or usage analytics. Do not replace
a working 1.2.6 relay with this source build.

**Test without stopping your working version:** If AiMaMi provides the relay
used by Codex, exiting it will disconnect that session. Keep it running and use:

```powershell
powershell -NoProfile -ExecutionPolicy Bypass -File .\scripts\test-desktop.ps1
```

This builds a debug app with a separate application identifier, temporary
`CODEX_HOME`, and separate WebView2 data. It does not install over the existing
app. Its window is titled `AiMaMi - Isolated Test`; missing files in its empty
test directory are expected. Check the dashboard, refresh, MCP, Skills and
Settings, then exit only the test app from its tray menu. The script reports
the test PID and log directory and leaves test data for inspection.

Default production builds still enforce one instance. Closing the window only
hides it. Exit the production app for installer testing only during a maintenance
window that does not depend on its relay. No account or Codex configuration
deletion is needed for these fixes.

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
corepack pnpm install --frozen-lockfile
corepack pnpm build
cargo test --manifest-path src-tauri/Cargo.toml --locked
corepack pnpm tauri build -- --locked
```

Build success is not a startup test. Use the isolated script above to check the
actual window, dashboard status, refresh, MCP, Settings and update action first.
Installer upgrades need separate testing on a test PC or during a maintenance
window that does not depend on the relay; a debug run does not prove that path.

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
