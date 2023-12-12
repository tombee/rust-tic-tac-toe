mod cursorcontroller;
mod gameloop;
mod input;
mod output;
mod window;

use crossterm::terminal;
use std::io;

struct CleanUp;

impl Drop for CleanUp {
    fn drop(&mut self) {
        terminal::disable_raw_mode().expect("Unable to disable raw mode");
        output::Output::clear_screen().expect("Error");
    }
}

fn main() -> io::Result<()> {
    let _clean_up = CleanUp;
    terminal::enable_raw_mode()?;
    let mut gameloop = gameloop::GameLoop::new();
    while gameloop.run()? {}
    Ok(())
}
