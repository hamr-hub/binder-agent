# AI Intent Binder

[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Language: Rust](https://img.shields.io/badge/Language-Rust-orange.svg)](https://www.rust-lang.org/)
[![Platform: Cross](https://img.shields.io/badge/Platform-Android%20%7C%20Linux%20%7C%20macOS%20%7C%20Windows%20%7C%20Browser-blue.svg)]()

一个跨平台的 AI Intent Agent + Binder Runtime，让用户用自然语言表达任务，由系统在受限、可验证、可回滚、可审计的边界内控制终端设备。

## 核心架构

```
┌───────────────────────────────────────────────┐
│  User (自然语言)                               │
└─────────────────────┬─────────────────────────┘
                      ↓
┌───────────────────────────────────────────────┐
│  LLM Intent Agent                             │
│  ├─ 自然语言理解 → 意图分类                    │
│  ├─ 参数抽取 → 结构化 Intent                  │
│  └─ 只输出 Intent，不执行动作                  │
└─────────────────────┬─────────────────────────┘
                      ↓
┌───────────────────────────────────────────────┐
│  Binder Core                                  │
│  ├─ Intent Gateway (Schema 校验/置信度)       │
│  ├─ Capability Registry (能力注册/解析)       │
│  ├─ Policy Engine (权限/风险/确认)            │
│  ├─ Execution State Machine (计划/执行/锁)   │
│  ├─ Verifier (结果验证/UI/状态/浏览器)        │
│  ├─ Rollback Manager (快照/回滚执行)          │
│  └─ Audit Logger (哈希链/日志/脱敏)           │
└─────────────────────┬─────────────────────────┘
                      ↓
┌───────────────────────────────────────────────┐
│  Platform Host Adapters                       │
│  ├─ Android (UIAutomator / Accessibility)    │
│  ├─ Linux (systemd / DBus / proc / sys)      │
│  ├─ macOS (NSWorkspace / Accessibility)      │
│  ├─ Windows (UI Automation / Win32)          │
│  └─ Browser (Playwright / chromiumoxide)     │
└─────────────────────┬─────────────────────────┘
                      ↓
┌───────────────────────────────────────────────┐
│  Device / OS / App / Browser                  │
└───────────────────────────────────────────────┘
```

## 核心原则

| 原则 | 说明 |
|------|------|
| AI 不直接执行 | LLM 只输出 Intent，不生成命令，不调用 API |
| 能力注册制 | 所有可执行动作必须来自 Capability Registry |
| 权限强约束 | Policy Engine 独立于 LLM 做授权决策 |
| 可验证 | 每步执行结果由 Verifier 检查 |
| 可回滚 | 失败时 Rollback Manager 自动恢复 |
| 可审计 | Audit Logger 记录完整哈希链，不可篡改 |

## 语言与技术栈

| 分类 | 技术 |
|------|------|
| 核心语言 | **Rust 2021 Edition** |
| 异步运行时 | tokio |
| RPC 框架 | JSON-RPC (jsonrpsee) over Unix Socket / Named Pipe |
| Schema 校验 | serde + jsonschema |
| 数据库 | SQLite (rusqlite) |
| HTTP 客户端 | reqwest |
| 浏览器自动化 | chromiumoxide (Playwright) |
| CLI | clap |
| 日志/追踪 | tracing + OpenTelemetry |
| 错误处理 | thiserror + anyhow |
| 序列化 | serde + serde_json + serde_yaml |
| 加密哈希 | sha2 (审计哈希链) |
| 时间 | chrono |

## 项目结构

```
android-binder-agent/
├── Cargo.toml                    # Rust workspace root
├── LICENSE                       # MIT License
├── README.md
├── agent.md                      # Agent 工作指南
├── docs/                         # 文档目录
│   ├── prd.md                    # 产品需求文档
│   ├── tech-solution.md          # 技术方案文档
│   └─ api-protocol.md            # 接口协议文档
├── crates/
│   ├── binder-core/              # Binder Core 核心库
│   ├── binder-schemas/           # 公共 Schema 定义
│   ├── binder-llm/               # LLM Intent Agent
│   ├── binder-host-android/      # Android Host Adapter
│   ├── binder-host-browser/      # Browser Host Adapter
│   ├── binder-host-darwin/       # macOS Host Adapter
│   ├── binder-host-linux/        # Linux Host Adapter
│   ├── binder-host-windows/      # Windows Host Adapter
│   └─ binder-rpc/                # 内部 RPC 协议
├── bin/
│   ├── binderd/                  # Binder Core 守护进程
│   ├── binderctl/                # CLI 控制工具
│   └─ binder-host-android/       # Android 守护进程
├── capabilities/                 # Capability YAML 定义
├── schemas/                      # JSON Schema 文件
├── tests/                        # 测试套件
└─ packaging/                     # Docker / 打包
```

## 快速开始

### 前置要求

- Rust 1.75+ (`rustup install stable`)
- SQLite3 (bundled，无需单独安装)
- Linux / macOS / Windows

### 编译与运行

```bash
# 克隆项目
git clone https://github.com/hamr-hub/binder-agent.git
cd binder-agent

# 编译
cargo build --release

# 启动 Binder Daemon（后台服务）
./target/release/binderd

# 使用 CLI 发送自然语言指令
./target/release/binderctl chat "帮我检查系统状态"

# 查看健康状态
./target/release/binderctl health

# 查看已注册能力列表
./target/release/binderctl capabilities
```

### Docker 部署

```bash
# 构建镜像
docker build -t binder-agent -f packaging/docker/Dockerfile .

# 运行
docker run -d --name binderd binder-agent
```

### Android 平台

```bash
# 编译 Android Host Adapter
cargo build --release -p binder-host-android

# 通过 adb 推送到设备
adb push ./target/release/binder-host-android /data/local/tmp/
```

## 文档与 Wiki

| 文档 | 说明 | 链接 |
|------|------|------|
| PRD | 产品需求文档：定位、用户故事、验收标准 | [Wiki: PRD](https://github.com/hamr-hub/binder-agent/wiki/PRD) |
| 技术方案 | 技术方案：架构、模块、状态机 | [Wiki: Tech-Solution](https://github.com/hamr-hub/binder-agent/wiki/Tech-Solution) |
| 接口协议 | 接口协议：Intent/Capability/Audit/Adapter RPC | [Wiki: API-Protocol](https://github.com/hamr-hub/binder-agent/wiki/API-Protocol) |

本地文档见 `docs/` 目录。

## MVP 范围（Phase 1）

- **平台**：Android、Linux x86_64/ARM64、Windows x86_64、macOS Apple Silicon、Browser
- **能力**：系统信息、受限文件访问、浏览器自动化、App 控制、服务状态
- **安全**：Intent Schema 校验、Policy Engine、SQLite Audit、Prompt Injection 防护

## License

[MIT](./LICENSE)
