use crate::input;
use crate::output;

use crossterm::event;
use crossterm::event::KeyEvent;
use crossterm::event::*;
use std::io;

pub const EMPTY: char = ' ';
const START_GRID_STATE: [[char; 3]; 3] = [
    [EMPTY, EMPTY, EMPTY],
    [EMPTY, EMPTY, EMPTY],
    [EMPTY, EMPTY, EMPTY],
];
const PLAYER_X: char = 'X';
const PLAYER_O: char = 'O';

pub struct GameState {
    pub grid_state: [[char; 3]; 3],
    pub active_player: char,
    game_state_changed: bool,
    pub winner: char,
}

impl GameState {
    fn new() -> Self {
        Self {
            grid_state: START_GRID_STATE,
            active_player: PLAYER_X,
            game_state_changed: false,
            winner: EMPTY,
        }
    }
}

pub struct GameLoop {
    input: input::Input,
    output: output::Output,
    game_state: GameState,
}

impl GameLoop {
    pub fn new() -> Self {
        Self {
            input: input::Input,
            output: output::Output::new(),
            game_state: GameState::new(),
        }
    }

    fn place_mark(&mut self) {
        if self.game_state.winner != EMPTY {
            return;
        }
        let x = self.output.cursor_controller.cursor_x;
        let y = self.output.cursor_controller.cursor_y;
        if self.game_state.grid_state[y][x] == EMPTY {
            self.game_state.grid_state[y][x] = self.game_state.active_player;
            self.game_state.active_player = if self.game_state.active_player == PLAYER_X {
                PLAYER_O
            } else {
                PLAYER_X
            };
            self.game_state.game_state_changed = true;
        }
    }

    pub fn process_keypress(&mut self) -> io::Result<bool> {
        match self.input.read_key()? {
            KeyEvent {
                code: KeyCode::Char('c'),
                modifiers: event::KeyModifiers::CONTROL,
                ..
            } => return Ok(false),
            KeyEvent {
                code: direction @ (KeyCode::Up | KeyCode::Down | KeyCode::Left | KeyCode::Right),
                modifiers: KeyModifiers::NONE,
                ..
            } => self.output.move_cursor(direction),
            KeyEvent {
                code: KeyCode::Enter,
                ..
            } => self.place_mark(),
            _ => {}
        }
        Ok(true)
    }

    fn check_game_state(&mut self) {
        let grid = &self.game_state.grid_state;
        (0..2).for_each(|row| {
            if (grid[row][0] != EMPTY)
                && grid[row][0] == grid[row][1]
                && grid[row][1] == grid[row][2]
            {
                self.game_state.winner = grid[row][0];
            }
        });
        (0..2).for_each(|col| {
            if (grid[0][col] != EMPTY)
                && grid[0][col] == grid[1][col]
                && grid[1][col] == grid[2][col]
            {
                self.game_state.winner = grid[0][col];
            }
        });
        if (grid[0][0] != EMPTY) && grid[0][0] == grid[1][1] && grid[1][1] == grid[2][2] {
            self.game_state.winner = grid[0][0];
        }
        if (grid[0][2] != EMPTY) && grid[0][2] == grid[1][1] && grid[1][1] == grid[2][0] {
            self.game_state.winner = grid[0][2];
        }
    }

    pub fn run(&mut self) -> io::Result<bool> {
        if self.game_state.game_state_changed {
            self.check_game_state();
        }
        self.output.refresh_screen(&self.game_state)?;
        self.process_keypress()
    }
}
