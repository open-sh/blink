use anyhow::{Context, Result};
use config::HTTPRequest;
use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Style},
    widgets::{Block, Borders, List, ListItem, ListState, Paragraph},
    DefaultTerminal, Frame,
};

pub mod events;

/// `BlinkRenderer` controls the state in which the terminal should be rendered.
pub struct BlinkRenderer {
    pub message: String,
    pub requests: Vec<HTTPRequest>,
    pub focus_area: FocusArea,
}

/// Determines the direction in which the cursor focus takes place.
#[derive(PartialEq)]
pub enum FocusArea {
    SidePanel,
    URLInput,
    Editor,
}

impl BlinkRenderer {
    pub fn new(message: String, requests: Vec<HTTPRequest>) -> Self {
        Self {
            message,
            requests,
            focus_area: FocusArea::URLInput,
        }
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

    /// Draw the UI based on a `terminal`.
    pub fn draw(&mut self, terminal: &mut DefaultTerminal) -> Result<()> {
        terminal
            .draw(|f| {
                self.draw_blink(f);
            })
            .context("ERROR: Drawing the renderer to the terminal.")?;

        Ok(())
    }

    pub fn draw_blink(&mut self, f: &mut Frame) {
        let size = f.area();

        // This creates the main vertical layout.
        let horizontal_chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([
                Constraint::Length(30), // Fixed width for lateral requests bar.
                Constraint::Min(0),     // Remaining space for main content.
            ])
            .split(size);

        let side_panel_area = horizontal_chunks[0];
        let main_area = horizontal_chunks[1];

        let vertical_chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(3), // Fixed height for URL entry.
                Constraint::Min(0),    // Editor for the rest.
            ])
            .split(main_area);

        let url_panel_area = vertical_chunks[0];
        let editor_area = vertical_chunks[1];

        // Render widgets in each area
        self.render_url_input(f, url_panel_area);
        self.render_side_panel(f, side_panel_area);
        self.render_editor(f, editor_area);
    }

    //
    // Update `BlinkRenderer` state.
    //

    pub fn update_message(&mut self, new_message: String) {
        self.message = new_message;
    }

    pub fn update_requests(&mut self, new_requests: Vec<HTTPRequest>) {
        self.requests = new_requests;
    }

    //
    // Widget rendering.
    //

    pub fn render_url_input(&mut self, f: &mut Frame, area: Rect) {
        let block = if self.focus_area == FocusArea::URLInput {
            Block::default()
                .borders(Borders::ALL)
                .title("URL")
                .border_style(Style::default().fg(Color::Yellow))
        } else {
            Block::default().borders(Borders::ALL).title("URL")
        };

        let url_input = Paragraph::new("we gucci").block(block);
        f.render_widget(url_input, area);
    }

    pub fn render_side_panel(&mut self, f: &mut Frame, area: Rect) {
        let items: Vec<ListItem> = self
            .requests
            .iter()
            .map(|request| ListItem::new(request.name.clone()))
            .collect();

        // Determine the block style based on the focus.
        // TODO: Should probably put all color related things into globals/config.
        let block = if self.focus_area == FocusArea::SidePanel {
            Block::default()
                .borders(Borders::ALL)
                .title("Requests")
                .border_style(Style::default().fg(Color::Yellow)) // Yellow style when focused.
        } else {
            Block::default().borders(Borders::ALL).title("Requests")
        };

        let mut state = ListState::default();
        if !self.requests.is_empty() {
            state.select(Some(0)); // Initial request selection.
        }

        let requests = List::new(items)
            .block(block)
            .highlight_style(Style::default().bg(Color::Blue))
            .highlight_symbol("> ");

        f.render_stateful_widget(requests, area, &mut state);
    }

    pub fn render_editor(&mut self, f: &mut Frame, area: Rect) {
        let block = if self.focus_area == FocusArea::Editor {
            Block::default()
                .borders(Borders::ALL)
                .title("Request body")
                .border_style(Style::default().fg(Color::Yellow)) // Yellow style when focused.
        } else {
            Block::default().borders(Borders::ALL).title("Request body")
        };

        let request_body: String = String::from(self.message.clone());
        let editor = Paragraph::new(request_body).block(block);

        f.render_widget(editor, area);
    }
}
