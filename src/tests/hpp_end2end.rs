
#[cfg(test)]
mod hpp_end2end {
    use crate::hpp::dispatcher::SimpleHandler;
    use crate::hpp::server::HppServer;
    use crate::hpp::client::HppClient;
    use crate::hpp::packet::HppPacket;
    use bytes::Bytes;
    use std::sync::Arc;
    use tokio::time::{sleep, Duration};

    #[tokio::test(flavor = "multi_thread", worker_threads = 2)]
    async fn test_hpp_client_server_roundtrip() {
        // bind to fixed port 6100 for test (if taken, test will fail)
        let addr = "127.0.0.1:6100";
        let handler = Arc::new(SimpleHandler::new());
        handler.register(42, |b: Bytes| async move {
            // respond with method id + body prefixed by 0x01
            let mut v = vec![1u8];
            v.extend_from_slice(&b);
            Ok(Bytes::from(v))
        });
        let server = HppServer::bind(addr, handler.clone()).await.expect("bind");
        // run server in background
        let srv_task = tokio::spawn(async move {
            let _ = server.run().await;
        });
        // give server a moment to start
        sleep(Duration::from_millis(100)).await;

        // Create client and connect
        let mut client = HppClient::connect(addr).await.expect("connect");

        // Build RMC request: method id 42 + body [9,9,9]
        let mut body = vec![];
        body.extend_from_slice(&42u32.to_le_bytes());
        body.extend_from_slice(&[9u8,9u8,9u8]);
        let payload = Bytes::from(body);
        let resp = client.send_and_receive(payload).await.expect("send_receive");
        // response should be the method handler output (prefixed by 1)
        assert_eq!(resp[0], 1u8);
        assert_eq!(resp.len(), 4);

        // cleanup: abort server
        srv_task.abort();
    }
}
