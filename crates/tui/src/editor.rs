use ropey::Rope;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EditorMode {
    Normal,
    Insert,
}

pub struct Editor {
    pub content: Rope,
    pub cursor_x: usize,
    pub cursor_y: usize,
    pub mode: EditorMode,
}

impl Editor {
    pub fn new() -> Self {
        Self {
            content: Rope::new(),
            cursor_x: 0,
            cursor_y: 0,
            mode: EditorMode::Normal,
        }
    }

    pub fn enter_insert_mode(&mut self) {
        self.mode = EditorMode::Insert
    }

    pub fn enter_normal_mode(&mut self) {
        self.mode = EditorMode::Normal
    }

    pub fn insert_char(&mut self, c: char) {
        // Gotta make sure that `cursor_y` does not go beyond the number of existing lines.
        // If `cursor_y == self.content.len_lines()`, for instance, just insert a newline:
        while self.cursor_y >= self.content.len_lines() {
            self.content.insert_char(self.content.len_chars(), '\n');
        }

        let line_start = self.content.line_to_char(self.cursor_y);
        let char_offset = line_start + self.cursor_x;

        self.content.insert_char(char_offset, c);
        self.cursor_x += 1;
    }

    pub fn delete_char(&mut self) {
        if self.cursor_y < self.content.len_lines() {
            let line_len = self.content.line(self.cursor_y).len_chars();
            if self.cursor_x < line_len {
                let line_start = self.content.line_to_char(self.cursor_y);
                let char_offset = line_start + self.cursor_x;
                self.content.remove(char_offset..char_offset + 1);
            }
        }
    }

    pub fn backspace(&mut self) {
        if self.mode == EditorMode::Insert {
            if self.cursor_y < self.content.len_lines() {
                if self.cursor_x > 0 {
                    // Remove char previous to cursor.
                    let line_start = self.content.line_to_char(self.cursor_y);
                    let char_offset = line_start + self.cursor_x - 1;
                    self.content.remove(char_offset..char_offset + 1);
                    self.cursor_x -= 1;
                } else if self.cursor_y > 0 {
                    // If Se cursor_x == 0, we need to join lines with the previous,
                    // if it is the desired behavior. We'll leave like this for simplicity.
                    self.cursor_y -= 1;
                    let line_len = self.content.line(self.cursor_y).len_chars();
                    let line_start = self.content.line_to_char(self.cursor_y);
                    let char_offset = line_start + line_len.saturating_sub(1);
                    if line_len > 0 {
                        self.content.remove(char_offset..char_offset + 1);
                        self.cursor_x = self.content.line(self.cursor_y).len_chars();
                    } else {
                        // Previous line is empty, nothing to delete.
                        self.cursor_x = 0;
                    }
                }
            }
        }
    }

    pub fn move_cursor_left(&mut self) {
        if self.cursor_y < self.content.len_lines() {
            if self.cursor_x > 0 {
                self.cursor_x -= 1;
            } else if self.cursor_y > 0 {
                self.cursor_y -= 1;
                let line_len = self.content.line(self.cursor_y).len_chars();
                self.cursor_x = line_len;
            }
        }
    }

    pub fn move_cursor_right(&mut self) {
        if self.cursor_y < self.content.len_lines() {
            let line_len = self.content.line(self.cursor_y).len_chars();
            if self.cursor_x < line_len {
                self.cursor_x += 1;
            } else if self.cursor_y + 1 < self.content.len_lines() {
                self.cursor_y += 1;
                self.cursor_x = 0;
            }
        }
    }

    pub fn move_cursor_up(&mut self) {
        if self.cursor_y > 0 {
            self.cursor_y -= 1;
            let line_len = self.content.line(self.cursor_y).len_chars();
            if self.cursor_x > line_len {
                self.cursor_x = line_len;
            }
        }
    }

    pub fn move_cursor_down(&mut self) {
        if self.cursor_y + 1 < self.content.len_lines() {
            self.cursor_y += 1;
            let line_len = self.content.line(self.cursor_y).len_chars();
            if self.cursor_x > line_len {
                self.cursor_x = line_len;
            }
        }
    }
}
