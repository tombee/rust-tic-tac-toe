use crossterm::event::*;

pub struct CursorController {
    pub cursor_x: usize,
    pub cursor_y: usize,
}

impl CursorController {
    pub fn new() -> CursorController {
        Self {
            cursor_x: 0,
            cursor_y: 0,
        }
    }

    pub fn move_cursor(&mut self, direction: KeyCode) {
        match direction {
            KeyCode::Up => {
                self.cursor_y = self.cursor_y.saturating_sub(1);
            }
            KeyCode::Left => {
                self.cursor_x = self.cursor_x.saturating_sub(1);
            }
            KeyCode::Down => {
                if self.cursor_y != 3 - 1 {
                    self.cursor_y += 1;
                }
            }
            KeyCode::Right => {
                if self.cursor_x != 3 - 1 {
                    self.cursor_x += 1;
                }
            }
            _ => unimplemented!(),
        }
    }
}
