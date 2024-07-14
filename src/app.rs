use std::error;
use std::fs;

use block::Title;
use connection::Connection;
use ratatui::prelude::*;
use ratatui::widgets::*;

use crate::wg::WgConfig;

mod connection;

/// Application result type.
pub type AppResult<T> = std::result::Result<T, Box<dyn error::Error>>;

/// Application.
pub struct App {
    /// Is the application running?
    pub running: bool,
    connections: Vec<Connection>,
    state: TableState,
}

impl App {
    /// Constructs a new instance of [`App`].
    ///
    /// Loads the wireguard configurations from `/etc/wireguard`.
    pub fn new() -> AppResult<Self> {
        let files = fs::read_dir("/etc/wireguard")?;

        let mut connections = Vec::new();

        for file in files {
            let file = file?;
            let config = fs::read_to_string(&file.path())?;
            connections.push(Connection::new(
                file.file_name().to_string_lossy().trim_end_matches(".conf"),
                WgConfig::from(config.as_str()),
            ))
        }

        Ok(Self {
            running: true,
            connections,
            state: TableState::default().with_selected(0),
        })
    }

    /// Handles the tick event of the terminal.
    pub fn tick(&mut self) {
        for con in &mut self.connections {
            con.update();
        }
    }

    /// Select the next element in the app
    pub fn down(&mut self) {
        let mut new = self.state.selected().unwrap_or(0) + 1;

        if new >= self.connections.len() {
            new = 0;
        }

        self.state.select(Some(new));
    }

    /// Select the previous element in the app
    pub fn up(&mut self) {
        let new = self
            .state
            .selected()
            .unwrap_or(0)
            .checked_sub(1)
            .unwrap_or(self.connections.len() - 1);

        self.state.select(Some(new));
    }

    pub fn connect_selected(&mut self) {
        if let Some(con) = self.connections.get(self.state.selected().unwrap_or(0)) {
            let _ = con.connect();
        }
    }

    pub fn disconnect_selected(&mut self) {
        if let Some(con) = self.connections.get(self.state.selected().unwrap_or(0)) {
            let _ = con.disconnect();
        }
    }

    /// Set running to false to quit the application.
    pub fn quit(&mut self) {
        self.running = false;
    }
}

impl Widget for &mut App {
    fn render(self, area: Rect, buf: &mut Buffer)
    where
        Self: Sized,
    {
        let border = Block::bordered()
            .border_type(BorderType::Rounded)
            .title(Title::from(" Connections ").alignment(Alignment::Center));

        let list = Table::default()
            .rows(self.connections.iter().map(Row::from))
            .header(
                Row::new([
                    "Name",
                    "Address",
                    "MTU",
                    "Received",
                    "Sent",
                    "Endpoint",
                    "Allowed IPs",
                    "Latest ",
                    "Public key",
                    "DNS",
                ])
                .bold()
                .underlined(),
            )
            .widths([
                Constraint::Max(12),
                Constraint::Fill(1),
                Constraint::Length(4),
                Constraint::Max(8),
                Constraint::Max(8),
                Constraint::Fill(1),
                Constraint::Fill(1),
                Constraint::Max(12),
                Constraint::Max(10),
                Constraint::Fill(1),
            ])
            .block(border)
            .highlight_style(Style::new().reversed())
            .highlight_symbol(">> ");

        StatefulWidget::render(list, area, buf, &mut self.state);
    }
}
