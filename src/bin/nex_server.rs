// use std::sync::Arc;
// use tokio::signal;
// use tracing_subscriber;
// use std::net::SocketAddr;

// #[tokio::main(flavor = "multi_thread", worker_threads = 4)]
// async fn main() -> Result<(), Box<dyn std::error::Error>> {
//     tracing_subscriber::fmt::init();
//     // start HPP server on 127.0.0.1:6000 and PRUDP server on 0.0.0.0:5000
//     let hpp_addr = "127.0.0.1:6000";
//     let prudp_addr = "0.0.0.0:5000";
//     // create HPP handler
//     let handler = Arc::new(crate::hpp::dispatcher::SimpleHandler::new());
//     // register a sample method 1
//     handler.register(1, |b: bytes::Bytes| async move {
//         tracing::info!(len = b.len(), "sample handler invoked");
//         let mut v = vec![0u8];
//         v.extend_from_slice(&b);
//         Ok(bytes::Bytes::from(v))
//     });
//     // start servers
//     let hpp_server = crate::hpp::server::HppServer::bind(hpp_addr, handler.clone()).await?;
//     let prudp_server = crate::prudp::server::PRUDPServer::bind(prudp_addr).await?;
//     // run both servers concurrently
//     let hpp_task = tokio::spawn(async move {
//         if let Err(e) = hpp_server.run().await {
//             tracing::error!("HPP server error: {:?}", e);
//         }
//     });
//     let prudp_task = tokio::spawn(async move {
//         if let Err(e) = prudp_server.run().await {
//             tracing::error!("PRUDP server error: {:?}", e);
//         }
//     });
//     // wait for ctrl-c
//     signal::ctrl_c().await?;
//     tracing::info!("Shutting down");
//     // cancel tasks
//     hpp_task.abort();
//     prudp_task.abort();
//     Ok(())
// }
use std::sync::Arc;
use tokio::signal;
use tracing_subscriber;

use nex::hpp::dispatcher::SimpleHandler;
use nex::hpp::server::HppServer;
use nex::prudp::server::PRUDPServer;

#[tokio::main(flavor = "multi_thread", worker_threads = 4)]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt::init();

    // Addresses
    let hpp_addr = "127.0.0.1:6000";
    let prudp_addr = "0.0.0.0:5000";

    // HPP handler and a sample method
    let handler = Arc::new(SimpleHandler::new());
    handler.register(1, |b: bytes::Bytes| async move {
        tracing::info!(len = b.len(), "sample handler invoked");
        let mut v = vec![0u8];
        v.extend_from_slice(&b);
        Ok(bytes::Bytes::from(v))
    });

    // Bind servers
    let hpp_server = HppServer::bind(hpp_addr, handler.clone()).await?;
    let prudp_server = PRUDPServer::bind(prudp_addr).await?;

    // Run both concurrently
    let hpp_task = tokio::spawn(async move {
        if let Err(e) = hpp_server.run().await {
            tracing::error!("HPP server error: {:?}", e);
        }
    });
    let prudp_task = tokio::spawn(async move {
        if let Err(e) = prudp_server.run().await {
            tracing::error!("PRUDP server error: {:?}", e);
        }
    });

    // Wait for Ctrl+C then shut down
    signal::ctrl_c().await?;
    tracing::info!("Shutting down");
    hpp_task.abort();
    prudp_task.abort();

    Ok(())
}
