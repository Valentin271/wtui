//! Everything related to wireguard.
//!
//! This is mainly the parser and the data representation of a wireguard config.

mod connection_status;
mod interface;
mod peer;
pub mod types;
mod wgconfig;

pub use connection_status::*;
pub use interface::*;
pub use peer::*;
pub use wgconfig::*;
