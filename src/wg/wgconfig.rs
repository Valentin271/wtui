use std::{net::IpAddr};

use super::{Interface, Peer};

pub struct WgConfig {
    pub interface: Interface,
    pub peer: Peer,
}

impl From<&str> for WgConfig {
    fn from(config: &str) -> Self {
        // interface
        let mut private_key = String::new();
        let mut address = String::new();
        let mut mtu = 1500;
        let mut dns: Vec<IpAddr> = Vec::new();

        // peer
        let mut peer_pubkey = String::new();
        let mut allowed_ips: Vec<String> = Vec::new();
        let mut endpoint = String::new();

        for line in config.split('\n') {
            if let Some(pair) = line.trim().split_once('=') {
                let value = pair.1.trim();
                match pair.0.trim() {
                    "PrivateKey" => private_key = value.into(),
                    "Address" => address = value.into(),
                    "MTU" => mtu = value.parse().unwrap_or(mtu),
                    "DNS" => {
                        dns = value
                            .split(',')
                            .filter_map(|ip| ip.trim().parse().ok())
                            .collect();
                    }
                    "PublicKey" => peer_pubkey = value.into(),
                    "AllowedIPs" => {
                        allowed_ips = value.split(',').map(|ip| ip.trim().into()).collect();
                    }
                    "Endpoint" => endpoint = value.into(),
                    _ => {}
                };
            }
        }

        Self {
            interface: Interface::new(&private_key, &address)
                .set_dns(dns)
                .set_mtu(mtu),
            peer: Peer::new(&peer_pubkey, allowed_ips, endpoint),
        }
    }
}

#[cfg(test)]
mod tests {
    use std::{
        net::{IpAddr, Ipv4Addr},
    };

    use crate::wg::types::{
        key::{Private, Public},
        Key,
    };

    use super::*;

    #[test]
    fn from_string_full_ipv4() {
        let data = r#"[Interface]
PrivateKey = oMVUWFwDf+20fIfeRUe7c0rlUKSYnHk2K0y2920SX1c=
Address = 192.168.5.2
MTU = 1420
DNS = 9.9.9.9, 1.1.1.1

[Peer]
PublicKey = 60TUAvOo+Wi4SCyir581cCyBx4wIcHtrIrUgBv/iqRM=
AllowedIPs = 192.168.5.0/24, 192.168.6.0/24
Endpoint = vpn.example.com:51820"#;

        let config = WgConfig::from(data);

        // interface
        assert_eq!(
            config.interface.private_key(),
            &Key::<Private>::from("oMVUWFwDf+20fIfeRUe7c0rlUKSYnHk2K0y2920SX1c=")
        );
        assert_eq!(
            config.interface.pubkey(),
            &Key::<Public>::from("CLjhKsWxLTR+N5fs/jMYqVXL7xwtuEzufupX82c7LCs=")
        );
        assert_eq!(config.interface.address(), "192.168.5.2");
        assert_eq!(config.interface.mtu(), 1420);
        assert_eq!(
            config.interface.dns(),
            vec![
                IpAddr::V4(Ipv4Addr::new(9, 9, 9, 9)),
                IpAddr::V4(Ipv4Addr::new(1, 1, 1, 1)),
            ]
        );

        // peer
        assert_eq!(
            config.peer.pubkey(),
            &Key::<Public>::from("60TUAvOo+Wi4SCyir581cCyBx4wIcHtrIrUgBv/iqRM=")
        );
        assert_eq!(
            config.peer.allowed_ips(),
            vec!["192.168.5.0/24", "192.168.6.0/24"]
        );
        assert_eq!(config.peer.endpoint(), "vpn.example.com:51820");
    }

    #[test]
    fn from_string_ipv6() {
        let data = r#"[Interface]
PrivateKey = oMVUWFwDf+20fIfeRUe7c0rlUKSYnHk2K0y2920SX1c=
Address = 2001:DB8::1

[Peer]
PublicKey = 60TUAvOo+Wi4SCyir581cCyBx4wIcHtrIrUgBv/iqRM=
AllowedIPs = ::/0
Endpoint = vpn.example.com:51820"#;

        let config = WgConfig::from(data);

        // interface
        assert_eq!(
            config.interface.private_key(),
            &Key::<Private>::from("oMVUWFwDf+20fIfeRUe7c0rlUKSYnHk2K0y2920SX1c=")
        );
        assert_eq!(
            config.interface.pubkey(),
            &Key::<Public>::from("CLjhKsWxLTR+N5fs/jMYqVXL7xwtuEzufupX82c7LCs=")
        );
        assert_eq!(config.interface.address(), "2001:DB8::1");
        assert_eq!(config.interface.mtu(), 1500);
        assert_eq!(config.interface.dns(), Vec::<IpAddr>::new());

        // peer
        assert_eq!(
            config.peer.pubkey(),
            &Key::<Public>::from("60TUAvOo+Wi4SCyir581cCyBx4wIcHtrIrUgBv/iqRM=")
        );
        assert_eq!(config.peer.allowed_ips(), vec!["::/0"]);
        assert_eq!(config.peer.endpoint(), "vpn.example.com:51820");
    }
}
