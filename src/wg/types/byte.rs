use std::{num::ParseIntError, ops::Deref, str::FromStr};

use ratatui::prelude::*;

#[derive(Default, Debug, Clone, Copy)]
pub struct Byte(usize);

impl From<usize> for Byte {
    fn from(value: usize) -> Self {
        Byte(value)
    }
}

impl FromStr for Byte {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Byte(s.parse()?))
    }
}

impl Deref for Byte {
    type Target = usize;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl From<&Byte> for Text<'_> {
    fn from(value: &Byte) -> Self {
        let (number, unit) = match value.0 {
            ..=1_000 => (value.0 as f32, "B"),
            1_001..=1_000_000 => (value.0 as f32 / 1_000., "KB"),
            1_000_001..=1_000_000_000 => (value.0 as f32 / 1_000_000., "MB"),
            1_000_000_001.. => (value.0 as f32 / 1_000_000_000., "GB"),
        };

        Line::from(vec![
            format!("{:.1} ", number).into(),
            Span::from(unit).cyan(),
        ])
        .into()
    }
}
