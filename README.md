# AI Intent Binder

一个跨平台的 AI Intent Agent + Binder Runtime，让用户用自然语言表达任务，由系统在受限、可验证、可回滚、可审计的边界内控制终端设备。

## 核心架构

```
┌──────────────────────────────────────────┐
│  LLM Intent Agent                         │
│  - 自然语言理解                           │
│  - 意图分类                               │
│  - 参数抽取                               │
│  - 只输出 Intent，不执行动作               │
└───────────────────┬──────────────────────┘
                    ↓
┌──────────────────────────────────────────┐
│  Binder Core                              │
│  - Intent Gateway                         │
│  - Capability Registry                    │
│  - Policy Engine                          │
│  - Execution State Machine                │
│  - Verifier                               │
│  - Rollback Manager                       │
│  - Audit Logger                           │
└───────────────────┬──────────────────────┘
                    ↓
┌──────────────────────────────────────────┐
│  Platform Host Adapters                   │
│  - Android / iOS / Linux / Windows / macOS │
│  - Browser (Playwright)                   │
└───────────────────┬──────────────────────┘
                    ↓
┌──────────────────────────────────────────┐
│  Device / OS / App / Browser              │
└──────────────────────────────────────────┘
```

## 核心原则

- **LLM 不持有设备凭据**：AI 无法直接访问设备
- **LLM 不直接调用系统 API**：所有系统调用通过 Host Adapter
- **LLM 不生成可执行命令**：输出必须是结构化 Intent，不是 shell/PowerShell/代码
- **LLM 不决定最终授权**：Policy Engine 独立于 LLM 做授权决策
- **Binder Core 不直接依赖 OS API**：平台能力全部下沉到 Host Adapter
- **所有可执行动作来自 Capability Registry**：没有注册的能力不可执行

## 项目结构

```
ai-intent-binder/
├── Cargo.toml                    # Rust workspace root
├── README.md                     # 本文档
├── docs/
│   ├── prd.md                    # 产品需求文档
│   ├── tech-solution.md          # 技术方案文档
│   └── api-protocol.md           # 接口协议文档
├── crates/
│   ├── binder-core/              # Binder Core 核心库
│   ├── binder-schemas/           # 公共 Schema 定义
│   ├── binder-llm/               # LLM Intent Agent
│   ├── binder-host-linux/        # Linux Host Adapter
│   ├── binder-host-windows/      # Windows Host Adapter
│   ├── binder-host-darwin/       # macOS Host Adapter
│   ├── binder-host-browser/      # Browser Adapter
│   └── binder-rpc/               # 内部 RPC 协议
├── bin/
│   ├── binderd/                  # Binder Core 守护进程
│   └── binderctl/                # CLI 控制工具
├── capabilities/                 # Capability YAML 定义
├── schemas/                      # JSON Schema 文件
├── tests/                        # 测试套件
└── packaging/                    # Docker / 打包
```

## 技术栈

| 模块 | 技术 |
|------|------|
| 核心语言 | Rust |
| RPC 框架 | JSON-RPC over Unix Socket / Named Pipe |
| Schema 校验 | serde + jsonschema |
| 数据库 | SQLite (rusqlite) |
| HTTP 客户端 | reqwest |
| 浏览器自动化 | chromiumoxide |
| 异步运行时 | tokio |
| 日志/追踪 | tracing + OpenTelemetry |

## 快速开始

> 详细文档见 [docs/](./docs/) 目录。

```bash
# 克隆项目
git clone <repo-url>
cd ai-intent-binder

# 编译
cargo build --release

# 启动 Binder Daemon
./target/release/binderd

# 使用 CLI 发送自然语言指令
./target/release/binderctl chat "帮我检查系统状态"
```

## 文档索引

| 文档 | 说明 |
|------|------|
| [PRD](./docs/prd.md) | 产品需求文档：定位、用户故事、验收标准 |
| [技术方案](./docs/tech-solution.md) | 技术方案：架构、模块、状态机 |
| [接口协议](./docs/api-protocol.md) | 接口协议：Intent/Capability/Audit/Adapter RPC Schema |

## MVP 范围（Phase 1）

- **平台**：Linux x86_64/ARM64、Windows x86_64、macOS Apple Silicon、Browser
- **能力**：系统信息、受限文件访问、浏览器自动化、App 控制、服务状态
- **安全**：Intent Schema 校验、Policy Engine、SQLite Audit、Prompt Injection 防护

## License

[待定]