use std::error;
use std::fs;

use block::Title;
use clipboard::ClipboardContext;
use clipboard::ClipboardProvider;
use connection::Connection;
use ratatui::prelude::*;
use ratatui::widgets::*;
use state::State;

use crate::wg::WgConfig;

mod connection;
mod state;

/// Application result type.
pub type AppResult<T> = std::result::Result<T, Box<dyn error::Error>>;

/// Application.
pub struct App {
    /// Is the application running?
    pub running: bool,
    connections: Vec<Connection>,
    table_state: TableState,
    state: State,
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
            let config = fs::read_to_string(file.path())?;
            connections.push(Connection::new(
                file.file_name().to_string_lossy().trim_end_matches(".conf"),
                WgConfig::from(config.as_str()),
            ))
        }

        Ok(Self {
            running: true,
            connections,
            table_state: TableState::default().with_selected(0),
            state: State::Main,
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
        let mut new = self.table_state.selected().unwrap_or(0) + 1;

        if new >= self.connections.len() {
            new = 0;
        }

        self.table_state.select(Some(new));
    }

    /// Select the previous element in the app
    pub fn up(&mut self) {
        let new = self
            .table_state
            .selected()
            .unwrap_or(0)
            .checked_sub(1)
            .unwrap_or(self.connections.len() - 1);

        self.table_state.select(Some(new));
    }

    pub fn selected(&self) -> Option<&Connection> {
        self.connections
            .get(self.table_state.selected().unwrap_or(0))
    }

    /// Connects to the selected (hovered) connection
    pub fn connect_selected(&mut self) {
        if let Some(con) = self.selected() {
            let _ = con.connect();
        }
    }

    /// Disconnects from the selected (hovered) connection
    pub fn disconnect_selected(&mut self) {
        if let Some(con) = self.selected() {
            let _ = con.disconnect();
        }
    }

    /// Disconnects all connections.
    pub fn disconnect_all(&mut self) {
        self.connections.iter().for_each(|c| {
            let _ = c.disconnect();
        });
    }

    /// Enable the yank (copy) menu
    pub fn yank_menu(&mut self) {
        self.state = State::Yank;

        if let Some(con) = self.selected() {
            let pubkey = con.pubkey();

            let mut ctx: ClipboardContext = ClipboardProvider::new().unwrap();
            ctx.set_contents(pubkey.to_string()).unwrap();
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
                    "Public Key",
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

        StatefulWidget::render(list, area, buf, &mut self.table_state);
    }
}
