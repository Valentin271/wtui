//! A single VPN connection
use std::{
    io::{self},
    process::{Command, Stdio},
};

use ratatui::{prelude::*, style::Styled, widgets::Row};

use crate::wg::{
    types::{key::Public, Key},
    ConnectionStatus, WgConfig,
};

pub struct Connection {
    /// Config file name.
    ///
    /// Also the interface name.
    name: String,
    /// Whether this VPN connection is active or not.
    status: ConnectionStatus,
    config: WgConfig,
}

impl Connection {
    pub fn new(name: &str, config: WgConfig) -> Self {
        Self {
            name: name.to_string(),
            status: Default::default(),
            config,
        }
    }

    pub fn update(&mut self) {
        self.status.update(&self.name);
    }

    /// Connects to this connection.
    ///
    /// Does nothing and return [Ok] if the connection is already connected.
    pub fn connect(&self) -> io::Result<()> {
        if self.status.is_connected() {
            return Ok(());
        }

        Command::new("wg-quick")
            .arg("up")
            .arg(&self.name)
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .spawn()
            .map(|_| ())
    }

    /// Disconnects from this connection.
    ///
    /// Does nothing and return [Ok] if the connection is already disconnected.
    pub fn disconnect(&self) -> io::Result<()> {
        if !self.status.is_connected() {
            return Ok(());
        }

        Command::new("wg-quick")
            .arg("down")
            .arg(&self.name)
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .spawn()
            .map(|_| ())
    }

    pub fn pubkey(&self) -> &Key<Public> {
        self.config.interface.pubkey()
    }
}

impl<'a> From<&Connection> for Row<'a> {
    fn from(con: &Connection) -> Self {
        let row_height = con
            .config
            .peer
            .allowed_ips()
            .len()
            .max(con.config.interface.dns().len()) as u16;

        Self::new([
            con.name.clone().set_style(Style::new().bold()).into(),
            con.config.interface.address().to_string().into(),
            con.config.interface.mtu().to_string().into(),
            con.status
                .bytes_received()
                .map(Text::from)
                .unwrap_or_default(),
            con.status.bytes_sent().map(Text::from).unwrap_or_default(),
            con.config.peer.endpoint().to_string().into(),
            con.config.peer.allowed_ips().join("\n").into(),
            con.status.handshake_since().into(),
            con.config.interface.pubkey().truncated().into(),
            Text::from(
                con.config
                    .interface
                    .dns()
                    .iter()
                    .map(|ip| ip.to_string())
                    .collect::<Vec<_>>()
                    .join("\n"),
            ),
        ])
        .height(row_height)
        .set_style(if !con.status.is_connected() {
            Style::new().add_modifier(Modifier::DIM)
        } else {
            Style::new().green()
        })
    }
}
