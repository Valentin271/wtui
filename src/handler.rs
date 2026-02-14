use crate::app::{App, AppResult, focus::Focus};
use ratatui::crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

/// Handles the key events and updates the state of [`App`].
pub fn handle_key_events(key_event: KeyEvent, app: &mut App) -> AppResult<()> {
    if let Focus::Search = app.focus() {
        handle_key_search(key_event, app);
        return Ok(());
    }

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
        // Search
        (_, KeyCode::Char('/')) => {
            let current_search = app.search_term().to_owned();
            app.search(Some(&current_search));
        }
        // Show help
        (_, KeyCode::Char('?')) => {
            todo!("Implement help popup")
        }
        // Other handlers you could add here.
        _ => {}
    }
    Ok(())
}

fn handle_key_search(key_event: KeyEvent, app: &mut App) {
    match (key_event.modifiers, key_event.code) {
        // Exit search mode
        (_, KeyCode::Esc | KeyCode::Enter) | (KeyModifiers::CONTROL, KeyCode::Char('c')) => {
            app.search(None)
        }
        // Down
        (KeyModifiers::CONTROL, KeyCode::Char('j')) | (_, KeyCode::Down) => app.down(),
        // Up
        (KeyModifiers::CONTROL, KeyCode::Char('k')) | (_, KeyCode::Up) => app.up(),
        // Delete
        (_, KeyCode::Backspace) => {
            let mut current_term = app.search_term().to_owned();
            if current_term.is_empty() {
                app.search(None);
            } else {
                current_term.pop();
                app.search(Some(&current_term))
            }
        }
        // Search char
        (_, KeyCode::Char(c)) => {
            let mut term = app.search_term().to_owned();
            term.push(c);
            app.search(Some(&term));
        }
        // Ignore rest
        _ => {}
    }
}
