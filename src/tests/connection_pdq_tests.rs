
#[cfg(test)]
mod connection_pdq_tests {
    use crate::prudp::connection::Connection;
    use crate::prudp::packet::{PRUDPHeader, PRUDPPacket, Version, Flags};
    use bytes::Bytes;
    use std::net::SocketAddr;

    #[tokio::test(flavor = "multi_thread")]
    async fn test_connect_packet_enqueue() {
        // Create a fake connection with dummy socket -- use UdpSocket::bind to localhost ephemeral port
        let sock = tokio::net::UdpSocket::bind("127.0.0.1:0").await.unwrap();
        let addr: SocketAddr = "127.0.0.1:9999".parse().unwrap();
        let arc_sock = std::sync::Arc::new(sock);
        let conn = Connection::new(arc_sock.clone(), addr, tokio::sync::mpsc::channel(8).0).await;
        // Build a CONNECT packet (seq 1)
        let header = PRUDPHeader {
            version: Version::V0,
            src: 0,
            dst: 0,
            flags: Flags::NEW_CONN,
            session_id: 0,
            seq: 1,
            payload_len: Some(0),
            signature: None,
            multi_ack_mask: None,
            v1_extra: None,
        };
        let pkt = PRUDPPacket { header, payload: Bytes::from(vec![]) };
        // handle packet
        conn.handle_incoming_packet(pkt).await.unwrap();
        // ensure dispatch queue has queued seq 1
        let dq = conn.dispatch_queue.lock().await;
        assert!(dq.queue.contains_key(&1u16));
    }
}
