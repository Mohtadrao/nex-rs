
use rand::Rng;
use bytes::Bytes;

/// Simple Kerberos-like ticket stub.
pub struct Ticket {
    pub session_key: [u8; 16],
    pub user_id: u32,
}

pub struct Kerberos;

impl Kerberos {
    pub fn issue_ticket(user_id: u32) -> Ticket {
        let mut rng = rand::thread_rng();
        let mut key = [0u8; 16];
        rng.fill(&mut key);
        Ticket { session_key: key, user_id }
    }

    pub fn encrypt(ticket: &Ticket, data: &[u8]) -> Bytes {
        // placeholder: xor with session key repeated
        let mut out = Vec::with_capacity(data.len());
        for (i, b) in data.iter().enumerate() {
            out.push(b ^ ticket.session_key[i % ticket.session_key.len()]);
        }
        Bytes::from(out)
    }

    pub fn decrypt(ticket: &Ticket, data: &[u8]) -> Bytes {
        Self::encrypt(ticket, data)
    }
}
