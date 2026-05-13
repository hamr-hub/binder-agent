# AI Intent Binder - Agent 工作指南

## 项目结构

```
android-binder-agent/
├── Cargo.toml                    # Rust workspace root
├── README.md                     # 项目总览
├── agent.md                      # 本文件 - Agent 工作指南
├── docs/
│   ├── 2026-05-13-PRD.md         # 完整方案文档
│   ├── prd.md                    # 产品需求文档
│   ├── tech-solution.md          # 技术方案文档
│   └── api-protocol.md           # 接口协议文档
├── crates/
│   ├── binder-core/              # Binder Core 核心库
│   │   ├── src/
│   │   │   ├── lib.rs
│   │   │   ├── audit.rs          # 审计模块入口
│   │   │   ├── audit/
│   │   │   │   ├── hashchain.rs  # 哈希链
│   │   │   │   ├── logger.rs     # 审计日志
│   │   │   │   └── redaction.rs  # 数据脱敏
│   │   │   ├── capability.rs     # 能力模块入口
│   │   │   ├── capability/
│   │   │   │   ├── registry.rs   # 能力注册表
│   │   │   │   ├── resolver.rs   # 能力解析器
│   │   │   ├── execution.rs      # 执行模块入口
│   │   │   ├── execution/
│   │   │   │   ├── adapter_client.rs  # Adapter 客户端
│   │   │   │   ├── lock_manager.rs    # 锁管理
│   │   │   │   ├── planner.rs         # 执行计划
│   │   │   │   ├── state_machine.rs   # 状态机
│   │   │   ├── intent.rs         # 意图模块入口
│   │   │   ├── intent/
│   │   │   │   ├── normalizer.rs # 意图规范化
│   │   │   │   ├── router.rs     # 意图路由
│   │   │   │   ├── validator.rs  # 意图校验
│   │   │   ├── policy.rs         # 策略模块入口
│   │   │   ├── policy/
│   │   │   │   ├── confirmation.rs # 确认机制
│   │   │   │   ├── engine.rs       # 策略引擎
│   │   │   │   ├── risk.rs          # 风险评估
│   │   │   ├── rollback.rs       # 回滚模块入口
│   │   │   ├── rollback/
│   │   │   │   ├── executor.rs   # 回滚执行器
│   │   │   │   ├── snapshot.rs   # 快照管理
│   │   │   ├── verifier.rs       # 验证模块入口
│   │   │   ├── verifier/
│   │   │   │   ├── browser_verifier.rs  # 浏览器验证
│   │   │   │   ├── state_verifier.rs    # 状态验证
│   │   │   │   ├── ui_verifier.rs       # UI 验证
│   ├── binder-schemas/           # 公共 Schema 定义
│   │   ├── src/
│   │   │   ├── lib.rs
│   │   │   ├── audit.rs
│   │   │   ├── capability.rs
│   │   │   ├── intent.rs
│   ├── binder-llm/               # LLM Intent Agent
│   │   ├── src/
│   │   │   ├── lib.rs
│   │   │   ├── client.rs         # LLM 客户端
│   │   │   ├── extractor.rs      # 参数抽取
│   │   │   ├── prompts.rs        # Prompt 模板
│   ├── binder-rpc/               # 内部 RPC 协议
│   │   ├── src/
│   │   │   ├── lib.rs
│   ├── binder-host-android/      # Android Host Adapter
│   │   ├── src/
│   │   │   ├── lib.rs
│   ├── binder-host-browser/      # Browser Host Adapter
│   │   ├── src/
│   │   │   ├── lib.rs
│   ├── binder-host-darwin/       # macOS Host Adapter
│   │   ├── src/
│   │   │   ├── lib.rs
│   ├── binder-host-linux/        # Linux Host Adapter (待创建)
│   │   ├── src/
│   │   │   ├── lib.rs
│   ├── binder-host-windows/      # Windows Host Adapter
│   │   ├── src/
│   │   │   ├── lib.rs
├── bin/
│   ├── binderd/                  # Binder Core 守护进程
│   │   ├── src/
│   │   │   ├── main.rs
│   ├── binderctl/                # CLI 控制工具
│   │   ├── src/
│   │   │   ├── main.rs
├── capabilities/                 # Capability YAML 定义
│   ├── android/
│   │   ├── android.app.launch.yaml
│   │   ├── android.device.info.yaml
│   │   ├── android.screen.click.yaml
│   ├── browser/
│   │   ├── click-element.yaml
│   │   ├── fill-field.yaml
│   │   ├── open-url.yaml
│   │   ├── verify-text.yaml
│   ├── common/
│   │   ├── capability-list.yaml
│   │   ├── file-read-confined.yaml
│   │   ├── health.yaml
│   │   ├── system-info.yaml
│   ├── darwin/
│   │   ├── darwin-capabilities.yaml
│   ├── linux/
│   │   ├── linux-capabilities.yaml
│   ├── windows/
│   │   ├── windows-capabilities.yaml
├── schemas/                      # JSON Schema 文件
│   ├── audit.schema.json
│   ├── capability.schema.json
│   ├── intent.schema.json
├── tests/                        # 测试套件
│   ├── adapter_contract/
│   ├── adversarial/
│   ├── intent_eval/
│   ├── policy/
│   ├── schema/
├── packaging/                    # Docker / 打包
│   ├── docker/
│   │   ├── Dockerfile
```

## docs 目录索引

`docs/` 目录为 GitHub Wiki 的 git submodule（指向 `binder-agent.wiki`）。

| 文档 | 说明 |
|------|------|
| `docs/Home.md` | Wiki 首页 |
| `docs/PRD.md` | 产品需求文档：定位、用户故事、验收标准 |
| `docs/Tech-Solution.md` | 技术方案：架构、模块、状态机、序列图 |
| `docs/API-Protocol.md` | 接口协议：Intent/Capability/Audit/Adapter RPC Schema |

> clone 项目后需执行 `git submodule update --init` 拉取 docs（Wiki）内容，
> 或使用 `git clone --recurse-submodules` 一次性拉取。

---

## Rules

### Git 提交推送规则

#### 远端仓库配置

本项目配置两个远端仓库：

| 远端名称 | 仓库地址 | 类型 | 说明 |
|----------|----------|------|------|
| `origin` | `android-binder-agent` (GitHub 私库) | 私有 | **默认远端**，只推送最新代码，每个任务完成打一个 tag |
| `public` | `binder-agent` (GitHub 公开库) | 公开 | 不包含公司等私密信息，版本落后私库，按规则同步 |

#### 配置命令

```bash
# origin 已默认配置为 android-binder-agent 私库
# 添加公开库远端
git remote add public https://github.com/hamr-hub/binder-agent.git
```

#### 提交推送流程

**规则：默认先拉取，解决冲突再推送。**

```
1. 拉取最新代码
   git fetch origin
   git pull origin main

2. 如果有冲突 → 解决冲突
   git mergetool 或手动解决
   git add <冲突文件>
   git commit -m "resolve: merge conflict with origin/main"

3. 提交代码
   git add <文件>
   git commit -m "<commit message>"

4. 推送到私库（origin）
   git push origin main
```

#### Tag 版本管理规则

1. **每个任务完成后打 tag**

```bash
# 查看现有 tag
git tag --sort=-v:refname | head -10

# 打 tag（使用语义化版本号）
git tag -a v0.1.0 -m "feat: 完成 Android Host Adapter 基础框架"
git push origin v0.1.0
```

2. **Tag 版本号规则**

- 使用语义化版本：`vMAJOR.MINOR.PATCH`
- MAJOR：重大架构变更
- MINOR：新增功能/模块
- PATCH：bugfix / 小改进

3. **私库与公开库同步规则**

- 私库（origin）每个任务打 tag，持续迭代
- 公开库（public）版本落后私库
- **当私库 tag 版本超过公开库 4 代时，将前 2 代推送至公开库**

```
示例：
  公开库当前版本: v0.4.0
  私库最新版本:   v0.8.0（超过公开库 4 代）
  
  触发同步 → 将 v0.6.0 和 v0.7.0 推送到公开库
  公开库更新后版本: v0.7.0
  
  下次私库达到 v0.11.0 时 → 将 v0.9.0 和 v0.10.0 推送到公开库
```

#### 同步到公开库的流程

```bash
# 1. 检查版本差距
git tag --sort=-v:refname          # 查看私库 tag
git ls-remote --tags public        # 查看公开库最新 tag

# 2. 计算需要推送的 tag（超过 4 代的前 2 代）
#    例如公开库 v0.4.0，私库 v0.8.0 → 推送 v0.6.0, v0.7.0

# 3. 推送代码和 tag 到公开库
git push public main               # 推送代码
git push public v0.6.0 v0.7.0      # 推送指定 tag

# 4. 验证
git ls-remote --tags public        # 确认公开库 tag 已更新
```

#### 公开库内容脱敏规则

推送至公开库前，必须确保代码不含以下内容：

- 公司内部域名、IP、服务地址
- 内部 API Key / Token / Secret
- 内部工具名称（kconf、kdb、halo 等）
- 内部文档链接（docs.corp.kuaishou.com）
- 业务相关敏感数据

#### 禁止操作

- **禁止**直接 `git push public` 不走同步规则
- **禁止** `git push --force` 到公开库
- **禁止**跳过 `git pull` 直接推送（必须先拉取解决冲突）
- **禁止**将私密信息推送到公开库

#### 快速参考

```bash
# 日常推送（私库）
git fetch origin && git pull origin main && git push origin main

# 打 tag
git tag -a vX.Y.Z -m "描述" && git push origin vX.Y.Z

# 同步公开库（仅当版本超过 4 代时）
git push public main && git push public <tag1> <tag2>
```
