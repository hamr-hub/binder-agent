# AI Intent Binder - 接口协议文档

## 1. Intent Schema

### 1.1 Intent JSON Schema

```json
{
  "$schema": "https://json-schema.org/draft/2020-12/schema",
  "$id": "https://ai-intent-binder/schemas/intent.schema.json",
  "title": "Intent",
  "description": "AI Intent Binder 的 LLM 意图输出结构。LLM 只能输出此结构，不能输出可执行命令。",
  "type": "object",
  "required": [
    "intent_id",
    "intent_type",
    "target",
    "parameters",
    "risk_hint",
    "confidence"
  ],
  "additionalProperties": false,
  "properties": {
    "intent_id": {
      "type": "string",
      "description": "意图唯一标识，UUID v4"
    },
    "intent_type": {
      "type": "string",
      "description": "意图类型，必须是预注册的 Capability ID",
      "enum": [
        "binder.capability.list",
        "binder.health.check",
        "system.info.get",
        "system.network.status",
        "system.service.status",
        "system.process.list",
        "app.open",
        "app.close",
        "device.audio.set_volume",
        "device.display.set_brightness",
        "browser.open_url",
        "browser.verify_text",
        "browser.click_known_element",
        "browser.fill_known_field",
        "mobile.test.run_flow",
        "file.read_confined",
        "file.write_confined",
        "ui.inspect",
        "accessibility.permission.check"
      ]
    },
    "target": {
      "type": "object",
      "required": ["platform", "device_id"],
      "additionalProperties": false,
      "properties": {
        "platform": {
          "type": "string",
          "enum": ["android", "ios", "linux", "windows", "darwin", "browser"]
        },
        "device_id": {
          "type": "string",
          "description": "设备标识，本地设备为 'local'"
        }
      }
    },
    "parameters": {
      "type": "object",
      "description": "意图参数，由对应 Capability 的 input_schema 二次校验"
    },
    "risk_hint": {
      "type": "string",
      "description": "LLM 对风险的初步判断，仅供参考，不决定最终授权",
      "enum": ["read_only", "low", "medium", "high", "critical"]
    },
    "confidence": {
      "type": "number",
      "minimum": 0,
      "maximum": 1,
      "description": "LLM 置信度，仅供调试参考"
    },
    "needs_clarification": {
      "type": "boolean",
      "description": "是否需要向用户追问澄清参数"
    },
    "clarification_question": {
      "type": "string",
      "description": "澄清问题文本"
    },
    "natural_language_reason": {
      "type": "string",
      "description": "自然语言解释为什么选择此意图"
    }
  }
}
```

### 1.2 Intent 类型定义

```rust
/// Intent 类型枚举
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum IntentType {
    // Common
    BinderCapabilityList,
    BinderHealthCheck,
    SystemInfoGet,
    FileReadConfined,
    FileWriteConfined,

    // Linux
    SystemNetworkStatus,
    SystemServiceStatus,
    SystemProcessList,

    // Windows / macOS
    AppOpen,
    AppClose,

    // Device (Linux/Windows/macOS)
    DeviceAudioSetVolume,
    DeviceDisplaySetBrightness,

    // Browser
    BrowserOpenUrl,
    BrowserVerifyText,
    BrowserClickKnownElement,
    BrowserFillKnownField,

    // Windows UI
    UiInspect,

    // macOS
    AccessibilityPermissionCheck,

    // Mobile (暂缓)
    MobileTestRunFlow,
}
```

### 1.3 Intent 校验规则

| 校验规则 | 失败行为 |
|----------|----------|
| `additionalProperties = false` | 拒绝含额外字段的 Intent |
| `intent_type` 不在 enum 中 | 返回 `unknown_intent_type` 错误 |
| `target.platform` 不在 enum 中 | 返回 `unsupported_platform` 错误 |
| `confidence` 不在 [0, 1] | 拒绝 |
| `parameters` 含有 `cmd`/`shell`/`command`/`code`/`script` 等危险字段 | 拒绝，记录安全事件 |
| `intent_type` 对应 Capability 未注册或未激活 | 返回 `capability_not_found` |
| `target.platform` 不在 Capability 的 `supported_platforms` 中 | 返回 `platform_not_supported` |

### 1.4 多意图拆解（Plan）

```json
{
  "plan_id": "plan_001",
  "intents": [
    {
      "intent_id": "intent_001",
      "intent_type": "device.display.set_brightness",
      "target": { "platform": "windows", "device_id": "local" },
      "parameters": { "level_percent": 80 },
      "risk_hint": "low",
      "confidence": 0.91
    },
    {
      "intent_id": "intent_002",
      "intent_type": "device.audio.set_volume",
      "target": { "platform": "windows", "device_id": "local" },
      "parameters": { "level_percent": 20 },
      "risk_hint": "low",
      "confidence": 0.89
    },
    {
      "intent_id": "intent_003",
      "intent_type": "browser.open_url",
      "target": { "platform": "browser", "device_id": "local" },
      "parameters": { "url_alias": "project_dashboard" },
      "risk_hint": "medium",
      "confidence": 0.87
    }
  ]
}
```

## 2. Capability Schema

### 2.1 Capability JSON Schema

```json
{
  "$schema": "https://json-schema.org/draft/2020-12/schema",
  "$id": "https://ai-intent-binder/schemas/capability.schema.json",
  "title": "Capability",
  "description": "Capability 注册定义。每个可执行动作必须在此注册。",
  "type": "object",
  "required": [
    "id",
    "version",
    "description",
    "risk_level",
    "reversible",
    "requires_confirmation",
    "input_schema",
    "supported_platforms",
    "bindings"
  ],
  "additionalProperties": false,
  "properties": {
    "id": {
      "type": "string",
      "pattern": "^[a-z_]+(\\.[a-z_]+)+$",
      "description": "Capability 唯一 ID，格式: category.action"
    },
    "version": {
      "type": "integer",
      "minimum": 1
    },
    "description": {
      "type": "string",
      "description": "人类可读的能力描述"
    },
    "risk_level": {
      "type": "string",
      "enum": ["read_only", "low", "medium", "high", "critical"]
    },
    "reversible": {
      "type": "boolean",
      "description": "是否可以回滚"
    },
    "requires_confirmation": {
      "type": "boolean",
      "description": "是否需要用户确认"
    },
    "input_schema": {
      "type": "object",
      "description": "参数的 JSON Schema 定义"
    },
    "supported_platforms": {
      "type": "array",
      "items": {
        "type": "object",
        "required": ["os"],
        "properties": {
          "os": { "type": "string", "enum": ["android", "ios", "linux", "windows", "darwin", "browser"] },
          "arch": { "type": "array", "items": { "type": "string" } }
        }
      }
    },
    "bindings": {
      "type": "object",
      "description": "各平台的 Adapter 绑定，key 为 os 名",
      "additionalProperties": {
        "type": "object",
        "required": ["adapter", "method"],
        "properties": {
          "adapter": { "type": "string" },
          "method": { "type": "string" },
          "verifier": { "type": "string" },
          "rollback": { "type": "string" }
        }
      }
    },
    "success_criteria": {
      "type": "array",
      "items": {
        "type": "object",
        "properties": {
          "field": { "type": "string" },
          "equals_param": { "type": "string" },
          "equals_value": {},
          "contains": { "type": "string" }
        }
      }
    },
    "rollback": {
      "type": "object",
      "properties": {
        "strategy": { "type": "string", "enum": ["restore_previous_value", "compensating_action", "snapshot_restore", "none"] }
      }
    }
  }
}
```

### 2.2 Capability Rust 类型

```rust
/// 注册的能力定义
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Capability {
    pub id: String,
    pub version: u32,
    pub description: String,
    pub risk_level: RiskLevel,
    pub reversible: bool,
    pub requires_confirmation: bool,
    pub input_schema: serde_json::Value,
    pub supported_platforms: Vec<PlatformSupport>,
    pub bindings: HashMap<String, CapabilityBinding>,
    pub success_criteria: Option<Vec<SuccessCriterion>>,
    pub rollback: Option<RollbackConfig>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlatformSupport {
    pub os: Platform,
    pub arch: Option<Vec<String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CapabilityBinding {
    pub adapter: String,
    pub method: String,
    pub verifier: Option<String>,
    pub rollback: Option<String>,
}
```

## 3. Audit Schema

### 3.1 Audit JSON Schema

```json
{
  "$schema": "https://json-schema.org/draft/2020-12/schema",
  "$id": "https://ai-intent-binder/schemas/audit.schema.json",
  "title": "AuditRecord",
  "description": "每次 Intent 执行的完整审计记录",
  "type": "object",
  "required": [
    "audit_id",
    "intent_id",
    "actor",
    "intent_type",
    "target",
    "capability_id",
    "policy_decision",
    "parameters",
    "execution_result",
    "timestamp",
    "hash"
  ],
  "additionalProperties": false,
  "properties": {
    "audit_id": { "type": "string", "description": "审计记录唯一 ID" },
    "intent_id": { "type": "string" },
    "actor": {
      "type": "object",
      "required": ["user_id", "role"],
      "properties": {
        "user_id": { "type": "string" },
        "role": { "type": "string", "enum": ["user", "operator", "admin", "device_owner"] }
      }
    },
    "intent_type": { "type": "string" },
    "target": {
      "type": "object",
      "required": ["platform", "device_id"],
      "properties": {
        "platform": { "type": "string" },
        "device_id": { "type": "string" }
      }
    },
    "capability_id": { "type": "string" },
    "capability_version": { "type": "integer" },
    "policy_decision": { "type": "string", "enum": ["allow", "deny", "confirm", "error"] },
    "pre_state": { "type": "object" },
    "parameters": { "type": "object" },
    "execution_result": {
      "type": "object",
      "properties": {
        "ok": { "type": "boolean" },
        "message": { "type": "string" },
        "adapter": { "type": "string" },
        "duration_ms": { "type": "integer" }
      }
    },
    "verification_result": {
      "type": "object",
      "properties": {
        "ok": { "type": "boolean" },
        "observed": { "type": "object" }
      }
    },
    "rollback": {
      "type": "object",
      "properties": {
        "attempted": { "type": "boolean" },
        "success": { "type": "boolean" }
      }
    },
    "timestamp": { "type": "string", "format": "date-time" },
    "previous_hash": { "type": "string", "description": "前一条审计记录的 SHA-256 hash" },
    "hash": { "type": "string", "description": "本条审计记录的 SHA-256 hash" }
  }
}
```

### 3.2 Hash Chain 计算

```
hash_n = SHA-256(hash_{n-1} + serialize(record_data_n))
```

其中 `record_data_n` 是除 `hash` 字段外的完整 JSON（按 key 字母序排序）。

## 4. Adapter RPC 协议

### 4.1 Host Adapter Trait

```rust
/// 所有平台 Host Adapter 必须实现的统一接口
#[async_trait]
pub trait HostAdapter: Send + Sync {
    /// 描述当前 Host 环境信息
    async fn describe_host(&self) -> Result<HostInfo, AdapterError>;

    /// 列出本 Adapter 支持的所有 Capability ID
    async fn list_capabilities(&self) -> Result<Vec<String>, AdapterError>;

    /// 前置条件检查：判断当前环境是否能执行指定 Capability
    async fn precheck(
        &self,
        capability_id: &str,
        params: &serde_json::Value,
    ) -> Result<PrecheckResult, AdapterError>;

    /// 保存执行前快照（用于回滚）
    async fn snapshot(
        &self,
        capability_id: &str,
        params: &serde_json::Value,
    ) -> Result<Snapshot, AdapterError>;

    /// 干运行：模拟执行，不产生实际效果
    async fn dry_run(
        &self,
        capability_id: &str,
        params: &serde_json::Value,
    ) -> Result<DryRunResult, AdapterError>;

    /// 执行 Capability 对应的实际操作
    async fn execute(
        &self,
        capability_id: &str,
        params: &serde_json::Value,
    ) -> Result<ExecuteResult, AdapterError>;

    /// 验证执行结果是否符合预期
    async fn verify(
        &self,
        capability_id: &str,
        params: &serde_json::Value,
        result: &ExecuteResult,
    ) -> Result<VerifyResult, AdapterError>;

    /// 回滚操作：恢复到执行前状态
    async fn rollback(
        &self,
        capability_id: &str,
        snapshot: &Snapshot,
    ) -> Result<RollbackResult, AdapterError>;
}
```

### 4.2 请求/响应类型

```rust
/// Host 环境信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HostInfo {
    pub hostname: String,
    pub os: String,          // "linux" | "windows" | "darwin"
    pub arch: String,        // "x86_64" | "aarch64" | "arm64"
    pub os_version: String,
    pub adapter_version: String,
}

/// 预检查结果
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PrecheckResult {
    pub ok: bool,
    pub reason: Option<String>,
    pub warnings: Vec<String>,
}

/// 快照数据
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Snapshot {
    pub timestamp: String,
    pub capability_id: String,
    pub state: serde_json::Value,
}

/// 干运行结果
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DryRunResult {
    pub ok: bool,
    pub expected_effects: Vec<String>,
    pub warnings: Vec<String>,
}

/// 执行结果
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExecuteResult {
    pub ok: bool,
    pub message: Option<String>,
    pub output: Option<serde_json::Value>,
    pub duration_ms: u64,
}

/// 验证结果
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VerifyResult {
    pub ok: bool,
    pub observed: Option<serde_json::Value>,
    pub diff: Option<String>,
}

/// 回滚结果
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RollbackResult {
    pub ok: bool,
    pub message: Option<String>,
}
```

### 4.3 错误码

```rust
/// Adapter 错误类型
#[derive(Debug, Error)]
pub enum AdapterError {
    #[error("Capability '{0}' not found on this adapter")]
    CapabilityNotFound(String),

    #[error("Platform not supported: {0}")]
    PlatformNotSupported(String),

    #[error("Precondition not met: {0}")]
    PreconditionFailed(String),

    #[error("Execution failed: {0}")]
    ExecutionFailed(String),

    #[error("Verification failed: {0}")]
    VerificationFailed(String),

    #[error("Rollback failed: {0}")]
    RollbackFailed(String),

    #[error("Not reversible: capability '{0}' does not support rollback")]
    NotReversible(String),

    #[error("Confirmation required for capability '{0}'")]
    ConfirmationRequired(String),

    #[error("Permission denied: {0}")]
    PermissionDenied(String),

    #[error("Internal error: {0}")]
    Internal(String),

    #[error("Timeout: {0}")]
    Timeout(String),
}
```

### 4.4 JSON-RPC 传输格式

所有 Binder Core 与 Host Adapter 之间的通信使用 JSON-RPC 2.0：

**请求示例**：
```json
{
  "jsonrpc": "2.0",
  "id": 1,
  "method": "execute",
  "params": {
    "capability_id": "system.info.get",
    "params": {}
  }
}
```

**成功响应**：
```json
{
  "jsonrpc": "2.0",
  "id": 1,
  "result": {
    "ok": true,
    "output": {
      "hostname": "my-laptop",
      "os": "linux",
      "arch": "x86_64",
      "kernel": "6.1.0"
    },
    "duration_ms": 15
  }
}
```

**错误响应**：
```json
{
  "jsonrpc": "2.0",
  "id": 1,
  "error": {
    "code": -32000,
    "message": "Capability not found: unknown.action",
    "data": {
      "type": "CapabilityNotFound"
    }
  }
}
```