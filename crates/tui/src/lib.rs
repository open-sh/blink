use events::{handle_event, poll_events, BlinkCommand};
use ratatui::{buffer::Buffer, layout::Rect, style::Style, widgets::Widget, DefaultTerminal};
use anyhow::{Context, Result};

mod events;

/// `BlinkRenderer` controls the state in which the terminal should be rendered.
#[derive(Default)]
pub struct BlinkRenderer {
    pub message: String,
    pub should_quit: bool,
}

impl BlinkRenderer {
    pub fn new(message: String) -> Self {
        Self {
            message,
            should_quit: false
        }
    }

    /// Initializes the terminal using the default `init` function from `ratatui`, returns
    /// a `DefaultTerminal` to be manipulated by the renderer.
    pub fn init(&self) -> DefaultTerminal {
        let terminal = ratatui::init(); // This uses `crossterm` as a backend.
        return terminal;
    }

    /// Restores the terminal.
    pub fn restore(&self) {
        ratatui::restore()
    }

    /// Draw the UI.
    pub fn draw(&mut self, mut terminal: DefaultTerminal) -> Result<()> {
        loop {
            // Rendering.
            terminal.draw(|f| {
                let size = f.area();
                f.render_widget(&*self, size);
            }).context("ERROR: Drawing the renderer to the terminal.")?;

            // Event handling.
            let events = poll_events().context("ERROR: polling events.")?;
            for event in events {
                let commands = handle_event(event);
                for command in commands {
                    match command {
                        BlinkCommand::Quit => self.should_quit = true,
                    }
                }
            }

            // Check if we should quit.
            if self.should_quit {
                break;
            }
        }

        Ok(())
    }
}

/// Widget means that `BlinkRenderer` will be drawn on a `Buffer` on a given `Rect`.
///
/// NOTE: Passing `BlinkRenderer` by reference to avoid consumption, since it's just a reading
/// operating anyways.
impl Widget for &BlinkRenderer {
    fn render(self, area: Rect, buf: &mut Buffer)
    where
        Self: Sized,
    {
        buf.set_style(area, Style::default());

        let x = area.x + (area.width.saturating_sub(self.message.len() as u16)) / 2;
        let y = area.y + area.height / 2;

        buf.set_string(x, y, &self.message, Style::default());
    }
}
