use std::net::SocketAddr;
use crate::Result;

pub struct VirtualPort {
    pub id: u16,
    pub bound_addr: Option<SocketAddr>,
}

impl VirtualPort {
    pub fn new(id: u16) -> Self { Self { id, bound_addr: None } }
    pub fn bind(&mut self, addr: SocketAddr) -> Result<()> {
        self.bound_addr = Some(addr);
        Ok(())
    }
}
