use wtui::app::{App, AppResult};
use wtui::event::{Event, EventHandler};
use wtui::handler::handle_key_events;

fn main() -> AppResult<()> {
    // Create an application.
    let mut app = App::new()?;

    let event_handler = EventHandler::new(250);

    ratatui::run(|terminal| {
        // Start the main loop.
        while app.running {
            // Render the user interface.
            terminal.draw(|frame| frame.render_widget(&mut app, frame.area()))?;

            // Handle events.
            match event_handler.next()? {
                Event::Tick => app.tick(),
                Event::Key(key_event) => handle_key_events(key_event, &mut app)?,
                Event::Mouse(_) => {}
                Event::Resize(_, _) => {}
            }
        }

        Ok(())
    })
}
