use crossterm::event;
use crossterm::event::KeyEvent;
use crossterm::event::*;
use std::io;
use std::time::Duration;

pub struct Input;

impl Input {
    pub fn read_key(&self) -> io::Result<KeyEvent> {
        loop {
            if event::poll(Duration::from_millis(500))? {
                if let Event::Key(event) = event::read()? {
                    return Ok(event);
                }
            }
        }
    }
}
