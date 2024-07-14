use super::types::{key::Public, Key};

pub struct Peer {
    pubkey: Key<Public>,
    allowed_ips: Vec<String>,
    /// Actual hostname or IP
    endpoint: String,
}

impl Peer {
    pub fn new(public_key: &str, allowed_ips: Vec<String>, endpoint: String) -> Self {
        Self {
            pubkey: public_key.into(),
            allowed_ips,
            endpoint,
        }
    }

    pub fn allowed_ips(&self) -> &[String] {
        &self.allowed_ips
    }

    pub fn endpoint(&self) -> &str {
        &self.endpoint
    }

    pub fn pubkey(&self) -> &Key<Public> {
        &self.pubkey
    }
}
