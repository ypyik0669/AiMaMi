<p align="center">
  <img src="assets/app-icon-composed.png" alt="AiMaMi" width="128" height="128" />
</p>

<h1 align="center">AiMaMi</h1>

<p align="center">
  <strong>面向 OpenAI Codex 的原生桌面伴侣 —— 统一管理账号、路由、会话与本地配置。</strong>
</p>

<p align="center">
  <a href="./README.md">English</a> · 简体中文
</p>

---

## 概述

Codex 的账号、会话、MCP、Skills、智能路由与中转配置分散在 `~/.codex` 下的多个文件里。多账号切换、额度耗尽、第三方模型接入与路由维护、会话清理和配置漂移，都会把日常操作变成手改 TOML / JSON / SQLite。

AiMaMi 基于 **Tauri 2 + React + Rust**，把这些高频操作 —— 含智能路由与中转管理 —— 收敛到一个桌面应用里，在本地安全读写 Codex 数据，减少手工改文件带来的风险。

---

## 核心能力

| 模块 | 解决的痛点 |
| --- | --- |
| **账号管理** | 多账号切换靠手改 `auth.json`；额度分散、导入导出麻烦 |
| **自动切换** | 5 小时 / 周额度触顶后任务中断，需自动找可用账号并重启 Codex |
| **智能路由** | 在 Codex 桌面内使用中转模型，同时尽量保留历史线程可续聊 |
| **中转管理** | Provider 配置、连通性测试、导入导出与路由诊断 |
| **会话管理** | 基于真实索引安全查看、统计与批量清理本地线程 |
| **MCP / Skills** | 图形化管理 MCP 条目与 Skills 生命周期，支持备份恢复 |
| **插件** | 统一管理内置扩展（如 web tools、image support） |
| **自定义指令** | 仅管理 `~/.codex/AGENTS.md` 中的 AiMaMi 受控区块，支持预览与回滚 |
| **系统维护** | 诊断、清理、重建 registry、强杀 Codex、修复常见配置问题 |
| **设置与运行时** | 主题、语言、额度刷新、API 代理、更新检查；托盘与 macOS 刘海额度展示 |

**智能路由说明：** 中转模型经 AiMaMi 本地代理转发，使用期间需保持 AiMaMi 运行。

<p align="center">
  <img src="assets/console.png" alt="AiMaMi" width="1200" height="812" />
</p>
<p align="center">
  <img src="assets/qr1.png" alt="AiMaMi 社区群二维码" width="400" height="300" />
</p>

---

## 平台支持

| 平台 | 说明 |
| --- | --- |
| macOS | Universal（Apple Silicon + Intel），macOS 12+ |
| Windows | x64，NSIS 安装包 |
| Linux | 部分能力为尽力支持 |

---

## 技术栈

Tauri 2 · React 18 · TypeScript · Vite 6 · Tailwind CSS · shadcn/ui · Rust

---

## 快速开始

**环境要求：** Node.js 24.12+（推荐 24.x；其他支持的版本见 `package.json`
的 `engines`）· pnpm 11.7.0 · Rust stable ·
[Tauri 系统依赖](https://v2.tauri.app/start/prerequisites/)。
若 Node 安装未包含 Corepack，请先单独安装 Corepack。

```bash
git clone https://github.com/ypyik0669/AiMaMi.git
cd AiMaMi
corepack enable
corepack pnpm --version
pnpm install --frozen-lockfile
pnpm tauri dev
```

仓库通过 `packageManager` 固定使用 **pnpm 11.7.0**。如果 `pnpm --version`
显示其他版本，说明 PATH 中的另一份安装可能优先被调用；请将后续命令中的
`pnpm` 替换为 `corepack pnpm`。

```bash
pnpm build                                      # 前端构建检查
cargo check --manifest-path src-tauri/Cargo.toml  # Rust 检查
pnpm tauri build                                # 生产构建
```

### Windows 安装

Windows 版本使用 NSIS 安装包。若系统尚未安装 WebView2 Runtime，请先安装
Microsoft Edge WebView2 Runtime，然后运行构建生成的安装包。安装包通常位于：

```text
src-tauri/target/release/bundle/nsis/
```

在另一台 Windows 电脑上从源码构建：

请先安装 Visual Studio Build Tools，勾选 **使用 C++ 的桌面开发** 和
Windows SDK，并安装 Rust MSVC 工具链。安装完成后重新打开 PowerShell。
源码编译本身会占用较多 CPU 和内存；低配置笔记本更适合直接安装开发电脑
生成并传过去的 NSIS 安装包，目标电脑需要 WebView2，无需 Node 或 Rust。

```powershell
git clone https://github.com/ypyik0669/AiMaMi.git
cd AiMaMi
corepack enable
corepack pnpm --version  # 应为 11.7.0
corepack pnpm install --frozen-lockfile
if ($LASTEXITCODE -ne 0) { throw "Dependency installation failed; stop here." }
corepack pnpm tauri build
```

### 安装报错排查

`ERR_PNPM_MINIMUM_RELEASE_AGE_VIOLATION` 表示锁定的依赖发布时间尚未达到
当前 pnpm 策略要求。此次报错涉及 2026 年 9 月 12 日发布的 Rollup 4.63.2 及其平台包；
仓库已在 `pnpm-workspace.yaml` 和锁文件中固定为 4.63.1，没有关闭发布时间
或完整性检查。

已有源码目录请先更新，再重试：

```powershell
git pull --ff-only
if ($LASTEXITCODE -ne 0) { throw "Update failed; resolve local changes first." }
corepack pnpm --version
corepack pnpm install --frozen-lockfile
if ($LASTEXITCODE -ne 0) { throw "Dependency installation failed; stop here." }
corepack pnpm tauri build
```

如果更严格的本机策略仍拦截依赖，请核对报错中的包版本、发布时间、截止时间
以及系统时钟。等待满足策略要求，或提交日志以便复核锁文件。不要删除锁文件、
设置 `minimumReleaseAge=0`，也不要在依赖安装失败后继续构建。

### 性能与资源占用

AiMaMi 使用按需懒加载页面，切换页面后会卸载不再使用的旧页面，启动时不会
预加载全部功能页面。WebView2 的渲染、GPU、网络和存储进程属于桌面应用的正常
架构，因此任务管理器中看到多个 WebView2 进程是预期现象。

如果启动时出现卡顿或无响应，请先更新 Microsoft Edge WebView2 Runtime 和显卡
驱动，并保留 AiMaMi 日志用于排查，再考虑其他措施。不要一开始就禁用 WebView2
硬件加速；这可能把负载转移到 CPU，反而让低配置电脑更慢。

### 发布前验证

```bash
pnpm build
cargo test --manifest-path src-tauri/Cargo.toml
cargo build --manifest-path src-tauri/Cargo.toml --release
```

---

## 项目结构

```text
src/           React 前端
src-tauri/     Tauri 壳与 Rust 后端
src/locales/   国际化（中 / 英）
scripts/       构建与发布脚本
assets/        品牌与文档素材
```

---

## 架构

```text
React UI ── invoke() ──▶ Tauri commands ──▶ core/
                                              ├── ~/.codex          (Codex 原生)
                                              └── ~/.codex/codexmate/ (AiMaMi 数据)
                         platform/            macOS / Windows 差异实现
```

---

## 参与贡献

欢迎提交 Issue 与 Pull Request。请先阅读 [CONTRIBUTING.md](CONTRIBUTING.md)，了解开发环境、提交前检查以及本地 Codex 数据安全注意事项。较大改动建议先开 Issue 讨论方案。

---

## 许可证

[Apache License 2.0](LICENSE)

---

## 免责声明

AiMaMi 是独立的 Codex 本地工作流工具，与 OpenAI 无隶属、背书或赞助关系。使用第三方中转服务请自行评估风险并遵守相应条款。
