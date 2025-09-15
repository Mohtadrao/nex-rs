
use crate::error::{Result, Error};
use crate::prudp_packet_v1::PRUDPPacketV1;
use crate::prudp_packet::PRUDPPacket;
use crate::constants::prudp_packet_types::PRUDPPacketType;
use crate::prudp_v1_settings::PRUDPV1Settings;
use crate::sliding_window::SlidingWindow;
use crate::byte_stream_in::ByteStreamIn;
use std::net::SocketAddr;
use std::time::{Instant, Duration};
use tokio::net::UdpSocket;

/// Minimal PRUDP connection (v1 only) with a basic send path.
pub struct PRUDPConnection {
    pub socket: UdpSocket,
    pub remote: SocketAddr,
    pub settings: PRUDPV1Settings,
    pub session_key: Vec<u8>,
    pub window: SlidingWindow,
    pub session_id: u8,
    pub substream_id: u8,
    pub source_port: u8,
    pub dest_port: u8,
}

impl PRUDPConnection {
    pub async fn connect(bind: SocketAddr, remote: SocketAddr, settings: PRUDPV1Settings, session_key: Vec<u8>) -> Result<Self> {
        let socket = UdpSocket::bind(bind).await.map_err(Error::from)?;
        socket.connect(remote).await.map_err(Error::from)?;
        // Simplified seq start
        let window = SlidingWindow::new(64, 1, Duration::from_millis(500));
        Ok(Self {
            socket, remote, settings, session_key, window,
            session_id: 1, substream_id: 0, source_port: 0, dest_port: 0,
        })
    }

    /// Send a single DATA packet with current sequence and payload.
    pub async fn send_data(&mut self, payload: Vec<u8>) -> Result<u16> {
        let now = Instant::now();
        let seq = self.window.on_send(payload.clone(), now);

        let mut p = PRUDPPacketV1::default();
        p.base.version = 1;
        p.base.source_virtual_port = self.source_port;
        p.base.destination_virtual_port = self.dest_port;
        p.base.packet_type = PRUDPPacketType::Data as u16;
        p.base.flags = 0; // set as needed
        p.base.session_id = self.session_id;
        p.base.substream_id = self.substream_id;
        p.base.sequence_id = seq;
        p.base.payload = payload;
        // connection signature based on remote
        p.compute_connection_signature(&self.settings, self.remote);
        // packet signature
        p.compute_signature(&self.settings, &self.session_key);

        let bytes = p.to_bytes();
        self.socket.send(&bytes).await.map_err(Error::from)?;
        self.window.mark_sent(seq, now);
        Ok(seq)
    }

    /// Receive a single inbound datagram and decode as PRUDPv1.
    pub async fn recv_once(&self) -> Result<PRUDPPacketV1> {
        let mut buf = vec![0u8; 65536];
        let n = self.socket.recv(&mut buf).await.map_err(Error::from)?;
        let bs = ByteStreamIn::from_bytes(bytes::Bytes::from(buf[..n].to_vec()));
        let pkt = PRUDPPacketV1::decode(bs)?;
        Ok(pkt)
    }
}
