use ratatui::style::Style;
use tui_textarea::{CursorMove, TextArea};
use utils::VimMode;
pub use tui_textarea;

pub struct URLInput<'a> {
    pub text_area: TextArea<'a>,
    pub mode: VimMode,
    pub vim_mode: bool,
}

impl<'a> URLInput<'a> {
    pub fn new(vim_mode: bool) -> Self {
        let mode = if vim_mode {
            VimMode::Normal
        } else {
            VimMode::Insert
        };
        let mut text_area = TextArea::default();
        text_area.set_cursor_line_style(Style::default());
        text_area.set_placeholder_text("Enter a valid URL");
        text_area.insert_str("http://");

        Self {
            text_area,
            mode,
            vim_mode,
        }
    }

    pub fn enter_insert_mode(&mut self) {
        if self.vim_mode {
            self.text_area.cancel_selection();
            self.mode = VimMode::Insert;
        }
    }

    pub fn enter_normal_mode(&mut self) {
        if self.vim_mode {
            self.text_area.cancel_selection();
            self.mode = VimMode::Normal;
        }
    }

    pub fn enter_visual_mode(&mut self) {
        if self.vim_mode {
            self.mode = VimMode::Visual;
            self.text_area.start_selection();
        }
    }

    fn clear_selection(&mut self) {
        self.text_area.cancel_selection();
    }

    //
    // Movement
    //


    pub fn move_cursor_left(&mut self) {
        if self.mode != VimMode::Visual {
            self.clear_selection();
        }
        self.text_area.move_cursor(CursorMove::Back);
    }

    pub fn move_cursor_left_selecting(&mut self) {
        if !self.text_area.is_selecting() {
            self.text_area.start_selection();
        }

        self.text_area.move_cursor(CursorMove::Back);
    }

    pub fn move_cursor_left_by_word(&mut self) {
        if self.mode != VimMode::Visual {
            self.clear_selection();
        }
        self.text_area.move_cursor(CursorMove::WordBack);
    }

    pub fn move_cursor_left_by_word_selecting(&mut self) {
        if !self.text_area.is_selecting() {
            self.text_area.start_selection();
        }

        self.text_area.move_cursor(CursorMove::WordBack);
    }

    pub fn move_cursor_left_by_word_paragraph(&mut self) {
        if self.mode != VimMode::Visual {
            self.clear_selection();
        }
        self.text_area.move_cursor(CursorMove::ParagraphBack);
    }

    pub fn move_cursor_right(&mut self) {
        if self.mode != VimMode::Visual {
            self.clear_selection();
        }
        self.text_area.move_cursor(CursorMove::Forward);
    }

    pub fn move_cursor_right_selecting(&mut self) {
        if !self.text_area.is_selecting() {
            self.text_area.start_selection();
        }

        self.text_area.move_cursor(CursorMove::Forward);
    }

    pub fn move_cursor_right_by_word(&mut self) {
        if self.mode != VimMode::Visual {
            self.clear_selection();
        }
        self.text_area.move_cursor(CursorMove::WordForward)
    }

    pub fn move_cursor_right_by_word_selecting(&mut self) {
        if !self.text_area.is_selecting() {
            self.text_area.start_selection();
        }

        self.text_area.move_cursor(CursorMove::WordForward);
    }

    pub fn move_cursor_right_by_word_paragraph(&mut self) {
        if self.mode != VimMode::Visual {
            self.clear_selection();
        }
        self.text_area.move_cursor(CursorMove::ParagraphForward)
    }

    pub fn move_cursor_right_by_word_end(&mut self) {
        if self.mode != VimMode::Visual {
            self.clear_selection();
        }
        self.text_area.move_cursor(CursorMove::WordEnd)
    }

    pub fn move_cursor_bol(&mut self) {
        if self.mode != VimMode::Visual {
            self.clear_selection();
        }
        self.text_area.move_cursor(CursorMove::Head);
    }

    pub fn move_cursor_bol_selecting(&mut self) {
        if !self.text_area.is_selecting() {
            self.text_area.start_selection();
        }

        self.text_area.move_cursor(CursorMove::Head);
    }

    pub fn move_cursor_eol(&mut self) {
        if self.mode != VimMode::Visual {
            self.clear_selection();
        }
        self.text_area.move_cursor(CursorMove::End);
    }

    pub fn move_cursor_eol_selecting(&mut self) {
        if !self.text_area.is_selecting() {
            self.text_area.start_selection();
        }

        self.text_area.move_cursor(CursorMove::End);
    }

    //
    // Editing
    //

    pub fn delete_word_back(&mut self) {
        let _ = self.text_area.delete_word(); // We don't really care about the bool value here.
    }

    pub fn delete_word_forward(&mut self) {
        let _ = self.text_area.delete_next_word();
    }

    /// NOTE: This deletes forward.
    pub fn delete_char(&mut self) {
        let _ = self.text_area.delete_next_char();
    }

    pub fn backspace(&mut self) {
        let _ = self.text_area.delete_char();
    }

    pub fn insert_char(&mut self, c: char) {
        self.text_area.insert_char(c);
    }

    pub fn delete_until_eol(&mut self) {
        let _ = self.text_area.delete_line_by_end();
    }

    pub fn delete_until_hol(&mut self) {
        let _ = self.text_area.delete_line_by_head();
    }

    pub fn undo(&mut self) {
        let _ = self.text_area.undo();
    }

    pub fn redo(&mut self) {
        let _ = self.text_area.redo();
    }

    pub fn copy(&mut self) {
        self.text_area.copy();
    }

    pub fn paste(&mut self) {
        self.text_area.paste();
    }

    pub fn cut(&mut self) {
        self.text_area.cut();
    }
}
