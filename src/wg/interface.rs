use std::net::IpAddr;

use super::types::{
    key::{Private, Public},
    Key,
};

/// The interface section of a wireguard configuration.
pub struct Interface {
    private_key: Key<Private>,
    pubkey: Key<Public>,
    address: String,
    mtu: u16,
    dns: Vec<IpAddr>,
}

impl Interface {
    pub fn new(private_key: &str, address: &str) -> Self {
        let private_key: Key<Private> = private_key.into();

        Self {
            private_key: private_key.clone(),
            pubkey: private_key.fetch_pubkey().expect("Failed to get pubkey"),
            address: address.into(),
            mtu: 1500,
            dns: Vec::new(),
        }
    }

    pub fn address(&self) -> &str {
        &self.address
    }

    pub fn mtu(&self) -> u16 {
        self.mtu
    }

    pub fn dns(&self) -> &[IpAddr] {
        &self.dns
    }

    pub fn set_dns(mut self, dns: Vec<IpAddr>) -> Self {
        self.dns = dns;
        self
    }

    pub fn set_mtu(mut self, mtu: u16) -> Self {
        self.mtu = mtu;
        self
    }

    pub fn pubkey(&self) -> &Key<Public> {
        &self.pubkey
    }

    pub fn private_key(&self) -> &Key<Private> {
        &self.private_key
    }
}
