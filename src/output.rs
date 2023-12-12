use crate::cursorcontroller;
use crate::gameloop;
use crate::window;

use crossterm::{cursor, execute, queue, terminal};
use crossterm::{event::*, terminal::ClearType};
use std::io;
use std::io::stdout;
use std::io::Write;

pub struct Output {
    contents: window::WindowContents,
    pub cursor_controller: cursorcontroller::CursorController,
}

impl Output {
    pub fn new() -> Self {
        Self {
            contents: window::WindowContents::new(),
            cursor_controller: cursorcontroller::CursorController::new(),
        }
    }

    pub fn clear_screen() -> io::Result<()> {
        execute!(stdout(), terminal::Clear(ClearType::All))?;
        execute!(stdout(), cursor::MoveTo(0, 0))
    }

    pub fn draw_game(&mut self, game_state: &gameloop::GameState) {
        let mut new_screenbuf = [
            [' ', '|', ' ', '|', ' '],
            ['-', '+', '-', '+', '-'],
            [' ', '|', ' ', '|', ' '],
            ['-', '+', '-', '+', '-'],
            [' ', '|', ' ', '|', ' '],
        ];

        for (y_idx, row) in game_state.grid_state.iter().enumerate() {
            for (x_idx, char) in row.iter().enumerate() {
                new_screenbuf[y_idx * 2][x_idx * 2] = *char;
            }
        }

        for row in new_screenbuf {
            for char in row {
                self.contents.push(char);
            }
            self.contents.push('\r');
            self.contents.push('\n');
        }
        self.contents.push_str("\r\n");
        if game_state.winner == gameloop::EMPTY {
            self.contents.push_str(&format!(
                "Turn: {} (<Ctrl-c> to quit)\r\n",
                &game_state.active_player
            ));
        } else {
            self.contents.push_str(&format!(
                "Winner: {} (<Ctrl-c> to quit)\r\n",
                &game_state.winner
            ));
        }
    }

    pub fn move_cursor(&mut self, direction: KeyCode) {
        self.cursor_controller.move_cursor(direction);
    }

    pub fn refresh_screen(&mut self, game_state: &gameloop::GameState) -> io::Result<()> {
        queue!(
            self.contents,
            terminal::Clear(ClearType::All),
            cursor::MoveTo(0, 0)
        )?;
        self.draw_game(game_state);

        let cursor_x = self.cursor_controller.cursor_x * 2;
        let cursor_y = self.cursor_controller.cursor_y * 2;
        queue!(
            self.contents,
            cursor::MoveTo(cursor_x as u16, cursor_y as u16),
            cursor::Show
        )?;

        self.contents.flush()
    }
}
