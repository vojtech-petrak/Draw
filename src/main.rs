use std::io::stdout;
use crossterm::{
    ExecutableCommand,
    terminal,
};
use draw::{
    utils::SCREEN_SIZE,
    event::event_capture,
};

fn main() {
    terminal::enable_raw_mode().unwrap();
    stdout().execute(terminal::EnterAlternateScreen).unwrap();

    let (x, y): (u16, u16) = terminal::size().unwrap();
    unsafe { SCREEN_SIZE = [x as usize >> 1, y as usize - 3] }
    event_capture(None);
}
