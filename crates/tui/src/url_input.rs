use utils::VimMode;

pub struct URLInput {
    pub input: String,
    pub cursor_position: usize,
    pub mode: VimMode,
    pub vim_mode: bool,
}

impl URLInput {
    pub fn new(vim_mode: bool) -> Self {
        let mode = if vim_mode { VimMode::Normal } else { VimMode::Any };

        Self {
            input: "http://".to_string(),
            cursor_position: 7,
            mode,
            vim_mode,
        }
    }

    pub fn insert_char(&mut self, c: char) {
        assert!(
            self.cursor_position <= self.input.len(),
            "ERROR: Cursor position can not exceed URL length"
        );
        self.input.insert(self.cursor_position, c);
        self.cursor_position += 1;
    }

    pub fn backspace(&mut self) {
        if self.cursor_position > 0 && !self.input.is_empty() {
            self.input.remove(self.cursor_position - 1);
            self.cursor_position -= 1;
        }
        assert!(
            self.cursor_position <= self.input.len(),
            "ERROR: Cursor position can not exceed URL length"
        );
    }

    pub fn move_cursor_left(&mut self) {
        if self.cursor_position > 0 {
            self.cursor_position -= 1;
        }
    }

    pub fn move_cursor_right(&mut self) {
        if self.cursor_position < self.input.len() {
            self.cursor_position += 1;
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
}
