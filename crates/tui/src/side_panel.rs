use config::HTTPRequest;
use utils::VimMode;

pub struct SidePanel {
    pub requests: Vec<HTTPRequest>,
    pub selected_request: usize,
    pub mode: VimMode,
    pub vim_mode: bool,
}

impl SidePanel {
    pub fn new(requests: Vec<HTTPRequest>, vim_mode: bool) -> Self {
        let mode = if vim_mode { VimMode::Normal } else { VimMode::Any };

        SidePanel {
            requests,
            selected_request: 0,
            mode,
            vim_mode,
        }
    }

    pub fn move_cursor_up(&mut self) {
        if self.selected_request > 0 {
            self.selected_request -= 1;
        }
    }

    pub fn move_cursor_down(&mut self) {
        if self.selected_request + 1 < self.requests.len() {
            self.selected_request += 1;
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
