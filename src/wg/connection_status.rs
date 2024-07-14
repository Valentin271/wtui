use std::{
    char,
    io::BufRead,
    process::{Command, Stdio},
};

use chrono::{DateTime, Local, TimeDelta, Utc};

use super::types::Byte;

/// The status of the wireguard connection.
///
/// Can be either disconnected or connected with data.
#[derive(Default)]
pub enum ConnectionStatus {
    Connected {
        /// The timestamp of the latest handshake
        latest_handshake: DateTime<Utc>,
        /// The number of bytes received through this connection/interface.
        bytes_received: Byte,
        /// The number of bytes sent through this connection/interface.
        bytes_sent: Byte,
    },
    #[default]
    Disconnected,
}

impl ConnectionStatus {
    /// Tells whether or not the status is connected.
    #[inline]
    #[must_use]
    pub fn is_connected(&self) -> bool {
        matches!(self, ConnectionStatus::Connected { .. })
    }

    /// Get the count of bytes received.
    ///
    /// [None] if the connection is disconnected.
    pub fn bytes_received(&self) -> Option<&Byte> {
        match self {
            ConnectionStatus::Connected { bytes_received, .. } => Some(bytes_received),
            ConnectionStatus::Disconnected => None,
        }
    }

    /// Get the count of bytes sent.
    ///
    /// [None] if the connection is disconnected.
    pub fn bytes_sent(&self) -> Option<&Byte> {
        match self {
            ConnectionStatus::Connected { bytes_sent, .. } => Some(bytes_sent),
            ConnectionStatus::Disconnected => None,
        }
    }

    /// Update this connection information from the given name.
    ///
    /// If anything fails inside of this method, the connection is considered disconnected.
    pub fn update(&mut self, connection_name: &str) {
        let result = Command::new("wg")
            .arg("show")
            .arg(connection_name)
            .arg("dump")
            .stderr(Stdio::null())
            .output();

        match result {
            Ok(res) if res.status.success() => {
                let Some(second_line) = res.stdout.lines().nth(1) else {
                    *self = ConnectionStatus::Disconnected;
                    return;
                };
                let second_line = second_line.expect("wg always returns valid utf-8");
                let data: Vec<_> = second_line.split(char::is_whitespace).collect();

                *self = ConnectionStatus::Connected {
                    latest_handshake: DateTime::from_timestamp(
                        data.get(4).unwrap_or(&"0").parse().unwrap(),
                        0,
                    )
                    .expect("wg always returns a valid timestamp"),
                    bytes_received: data.get(5).unwrap_or(&"0").parse().unwrap_or(0.into()),
                    bytes_sent: data.get(6).unwrap_or(&"0").parse().unwrap_or(0.into()),
                }
            }
            _ => *self = ConnectionStatus::Disconnected,
        }
    }

    /// Human readable representation of the elapsed time since the latest handshake.
    pub fn handshake_since(&self) -> String {
        use chrono::format::*;

        let latest_handshake = match self {
            ConnectionStatus::Connected {
                latest_handshake, ..
            } => latest_handshake,
            ConnectionStatus::Disconnected => return Default::default(),
        };

        let elapsed = Local::now().signed_duration_since(latest_handshake);
        let mut format = vec![];

        // show hours only if above an hour
        if elapsed >= TimeDelta::hours(1) {
            format.append(&mut vec![
                Item::Numeric(Numeric::Hour, Pad::None),
                Item::Literal("h, "),
            ]);
        }

        // show minutes only if above one minute
        if elapsed >= TimeDelta::minutes(1) {
            format.append(&mut vec![
                Item::Numeric(Numeric::Minute, Pad::None),
                Item::Literal("m"),
            ]);
        }

        // show seconds only if under an hour
        if elapsed < TimeDelta::hours(1) {
            format.append(&mut vec![
                if !format.is_empty() {
                    Item::Literal(", ")
                } else {
                    Item::Literal("")
                },
                Item::Numeric(Numeric::Second, Pad::None),
                Item::Literal("s"),
            ]);
        }

        format.push(Item::Literal(" ago"));

        DateTime::from_timestamp(elapsed.num_seconds(), 0)
            .expect("elapsed is always valid and positive")
            .format_with_items(format.iter())
            .to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_connected() {
        let status = ConnectionStatus::Connected {
            latest_handshake: DateTime::from_timestamp(0, 0).expect("0 is a valid timestamp"),
            bytes_received: 0.into(),
            bytes_sent: 0.into(),
        };

        let status2 = ConnectionStatus::Connected {
            latest_handshake: DateTime::from_timestamp(1720868567, 0)
                .expect("1720868567 is a valid timestamp"),
            bytes_received: 1286.into(),
            bytes_sent: 1645.into(),
        };

        assert!(status.is_connected());
        assert!(status2.is_connected());
    }

    mod latest_handshake {
        use super::*;

        #[test]
        fn one_second_ago() {
            let status = ConnectionStatus::Connected {
                latest_handshake: (Local::now() - TimeDelta::seconds(1)).into(),
                bytes_received: 0.into(),
                bytes_sent: 0.into(),
            };

            assert_eq!(status.handshake_since(), "1s ago")
        }

        #[test]
        fn one_minute_ago() {
            let status = ConnectionStatus::Connected {
                latest_handshake: (Local::now() - TimeDelta::seconds(61)).into(),
                bytes_received: 0.into(),
                bytes_sent: 0.into(),
            };

            assert_eq!(status.handshake_since(), "1m, 1s ago")
        }

        #[test]
        fn one_hour_ago() {
            let status = ConnectionStatus::Connected {
                latest_handshake: (Local::now() - TimeDelta::minutes(61)).into(),
                bytes_received: 0.into(),
                bytes_sent: 0.into(),
            };

            assert_eq!(status.handshake_since(), "1h, 1m ago")
        }
    }
}
