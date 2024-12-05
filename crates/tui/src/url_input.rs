use ratatui::style::Style;
use tui_textarea::{CursorMove, Input, TextArea};
use utils::VimMode;
pub use tui_textarea;

pub struct URLInput<'a> {
    pub text_area: TextArea<'a>,
    pub mode: VimMode,
    pub vim_mode: bool,
    selection_anchor: Option<(usize, usize)>, // Row and column where the selection begins.
}

impl<'a> URLInput<'a> {
    pub fn new(vim_mode: bool) -> Self {
        let mode = if vim_mode {
            VimMode::Normal
        } else {
            VimMode::Any
        };
        let mut text_area = TextArea::default();
        text_area.set_cursor_line_style(Style::default());
        text_area.set_placeholder_text("Enter a valid URL");
        text_area.insert_str("http://");

        Self {
            text_area,
            mode,
            vim_mode,
            selection_anchor: None,
        }
    }

    pub fn enter_insert_mode(&mut self) {
        if self.vim_mode {
            self.mode = VimMode::Insert;
        }
    }

    pub fn enter_normal_mode(&mut self) {
        if self.vim_mode {
            self.mode = VimMode::Normal;
        }
    }

    /// Envia um evento de entrada para o TextArea. Se o texto mudar, retorna true.
    pub fn input(&mut self, input: Input) -> bool {
        let changed = self.text_area.input(input);
        // Aqui você pode adicionar validação ou estilo condicional
        changed
    }

    fn clear_selection(&mut self) {
        self.selection_anchor = None;
        self.text_area.cancel_selection();
    }

    //
    // Movement
    //


    pub fn move_cursor_left(&mut self) {
        self.clear_selection();
        self.text_area.move_cursor(CursorMove::Back);
    }

    pub fn move_cursor_left_selecting(&mut self) {
        if !self.text_area.is_selecting() {
            self.text_area.start_selection();
        }

        self.text_area.move_cursor(CursorMove::Back);
    }

    pub fn move_cursor_right(&mut self) {
        self.clear_selection();
        self.text_area.move_cursor(CursorMove::Forward);
    }

    pub fn move_cursor_right_selecting(&mut self) {
        if !self.text_area.is_selecting() {
            self.text_area.start_selection();
        }

        self.text_area.move_cursor(CursorMove::Forward);
    }

    pub fn move_cursor_right_by_word_end(&mut self) {
        self.text_area.move_cursor(CursorMove::WordEnd)
    }

    pub fn move_cursor_right_by_word(&mut self) {
        self.text_area.move_cursor(CursorMove::WordForward)
    }

    //
    // Editing
    //

    pub fn delete_word(&mut self) {
        let _ = self.text_area.delete_word(); // We don't really care about the bool value here.
    }

    pub fn delete_char(&mut self) {
        let _ = self.text_area.delete_next_char(); // We don't really care about the bool value here.
    }

    pub fn backspace(&mut self) {
        let _ = self.text_area.delete_char(); // We don't really care about the bool value here.
    }
}
