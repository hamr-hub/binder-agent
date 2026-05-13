//! binderctl: Binder CLI 控制工具
//!
//! 子命令:
//!   binderctl chat "<自然语言>"   - 发送自然语言指令
//!   binderctl intent --type=...  - 直接发送 Intent JSON
//!   binderctl health             - 检查 binderd 健康状态
//!   binderctl capabilities list  - 列出所有 capability
//!   binderctl audit list         - 查看审计日志
//!   binderctl adapter status     - 查看 Adapter 状态

use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "binderctl")]
#[command(about = "AI Intent Binder CLI 控制工具")]
struct Cli {
    #[command(subcommand)]
    command: Commands,

    /// 目标平台
    #[arg(short, long, default_value = "linux")]
    platform: String,
}

#[derive(Subcommand)]
enum Commands {
    /// 发送自然语言指令
    Chat {
        /// 自然语言输入
        input: String,
    },
    /// 直接发送 Intent JSON
    Intent {
        /// Intent JSON 字符串
        json: String,
    },
    /// 检查 binderd 健康状态
    Health,
    /// Capability 管理
    Capabilities {
        #[command(subcommand)]
        action: CapabilityAction,
    },
    /// 审计日志管理
    Audit {
        #[command(subcommand)]
        action: AuditAction,
    },
    /// Adapter 状态
    Adapter {
        #[command(subcommand)]
        action: AdapterAction,
    },
}

#[derive(Subcommand)]
enum CapabilityAction {
    /// 列出所有可用 Capability
    List,
    /// 查看指定 Capability 详情
    Get { id: String },
}

#[derive(Subcommand)]
enum AuditAction {
    /// 列出最近审计记录 (默认 20 条)
    List { limit: Option<usize> },
}

#[derive(Subcommand)]
enum AdapterAction {
    /// 查看 Adapter 状态
    Status,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "binderctl=info".into()),
        )
        .init();

    let cli = Cli::parse();

    match cli.command {
        Commands::Chat { input } => {
            println!("📝 Processing: \"{}\"", input);
            // TODO: Phase 1.4 - 调用 binderd JSON-RPC 发送 chat 请求
            println!("⚠️  LLM Agent not yet connected in Phase 1 MVP skeleton.");
        }
        Commands::Intent { json } => {
            println!("📋 Raw Intent: {}", json);
            // TODO: Phase 1.4 - 调用 binderd JSON-RPC 发送 intent
            println!("⚠️  Direct intent execution not yet implemented.");
        }
        Commands::Health => {
            println!("🏥 Checking binderd health...");
            // TODO: 调用 binderd health check
            println!("⚠️  Health check endpoint not yet implemented.");
        }
        Commands::Capabilities { action } => match action {
            CapabilityAction::List => {
                println!("📦 Available Capabilities:");
                // TODO: 从 Registry 获取
                println!("⚠️  Capability list not yet implemented.");
            }
            CapabilityAction::Get { id } => {
                println!("📦 Capability: {}", id);
                // TODO: 从 Registry 获取
                println!("⚠️  Capability detail not yet implemented.");
            }
        },
        Commands::Audit { action } => match action {
            AuditAction::List { limit } => {
                let limit = limit.unwrap_or(20);
                println!("📜 Recent {} audit records:", limit);
                // TODO: 从 AuditLogger 获取
                println!("⚠️  Audit list not yet implemented.");
            }
        },
        Commands::Adapter { action } => match action {
            AdapterAction::Status => {
                println!("🔌 Adapter Status:");
                // TODO: 检查各 Adapter 状态
                println!("⚠️  Adapter status not yet implemented.");
            }
        },
    }

    Ok(())
}