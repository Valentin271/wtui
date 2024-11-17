use crate::app::{App, AppResult};
use ratatui::crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

/// Handles the key events and updates the state of [`App`].
pub fn handle_key_events(key_event: KeyEvent, app: &mut App) -> AppResult<()> {
    match (key_event.modifiers, key_event.code) {
        // Exit application on `ESC`, `q` or 'Ctrl-C'
        (_, KeyCode::Esc | KeyCode::Char('q')) | (KeyModifiers::CONTROL, KeyCode::Char('c')) => {
            app.quit()
        }
        // Down
        (_, KeyCode::Char('j') | KeyCode::Down) => app.down(),
        // Up
        (_, KeyCode::Char('k') | KeyCode::Up) => app.up(),
        // Connect
        (_, KeyCode::Char('c')) => app.connect_selected(),
        // Disconnect
        (_, KeyCode::Char('d')) => app.disconnect_selected(),
        (_, KeyCode::Char('D')) => app.disconnect_all(),
        // Yank
        (_, KeyCode::Char('y')) => app.yank_menu(),
        // Show help
        (_, KeyCode::Char('?')) => {
            todo!("Implement help popup")
        }
        // Other handlers you could add here.
        _ => {}
    }
    Ok(())
}
