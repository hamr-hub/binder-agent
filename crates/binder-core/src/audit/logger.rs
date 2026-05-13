//! SQLite Audit Logger：append-only 审计日志写入
//!
//! 使用 hash chain 保证完整性：每条记录的 hash = SHA-256(prev_hash + record_data)

use binder_schemas::audit::{AuditRecord, HashChain};
use rusqlite::Connection;
use std::sync::Mutex;

/// SQLite Audit Logger
pub struct AuditLogger {
    conn: Mutex<Connection>,
}

impl AuditLogger {
    /// 创建新的 Audit Logger，初始化数据库表
    pub fn new(db_path: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let conn = Connection::open(db_path)?;

        conn.execute_batch(
            "CREATE TABLE IF NOT EXISTS audit_log (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                audit_id TEXT NOT NULL UNIQUE,
                intent_id TEXT NOT NULL,
                user_id TEXT NOT NULL,
                role TEXT NOT NULL,
                intent_type TEXT NOT NULL,
                platform TEXT NOT NULL,
                device_id TEXT NOT NULL,
                capability_id TEXT NOT NULL,
                capability_version INTEGER,
                policy_decision TEXT NOT NULL,
                policy_reason TEXT,
                execution_ok INTEGER NOT NULL,
                execution_message TEXT,
                execution_adapter TEXT,
                execution_duration_ms INTEGER,
                verification_ok INTEGER,
                verification_diff TEXT,
                rollback_attempted INTEGER DEFAULT 0,
                rollback_success INTEGER,
                timestamp TEXT NOT NULL,
                previous_hash TEXT,
                hash TEXT NOT NULL
            );

            CREATE INDEX IF NOT EXISTS idx_audit_intent_id ON audit_log(intent_id);
            CREATE INDEX IF NOT EXISTS idx_audit_user_id ON audit_log(user_id);
            CREATE INDEX IF NOT EXISTS idx_audit_timestamp ON audit_log(timestamp);",
        )?;

        Ok(Self {
            conn: Mutex::new(conn),
        })
    }

    /// 写入一条审计记录
    pub fn write(&self, record: &AuditRecord) -> Result<(), Box<dyn std::error::Error>> {
        let conn = self.conn.lock().unwrap();

        // 获取上一条记录的 hash 作为 previous_hash
        let prev_hash: Option<String> = conn
            .query_row(
                "SELECT hash FROM audit_log ORDER BY id DESC LIMIT 1",
                [],
                |row| row.get(0),
            )
            .ok();

        // 计算本条记录的 hash
        let hash = HashChain::compute_hash(prev_hash.as_deref(), record)?;

        conn.execute(
            "INSERT INTO audit_log
            (audit_id, intent_id, user_id, role, intent_type, platform, device_id,
             capability_id, capability_version, policy_decision, policy_reason,
             execution_ok, execution_message, execution_adapter, execution_duration_ms,
             verification_ok, verification_diff,
             rollback_attempted, rollback_success,
             timestamp, previous_hash, hash)
            VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11,
                    ?12, ?13, ?14, ?15, ?16, ?17,
                    ?18, ?19, ?20, ?21, ?22)",
            rusqlite::params![
                record.audit_id,
                record.intent_id,
                record.actor.user_id,
                format!("{:?}", record.actor.role).to_lowercase(),
                record.intent_type,
                record.target.platform.to_string(),
                record.target.device_id,
                record.capability_id,
                record.capability_version,
                format!("{:?}", record.policy_decision).to_lowercase(),
                record.policy_reason,
                record.execution_result.ok as i32,
                record.execution_result.message,
                record.execution_result.adapter,
                record.execution_result.duration_ms,
                record.verification_result.as_ref().map(|v| v.ok as i32),
                record.verification_result.as_ref().and_then(|v| v.diff.clone()),
                record.rollback.as_ref().map(|r| r.attempted as i32).unwrap_or(0),
                record.rollback.as_ref().and_then(|r| r.success.map(|s| s as i32)),
                record.timestamp.to_rfc3339(),
                prev_hash,
                hash,
            ],
        )?;

        Ok(())
    }

    /// 列出最近 N 条审计记录
    pub fn list_recent(&self, limit: usize) -> Result<Vec<AuditRecord>, Box<dyn std::error::Error>> {
        let conn = self.conn.lock().unwrap();
        let mut stmt = conn.prepare(
            "SELECT audit_id, intent_id, user_id, role, intent_type, platform, device_id,
                    capability_id, capability_version, policy_decision, policy_reason,
                    execution_ok, execution_message, execution_adapter, execution_duration_ms,
                    verification_ok, verification_diff,
                    rollback_attempted, rollback_success,
                    timestamp, previous_hash, hash
             FROM audit_log ORDER BY id DESC LIMIT ?1",
        )?;

        let records = stmt.query_map([limit], |row| {
            Ok(AuditRecord {
                audit_id: row.get(0)?,
                intent_id: row.get(1)?,
                actor: binder_schemas::audit::Actor {
                    user_id: row.get(2)?,
                    role: {
                        let role_str: String = row.get(3)?;
                        serde_json::from_str(&format!("\"{}\"", role_str))
                            .unwrap_or(binder_schemas::Role::User)
                    },
                },
                intent_type: row.get(4)?,
                target: binder_schemas::audit::AuditTarget {
                    platform: {
                        let p: String = row.get(5)?;
                        serde_json::from_str(&format!("\"{}\"", p))
                            .unwrap_or(binder_schemas::Platform::Linux)
                    },
                    device_id: row.get(6)?,
                },
                capability_id: row.get(7)?,
                capability_version: row.get(8)?,
                policy_decision: {
                    let pd: String = row.get(9)?;
                    serde_json::from_str(&format!("\"{}\"", pd))
                        .unwrap_or(binder_schemas::audit::PolicyDecision::Error)
                },
                policy_reason: row.get(10)?,
                parameters: serde_json::Value::Null,
                pre_state: None,
                execution_result: binder_schemas::audit::ExecutionResult {
                    ok: row.get::<_, i32>(11)? != 0,
                    message: row.get(12)?,
                    adapter: row.get(13)?,
                    duration_ms: row.get(14)?,
                },
                verification_result: {
                    let verif_ok: Option<i32> = row.get(15)?;
                    let diff_val: Option<String> = row.get(16)?;
                    verif_ok.map(|ok| binder_schemas::audit::VerificationResult {
                        ok: ok != 0,
                        observed: None,
                        diff: diff_val,
                    })
                },
                rollback: {
                    let attempted: i32 = row.get(17)?;
                    let success: Option<i32> = row.get(18)?;
                    Some(binder_schemas::audit::RollbackResult {
                        attempted: attempted != 0,
                        success: success.map(|s| s != 0),
                        message: None,
                    })
                },
                timestamp: {
                    let ts: String = row.get(19)?;
                    chrono::DateTime::parse_from_rfc3339(&ts)
                        .unwrap()
                        .with_timezone(&chrono::Utc)
                },
                previous_hash: row.get(20)?,
                hash: row.get(21)?,
            })
        })?;

        let mut result = Vec::new();
        for record in records {
            result.push(record?);
        }
        Ok(result)
    }
}