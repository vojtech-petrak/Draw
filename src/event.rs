use std::io::stdout;
use crossterm::{
    ExecutableCommand,
    event::{read, KeyEvent, Event, KeyCode, KeyEventKind, KeyModifiers},
    terminal,
};
use crate::{
    utils::*,
    canvas::*,
};

pub fn event_capture(mut input: Option<String>)  -> Option<String> {
    let mut canvas: Option<Canvas> = None;
    loop {
        match read() {
            Ok(Event::Key(KeyEvent { code, modifiers, kind, .. })) => {
                if let Some(ref mut canvas) = canvas {
                    if kind == KeyEventKind::Press {
                        let direction: Option<(usize, Direction)> = match code {
                            KeyCode::Left => Some((X, Direction::Start)),
                            KeyCode::Right => Some((X, Direction::End)),
                            KeyCode::Up => Some((Y, Direction::Start)),
                            KeyCode::Down => Some((Y, Direction::End)),
                            _ => None,
                        };
                        if let Some(direction) = direction {
                            match modifiers {
                                KeyModifiers::NONE => Canvas::cursor_move(canvas, direction.0, direction.1).unwrap(),
                                KeyModifiers::SHIFT => (),
                                KeyModifiers::CONTROL => Canvas::shift(canvas, direction.0, direction.1).unwrap(),
                                _ => ()
                            }
                        }
                        match code {
                            KeyCode::Insert => canvas.pixels[point_to_index(canvas.span[X], &canvas.cursor)] = true,
                            KeyCode::Delete => canvas.pixels[point_to_index(canvas.span[X], &canvas.cursor)] = false,
                            KeyCode::Char(' ') => canvas.pixels[point_to_index(canvas.span[X], &canvas.cursor)] = {
                                if canvas.pixels[point_to_index(canvas.span[X], &canvas.cursor)] == true { false }
                                else { true }
                            },
                            _ => ()
                        }
                    }
                }
                if (modifiers == KeyModifiers::NONE || modifiers == KeyModifiers::SHIFT || modifiers == KeyModifiers::ALT)
                && kind == KeyEventKind::Press {
                    if let Some(ref mut string) = input {
                        match code {
                            KeyCode::Char(char) => string.push(char),
                            KeyCode::Backspace => { string.pop(); },
                            KeyCode::Enter => return input,
                            KeyCode::Esc => return None,
                            _ => (),
                        }
                        input = Some(string.to_string());
                    }

                } else if modifiers == KeyModifiers::CONTROL && kind == KeyEventKind::Press {
                    match code {
                        KeyCode::Char('n') => {
                            canvas = Canvas::new();
                            print(if let Some(ref canvas) = canvas {
                                Canvas::display(&canvas);
                                "canvas created".to_owned()
                            } else {
                                "canvas creation canceled".to_owned()
                            }, PrintType::Output)
                        },
                        KeyCode::Char('o') => {
                            match input_file_name() {
                                Some(name) => match Canvas::open(&name) {
                                    Ok(canvas_) => {
                                        Canvas::display(&canvas_);
                                        print("file opened".to_owned(), PrintType::Output);
                                        canvas = Some(canvas_);
                                    },
                                    Err(error) => print(error.to_string(), PrintType::Output),
                                },
                                None => print("file opening canceled".to_owned(), PrintType::Output),
                            }
                        },
                        KeyCode::Char('s') => {
                            if let Some(ref mut canvas) = canvas {
                                match Canvas::save(canvas) {
                                    Ok(_) => print("file saved".to_owned(), PrintType::Output),
                                    Err(error) => print(error.to_string(), PrintType::Output),
                                }
                            }
                        },
                        KeyCode::Char('w') => {
                            stdout().execute(terminal::Clear(terminal::ClearType::All)).unwrap();
                            canvas = None;
                        },
                        KeyCode::F(4) => panic!("program terminated"),
                        _ => ()
                    }
                }
            }
            Ok(Event::Resize(x, y)) => {
                unsafe { SCREEN_SIZE = [x as usize >> 1, y as usize - 3] }
            }
            _ => (),
        }
    }
}
