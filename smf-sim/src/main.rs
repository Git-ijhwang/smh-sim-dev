pub mod config;
pub mod transport;
pub mod state;

use std::path::PathBuf;
use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "smf-sim")]
#[command(about = "SMF PFCP Simulator for upf_edge testing")]
struct Cli {
    ///설정 파일 경로
    #[arg(short, long, default_value = "configs/sim-default.toml")]
    config: PathBuf,

    /// 로그 레벨
    #[arg(short, long, default_value = "info")]
    log_level: String,

    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    ///시나리오 실행
    Run {
        #[arg(short, long, default_value_t = 1)]
        scenario: u8,

        #[arg(short, long, default_value_t = 3)]
        num_ues: u32,
    },
    /// 단일 메시지 전송
    Send {
        #[command(subcommand)]
        message: SingleMessage,
    },
}

#[derive(Subcommand)]
enum SingleMessage {
    ///Heartbeat Request
    HeartBeat,
    /// Association Setup Request
    AssociationSetup,
}

#[tokio::main]
async fn main() -> anyhow::Result<()>
{
    let cli = Cli::parse();

    tracing_subscriber::fmt()
        .with_env_filter(&cli.log_level)
        .with_target(false)
        .init();

    let content = tokio::fs::read_to_string(&cli.config).await?;
    let config: config::SimConfig = toml::from_str(&content)?;

    tracing::info!("config loaded from{}", cli.config.display());
    tracing::info!("UPF target: {}:{}", config.network.upf_n4_addr, config.network.upf_n4_port);

    match cli.command {
        Commands::Run { scenario, num_ues } => {
            tracing::info!("Running scenario {} with {} UEs", scenario, num_ues);
            tracing::warn!("Not impremented yet");
        },

        Commands::Send { message } => match message {
            SingleMessage::HeartBeat => {
                tracing::info!("Sending Heartbeat (not implemented yet");
            }
            SingleMessage::AssociationSetup => {
                tracing::info!("Sending Association Setup (not implemented yet");
            }
        },
    }

    Ok(())

}
