# AI Intent Binder - 产品需求文档 (PRD)

## 1. 产品定位

### 1.1 一句话定位

一个跨平台 AI Intent Agent + Binder Runtime，让用户用自然语言表达任务，由系统在受限、可验证、可回滚、可审计的边界内控制终端设备。

### 1.2 核心差异

它不是简单的"AI 控屏幕"，而是：

```
用户表达目标
  → LLM 识别意图
    → Binder 选择受限能力
      → Policy 判断是否允许
        → Adapter 执行平台动作
          → Verifier 检查结果
            → Rollback 处理失败
              → Audit 记录全过程
```

### 1.3 产品原则

| 原则 | 说明 |
|------|------|
| AI 不直接执行 | LLM 只输出 Intent，不生成命令，不调用 API |
| 能力注册制 | 所有可执行动作必须来自 Capability Registry |
| 权限强约束 | Policy Engine 独立于 LLM 做授权决策 |
| 全链路可追踪 | 每个 Intent 从接收到执行到验证，全程审计 |
| 失败可恢复 | 可逆操作自动回滚，不可逆操作强确认 |
| 跨平台原生化 | Core 跨平台，每个平台 Adapter 走原生权限模型 |

## 2. 用户故事

### 2.1 个人终端助理

**用户说**：帮我进入会议模式。

**系统解析为**：
1. 降低系统音量到 30%
2. 打开日历
3. 打开会议链接
4. 打开摄像头预检页面
5. 开启勿扰模式

**Binder 判断**：

| 动作 | 风险等级 | 处理方式 |
|------|----------|----------|
| 调音量 | R1 低风险 | 自动执行 |
| 打开日历 | R1 低风险 | 自动执行 |
| 打开会议链接 | R2 中风险 | 需要确认域名 |
| 勿扰模式 | R2 中风险 | 需要用户授权 |
| 摄像头相关 | R3 高风险 | 只能打开检查页，不能静默启用 |

### 2.2 企业运维终端

**用户说**：检查这台边缘 Linux 设备的网络和采集服务状态。

**系统解析为**：
- `system.network.status`
- `system.service.status(service=collector)`
- `system.process.list(filter=collector)`

**Binder 判断**：
- 只读 R0 能力
- 允许执行
- 不需要回滚
- 记录审计

### 2.3 移动端自动化测试

**测试人员说**：在 Android 和 iOS 上验证登录流程。

**系统解析为**：
- `mobile.test.run_flow(flow=login_smoke)`
- `target_platforms=[android, ios]`

**执行路径**：
- Android：Appium UiAutomator2 或 Maestro
- iOS：Appium XCUITest 或 Maestro（宿主必须是 macOS）

### 2.4 高风险动作拦截

**用户说**：帮我把客户群里的所有人都发一条促销消息。

**系统解析后进入高风险**：
- `external_message.send_bulk`
- `risk = R4 critical`
- `requires_confirmation = true`
- `requires_preview = true`
- `requires_recipient_count = true`
- `requires_rate_limit = true`

**Binder 不允许 LLM 直接执行发送**，而是生成预览、收件人清单、发送范围、撤回能力说明，并要求人工确认。

## 3. 功能范围

### 3.1 MVP 范围（Phase 1 - 当前）

| 维度 | 范围 |
|------|------|
| 平台 | Linux x86_64/ARM64、Windows x86_64、macOS Apple Silicon、Browser |
| 核心能力 | Intent Schema、Capability Registry、Policy Engine、状态机、SQLite Audit |
| LLM Agent | OpenAI 兼容 API，结构化 Intent 输出 |
| Browser | URL alias 导航、文本验证、已知元素点击、已知字段填写 |
| Linux | 系统信息、网络状态、服务状态、进程列表、受限文件访问 |
| Windows | App 打开/关闭、UI inspect、系统信息 |
| macOS | App 打开/关闭、Accessibility 权限检查、系统信息 |
| CLI | binderctl chat / intent / health / capabilities / audit |

### 3.2 暂缓范围

| 项目 | 暂缓原因 |
|------|----------|
| iOS 生产控制 | 需要 App Intents SDK 集成 |
| Android 任意 Accessibility 控制 | Google Play 政策限制 |
| Linux 任意 GUI 控制 | GUI 生态碎片化 |
| 高风险外部动作（发消息、支付、删除） | 需要审批流 + 强确认 |
| 多租户 | 先单用户跑通 |
| 远程设备管理 | 先本地设备跑通 |

## 4. 验收标准

### 4.1 功能验收

| 验收项 | 标准 |
|--------|------|
| Intent 输出 | 100% 符合 JSON Schema |
| 未知能力 | 100% 拒绝 |
| 参数越界 | 100% 拒绝 |
| 平台不支持 | 返回明确 unsupported |
| 低风险能力 | 能完整执行并验证 |
| 中风险能力 | 必须出现确认流程 |
| 高风险能力 | 默认禁止或强确认 |
| 审计日志 | 每个 Intent 100% 有记录 |
| 回滚 | 可逆能力失败后自动恢复 |
| 多平台 | 同一 Intent 在支持平台调用对应 Adapter |

### 4.2 安全验收

| 验收项 | 标准 |
|--------|------|
| LLM 不能获得设备凭据 | 通过架构保证 |
| LLM 输出 shell 不会被执行 | Schema 拒绝任何含 shell/code 字段的 Intent |
| LLM 不能绕过 Capability Registry | 未注册 intent_type 直接拒绝 |
| LLM 不能改写 risk_level | Policy Engine 覆盖 LLM 的 risk_hint |
| Adapter 不能执行未注册方法 | Contract 级别校验 |
| 审计日志不能被删除 | Append-only + hash chain |
| Prompt Injection 防护 | 对抗测试 100% 拒绝 |

### 4.3 性能验收

| 指标 | 目标 |
|------|------|
| Intent 识别 P95 | < 2s |
| 低风险本地操作 P95 | < 3s |
| 浏览器打开并验证 P95 | < 8s |
| Binder Core 内存 | < 200MB |
| Host Adapter 内存 | < 150MB |

### 4.4 可靠性验收

| 验收项 | 标准 |
|--------|------|
| binderd 崩溃恢复 | 可恢复未完成任务状态 |
| Adapter 崩溃不影响 Core | 返回明确错误 |
| 设备离线 | 返回明确错误 |
| 重复 Intent | 幂等保护 |
| 并发操作同一资源 | 有锁 |
| 执行失败 | 进入 rollback 或 fail-safe |

## 5. 阶段性路线图

| 阶段 | 时间 | 交付 |
|------|------|------|
| Phase 0 | 当前 | 文档完善：PRD、技术方案、接口协议 |
| Phase 1 | 2-4 周 | MVP 代码实现：Core + Browser/Linux/Windows/macOS Adapter + CLI |
| Phase 2 | 4-8 周 | 跨平台 Alpha：Policy Engine 完善、Confirmation UI、Rollback、OpenTelemetry |
| Phase 3 | 8-12 周 | Mobile + 企业内测：Android Host App、iOS SDK、mTLS、设备注册 |
| Phase 4 | 生产化 | 多租户、审计不可篡改、审批流、远程更新、安全基线扫描 |