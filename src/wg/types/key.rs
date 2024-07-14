use std::{
    io::{self, Write},
    marker::PhantomData,
    process::{Command, Stdio},
};

#[derive(Clone, Debug, PartialEq)]
pub struct Key<T: KeyType>(String, PhantomData<T>);

impl<T: KeyType> Key<T> {
    /// Gets the key with the middle part truncated.
    ///
    /// Get the 4 first and 4 last chars of the key, separated by `..`.
    /// This is meant for display purposes.
    pub fn truncated(&self) -> String {
        format!(
            "{}..{}",
            self.0.get(..4).unwrap_or_default(),
            self.0.get(self.0.len() - 4..).unwrap_or_default(),
        )
    }
}

impl Key<Private> {
    /// Gets the public key from the private key using `wg` command.
    pub(crate) fn fetch_pubkey(&self) -> io::Result<Key<Public>> {
        let mut child = Command::new("wg")
            .arg("pubkey")
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::null())
            .spawn()?;

        child
            .stdin
            .take()
            .expect("Failed to get stdin")
            .write_all(self.0.as_bytes())?;

        let output = child.wait_with_output()?;

        String::from_utf8(output.stdout)
            .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))
            .map(Key::from)
    }
}

impl<T: KeyType> From<&str> for Key<T> {
    fn from(value: &str) -> Self {
        Self(value.trim().to_string(), Default::default())
    }
}

impl<T: KeyType> From<String> for Key<T> {
    fn from(value: String) -> Self {
        Self::from(value.as_str())
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Private;
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Public;

pub trait KeyType: Clone + private::Sealed {}

impl KeyType for Private {}
impl KeyType for Public {}

mod private {
    use super::*;

    pub trait Sealed {}
    impl Sealed for Private {}
    impl Sealed for Public {}
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn trim_key() {
        // from &str
        assert_eq!(
            Key::<Public>::from("CLjhKsWxLTR+N5fs/jMYqVXL7xwtuEzufupX82c7LCs=\n").0,
            "CLjhKsWxLTR+N5fs/jMYqVXL7xwtuEzufupX82c7LCs=".to_string()
        );
        // from String
        assert_eq!(
            Key::<Public>::from("CLjhKsWxLTR+N5fs/jMYqVXL7xwtuEzufupX82c7LCs=\n".to_string()).0,
            "CLjhKsWxLTR+N5fs/jMYqVXL7xwtuEzufupX82c7LCs=".to_string()
        )
    }
}
