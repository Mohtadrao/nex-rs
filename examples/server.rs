
use nex::rv::RVManager;
use std::sync::Arc;
use tokio::net::UdpSocket;
use tracing::info;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt::init();
    let rv = nex::rv::RVManager::new();
    info!("starting basic example server (PRUDP-based)");
    // Placeholder: bind socket and accept incoming PRUDP connections
    Ok(())
}
