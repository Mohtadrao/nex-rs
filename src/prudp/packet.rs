
use bytes::{BufMut, BytesMut, Bytes};
use byteorder::{LittleEndian, ByteOrder};
use anyhow::{Result, Context};
use std::convert::TryFrom;

bitflags::bitflags! {
    pub struct Flags: u8 {
        const ACK = 0x01;
        const RELIABLE = 0x02;
        const NEED_ACK = 0x04;
        const HAS_SIZE = 0x08;
        const MULTI_ACK = 0x10;
        const SIGNED = 0x20;
    }
}

#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Version {
    V1 = 1,
}

#[repr(u16)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PacketType {
    Syn = 0x0,
    Connect = 0x1,
    Data = 0x2,
    Disconnect = 0x3,
    Ping = 0x4,
    Ack = 0x5,
    Nak = 0x6,
}

#[derive(Debug, Clone)]
pub struct PRUDPHeader {
    pub version: u8,
    pub packet_type: u16,
    pub flags: Flags,
    pub session_id: u16,
    pub sequence_id: u16,
    pub ack_id: u16,
    // optional payload length present if HAS_SIZE flag set
    pub payload_len: Option<usize>,
    // optional signature/multi ack fields omitted for brevity
}

#[derive(Debug, Clone)]
pub struct PRUDPPacket {
    pub header: PRUDPHeader,
    pub payload: Bytes,
    // Fragmentation fields: if fragment_count > 1 then this packet is a fragment
    pub fragment_id: Option<u32>,
    pub fragment_index: Option<u16>,
    pub fragment_count: Option<u16>,
}

impl PRUDPPacket {
    pub fn to_bytes(&self) -> Result<Vec<u8>> {
        let mut buf = BytesMut::with_capacity(128 + self.payload.len());
        // Simple header serialization:
        buf.put_u8(self.header.version);
        buf.put_u16_le(self.header.packet_type);
        buf.put_u8(self.header.flags.bits());
        buf.put_u16_le(self.header.session_id);
        buf.put_u16_le(self.header.sequence_id);
        buf.put_u16_le(self.header.ack_id);
        if let Some(len) = self.header.payload_len {
            buf.put_u32_le(len as u32);
        }
        // Fragment header: present if fragment_count is Some and >1
        if let Some(count) = self.fragment_count {
            // marker: 0xF0 as fragment marker
            buf.put_u8(0xF0);
            buf.put_u32_le(self.fragment_id.unwrap_or(0));
            buf.put_u16_le(self.fragment_index.unwrap_or(0));
            buf.put_u16_le(count);
        } else {
            // no fragment marker
            buf.put_u8(0x00);
        }
        buf.put_slice(&self.payload);
        Ok(buf.to_vec())
    }

    pub fn from_bytes(mut data: &[u8]) -> Result<Self> {
        use std::convert::TryInto;
        if data.len() < 10 { // minimal header size
            anyhow::bail!("packet too small");
        }
        let version = data[0];
        let packet_type = LittleEndian::read_u16(&data[1..3]);
        let flags = Flags::from_bits_truncate(data[3]);
        let session_id = LittleEndian::read_u16(&data[4..6]);
        let sequence_id = LittleEndian::read_u16(&data[6..8]);
        let ack_id = LittleEndian::read_u16(&data[8..10]);
        let mut offset = 10usize;
        let payload_len = if flags.contains(Flags::HAS_SIZE) {
            if data.len() < offset + 4 { anyhow::bail!("missing size"); }
            let v = LittleEndian::read_u32(&data[offset..offset+4]) as usize;
            offset += 4;
            Some(v)
        } else { None };

        let mut fragment_id = None;
        let mut fragment_index = None;
        let mut fragment_count = None;

        if data.len() > offset {
            let marker = data[offset];
            offset += 1;
            if marker == 0xF0 {
                if data.len() < offset + 8 { anyhow::bail!("incomplete fragment header"); }
                fragment_id = Some(LittleEndian::read_u32(&data[offset..offset+4]));
                fragment_index = Some(LittleEndian::read_u16(&data[offset+4..offset+6]));
                fragment_count = Some(LittleEndian::read_u16(&data[offset+6..offset+8]));
                offset += 8;
            } else {
                // no fragment header, marker was zero - proceed
            }
        }

        let payload = if let Some(len) = payload_len {
            if data.len() < offset + len { anyhow::bail!("payload truncated"); }
            Bytes::copy_from_slice(&data[offset..offset+len])
        } else {
            Bytes::copy_from_slice(&data[offset..])
        };

        Ok(PRUDPPacket {
            header: PRUDPHeader {
                version,
                packet_type,
                flags,
                session_id,
                sequence_id,
                ack_id,
                payload_len,
            },
            payload,
            fragment_id,
            fragment_index,
            fragment_count,
        })
    }
}
