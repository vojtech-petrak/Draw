use std::io::{stdout, Write};
use crossterm::{
    QueueableCommand,
    ExecutableCommand,
    cursor,
    event::{read, KeyEvent, Event, KeyCode, KeyEventKind, KeyModifiers},
    terminal,
};
use crate::{
    utils::*,
    canvas::*,
};
#[derive(PartialEq)]
enum Action {
    None,
    Draw,
    Erase,
    Invert,
}

pub fn event_capture(mut input: Option<String>)  -> Option<String> {
    let mut canvas: Option<Canvas> = None;
    let mut action: Action = Action::None;
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
                                KeyModifiers::NONE => Canvas::cursor_move(canvas, direction.0, direction.1),
                                KeyModifiers::SHIFT => (),
                                KeyModifiers::CONTROL => Canvas::shift(canvas, direction.0, direction.1),
                                _ => ()
                            }
                            match action {
                                Action::None => (),
                                Action::Draw => canvas.pixels[point_to_index(canvas.span[X], &canvas.cursor)] = true,
                                Action::Erase => canvas.pixels[point_to_index(canvas.span[X], &canvas.cursor)] = false,
                                Action::Invert => canvas.pixels[point_to_index(canvas.span[X], &canvas.cursor)] = {
                                    if canvas.pixels[point_to_index(canvas.span[X], &canvas.cursor)] == true { false }
                                    else { true }
                                },
                            }
                        }
                        match code {
                            KeyCode::Insert if action != Action::Draw => {
                                canvas.pixels[point_to_index(canvas.span[X], &canvas.cursor)] = true;
                                action = Action::Draw;
                            },
                            KeyCode::Delete if action != Action::Erase => {
                                canvas.pixels[point_to_index(canvas.span[X], &canvas.cursor)] = false;
                                action = Action::Erase;
                            },
                            KeyCode::Char(' ') if action != Action::Invert => {
                                canvas.pixels[point_to_index(canvas.span[X], &canvas.cursor)] = {
                                    if canvas.pixels[point_to_index(canvas.span[X], &canvas.cursor)] == true { false }
                                    else { true }
                                };
                                action = Action::Invert;
                            },
                            _ => (),
                        }
                    } else if kind == KeyEventKind::Release
                    && (code == KeyCode::Insert || code == KeyCode::Delete || code == KeyCode::Char(' ')) {
                        action = Action::None;
                    }
                }
                if (!modifiers.contains(KeyModifiers::CONTROL) || modifiers.contains(KeyModifiers::ALT))
                && kind == KeyEventKind::Press {
                    if let Some(ref mut string) = input {
                        match code {
                            KeyCode::Char(char) => {
                                string.push(char);
                                print!("{}", char);
                                stdout().flush().unwrap();
                            },
                            KeyCode::Backspace => if let Some(_) = string.pop() {
                                stdout().queue(cursor::MoveLeft(1)).unwrap();
                                print!(" ");
                                stdout().queue(cursor::MoveLeft(1)).unwrap();
                                stdout().flush().unwrap();
                            },
                            KeyCode::Enter => return input,
                            KeyCode::Esc => {
                                print("", PrintType::Input);
                                return None;
                            },
                            _ => (),
                        }
                        input = Some(string.to_string());
                    }
                } else if modifiers == KeyModifiers::CONTROL && kind == KeyEventKind::Press && input == None {
                    match code {
                        KeyCode::Char('n') => {
                            print("creating canvas", PrintType::Output);
                            print(match Canvas::new() {
                                Some(canvas_new) => {
                                    Canvas::display(&canvas_new);
                                    canvas = Some(canvas_new);
                                    "canvas created"
                                },
                                None => "canvas creation canceled",
                            }, PrintType::Output);
                        },
                        KeyCode::Char('o') => {
                            print("opening file", PrintType::Output);
                            match Canvas::open() {
                                Ok(canvas_new) => {
                                    stdout().execute(terminal::Clear(terminal::ClearType::All)).unwrap();
                                    Canvas::display(&canvas_new);
                                    canvas = Some(canvas_new);
                                    print("file opened", PrintType::Output);
                                },
                                Err(error) => print(&error.to_string(), PrintType::Output),
                            }
                        },
                        KeyCode::Char('s') => match canvas {
                            Some(ref mut canvas) => {
                                print("saving file", PrintType::Output);
                                match Canvas::save(canvas) {
                                    Ok(_) => print("file saved", PrintType::Output),
                                    Err(error) => print(&error.to_string(), PrintType::Output),
                                }
                            },
                            None => print("nothing to save", PrintType::Output),
                        },
                        KeyCode::Char('w') => {
                            match canvas {
                                Some(_) => {
                                    canvas = None;
                                    stdout().execute(terminal::Clear(terminal::ClearType::All)).unwrap();
                                    print("canvas closed", PrintType::Output);
                                    print_help();
                                },
                                None => print("nothing to close", PrintType::Output),
                            }
                            if let Some(_) = input {
                                return None;
                            }
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
