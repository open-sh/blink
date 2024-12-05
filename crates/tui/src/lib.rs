use anyhow::{Context, Result};
use config::HTTPRequest;
use editor::Editor;
use ratatui::{
    layout::{Constraint, Direction, Layout, Position, Rect},
    style::{Color, Style},
    widgets::{Block, Borders, List, ListItem, ListState, Paragraph},
    DefaultTerminal, Frame,
};
use side_panel::SidePanel;
use url_input::URLInput;
use utils::VimMode;

mod editor;
pub mod events;
pub mod keys;
mod side_panel;
pub mod url_input;

/// `BlinkRenderer` controls the state in which the terminal should be rendered.
pub struct BlinkRenderer<'a> {
    pub focus_area: FocusArea,
    pub url_input: URLInput<'a>,
    pub editor: Editor,
    pub side_panel: SidePanel,
    pub vim_mode: bool,
}

/// Determines the direction in which the cursor focus takes place.
#[derive(PartialEq, Clone, Copy)]
pub enum FocusArea {
    SidePanel,
    URLInput,
    Editor,
}

impl<'a> BlinkRenderer<'a> {
    pub fn new(requests: Vec<HTTPRequest>, vim_mode: bool) -> Self {
        Self {
            focus_area: FocusArea::SidePanel,
            url_input: URLInput::new(vim_mode),
            editor: Editor::new(vim_mode),
            vim_mode,
            side_panel: SidePanel::new(requests, vim_mode),
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

    pub fn update_requests(&mut self, new_requests: Vec<HTTPRequest>) {
        self.side_panel.requests = new_requests;
    }

    pub fn update_vim_mode(&mut self, vim_mode: bool) {
        self.url_input.vim_mode = vim_mode;
        self.editor.vim_mode = vim_mode;
    }

    //
    // Widget rendering.
    //

    pub fn render_url_input(&mut self, f: &mut Frame, area: Rect) {
        let block = if self.focus_area == FocusArea::URLInput {
            let title = match self.url_input.mode {
                VimMode::Insert => "URL [Insert]",
                VimMode::Normal => "URL [Normal]",
                VimMode::Visual => "URL [Visual]",
                VimMode::Any => "URL",
            };

            Block::default()
                .borders(Borders::ALL)
                .title(title)
                .border_style(Style::default().fg(Color::Yellow))
        } else {
            Block::default().borders(Borders::ALL).title("URL")
        };

        self.url_input.text_area.set_block(block);

        f.render_widget(&self.url_input.text_area, area);
    }

    pub fn render_side_panel(&mut self, f: &mut Frame, area: Rect) {
        let items: Vec<ListItem> = self
            .side_panel
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
        if !self.side_panel.requests.is_empty() {
            state.select(Some(self.side_panel.selected_request));
        }

        let requests = List::new(items)
            .block(block)
            .highlight_style(Style::default().bg(Color::Blue));

        f.render_stateful_widget(requests, area, &mut state);
    }

    pub fn render_editor(&mut self, f: &mut Frame, area: Rect) {
        let block = if self.focus_area == FocusArea::Editor {
            let title = match self.editor.mode {
                VimMode::Insert => "Request body [Insert]",
                VimMode::Normal => "Request body [Normal]",
                VimMode::Visual => "Request body [Visual]",
                VimMode::Any => "Request body",
            };

            Block::default()
                .borders(Borders::ALL)
                .title(title)
                .border_style(Style::default().fg(Color::Yellow)) // Yellow style when focused.
        } else {
            Block::default().borders(Borders::ALL).title("Request body")
        };

        let text: String = self.editor.content.to_string();
        let paragraph = Paragraph::new(text).block(block);

        f.render_widget(paragraph, area);

        if self.focus_area == FocusArea::Editor {
            let x_offset = self.editor.cursor_x as u16;
            let y_offset = self.editor.cursor_y as u16;

            let x = area.x + x_offset + 1; // +1 for left border.
            let y = area.y + y_offset + 1; // +1 for right border.

            f.set_cursor_position(Position::new(x, y));
        }
    }
}
