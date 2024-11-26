use anyhow::{Context, Result};
use ratatui::{buffer::Buffer, layout::Rect, style::Style, widgets::Widget, DefaultTerminal};

pub mod events;

/// `BlinkRenderer` controls the state in which the terminal should be rendered.
pub struct BlinkRenderer {
    pub message: String,
}

impl BlinkRenderer {
    pub fn new(message: String) -> Self {
        Self { message }
    }

    /// Initializes the terminal using the default `init` function from `ratatui`, returns
    /// a `DefaultTerminal` to be manipulated by the renderer.
    pub fn init(&self) -> DefaultTerminal {
        ratatui::init() // This uses `crossterm` as a backend.
    }

    /// Restores the terminal.
    pub fn restore(&self) {
        ratatui::restore()
    }

    /// Draw the UI.
    pub fn draw(&mut self, terminal: &mut DefaultTerminal) -> Result<()> {
        terminal
            .draw(|f| {
                let size = f.area();
                f.render_widget(&*self, size);
            })
            .context("ERROR: Drawing the renderer to the terminal.")?;

        Ok(())
    }

    pub fn update_message(&mut self, new_message: String) {
        self.message = new_message;
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
