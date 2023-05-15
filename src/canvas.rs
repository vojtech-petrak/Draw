use std::{
    cmp::{min, max},
    io::{stdout, Write, Error, prelude::*, ErrorKind},
    fs::{File, OpenOptions},
};
use crossterm::{QueueableCommand, cursor};
use crate::utils::*;

pub enum Direction {
    Start,
    End,
}
pub struct CanvasFile {
    name: String,
    file: Option<File>,
}
pub struct Canvas {
    pub canvas_file: Option<CanvasFile>,
    pub span: [usize; 2],
    pub pixels: Vec<bool>,
    pub position: [usize; 2],
    pub cursor: [usize; 2],
}

impl Canvas {
    // file
    pub fn new() -> Option<Canvas> {
        let span: [usize; 2] = [
            match input_usize("canvas width", 255) {
                Some(x) => x,
                None => return None,
            },
            match input_usize("canvas height", 255) {
                Some(y) => y,
                None => return None,
            },
        ];
        return Some(Canvas {
            canvas_file: None,
            span,
            pixels: vec![false; span[X] * span[Y]],
            position: [0; 2],
            cursor: [0; 2]
        });
    }
    pub fn open(name: &str) -> Result<Canvas, Error> {
        let mut file: File = OpenOptions::new().read(true).write(true).open(&name)?;
        let mut content: Vec<u8> = Vec::<u8>::new();

        file.read_to_end(&mut content)?;
        let span: [usize; 2] = [match content.pop() {
            Some(dimension) => dimension,
            None => return Err(ErrorKind::InvalidData.into()),
        } as usize; 2];

        if content.len() != span[X] * span[Y] { return Err(ErrorKind::InvalidData.into()); }
        return Ok(Canvas {
            canvas_file: Some(CanvasFile { name: name.to_owned(), file: Some(file) } ),
            span,
            pixels: bytes_to_bits(&content),
            position: [0; 2],
            cursor: [0; 2]
        });
    }
    pub fn save(canvas: &mut Canvas) -> Result<Option<()>, Error> {
        let mut content: Vec<u8> = bits_to_bytes(&canvas.pixels);
        content.push(canvas.span[Y] as u8);
        content.push(canvas.span[X] as u8);

        if let Some(ref mut canvas_file) = canvas.canvas_file {
            if let Some(ref mut file) = canvas_file.file {
                file.write_all(&content)?;
            } else {
                canvas.canvas_file = Some(CanvasFile {
                    name: (&canvas_file.name).to_string(),
                    file: Some(OpenOptions::new().read(true).write(true).create(true).open(&canvas_file.name)?),
                });
            }
        } else if let Some(name) = input_file_name() {
            canvas.canvas_file = Some( CanvasFile {
                file: Some(OpenOptions::new().read(true).write(true).create(true).open(&name)?),
                name,
            });
        } else {
            return Ok(None);
        }
        Ok(Some(()))
    }

    // output
    fn cursor_set(canvas: &Canvas) -> Result<(), Error> {
        stdout().queue(cursor::MoveTo(
            ((canvas.cursor[X] - canvas.position[X]) << 1) as u16,
            (canvas.cursor[Y] - canvas.position[Y]) as u16
        ))?;
        Ok(())
    }
    fn print(canvas: &Canvas) {
        stdout().queue(cursor::MoveTo(0, 0)).unwrap();
        for row_index in canvas.position[Y]..min(canvas.span[Y], unsafe { SCREEN_SIZE[Y] }) + canvas.position[Y] {
            for column_index in canvas.position[X]..min(canvas.span[X], unsafe { SCREEN_SIZE[X] } ) + canvas.position[X] {
                print!("{}", {
                    if canvas.pixels[point_to_index(canvas.span[X], &[column_index, row_index ])] { "██" }
                    else { "  " }
                });
            }
            print!("\n");
        }
        stdout().flush().unwrap();
    }
    pub fn shift(canvas: &mut Canvas, axis: usize, direction: Direction) -> Result<(), Error> {
        match direction {
            Direction::Start => if canvas.position[axis] != 0 {
                canvas.position[axis] -= 1;
            },
            Direction::End => if canvas.position[axis]
            < max(0, canvas.span[axis] as isize - unsafe { SCREEN_SIZE[axis] } as isize) as usize {
                canvas.position[axis] += 1;
            }
        }
        Canvas::print(canvas);
        Canvas::cursor_set(canvas)?;
        print!("‡‡");
        stdout().flush()?;
        return Ok(());
    }
    pub fn cursor_move(canvas: &mut Canvas, axis: usize, direction: Direction) -> Result<(), Error> {
        Canvas::cursor_set(canvas).unwrap();
        print!("{}", {
            if canvas.pixels[point_to_index(canvas.span[X], &canvas.cursor)] { "██" }
            else { "  " }
        });
        match direction {
            Direction::Start => {
                if canvas.cursor[axis] == canvas.position[axis] {
                    canvas.cursor[axis] = min(
                        canvas.span[axis],
                        unsafe { SCREEN_SIZE[axis] }
                    ) + canvas.position[axis];
                }
                canvas.cursor[axis] -= 1;
            },
            Direction::End => {
                canvas.cursor[axis] += 1;
                if canvas.cursor[axis] == min(
                    canvas.span[axis],
                    unsafe { SCREEN_SIZE[axis] }
                ) + canvas.position[axis] {
                    canvas.cursor[axis] = canvas.position[axis];
                }
            },
        }
        Canvas::cursor_set(canvas)?;
        print!("‡‡");
        stdout().flush()?;
        return Ok(());
    }
    pub fn display(canvas: &Canvas) {
        Canvas::print(canvas);
        stdout().queue(cursor::MoveTo(0, 0)).unwrap();
        print!("‡‡");
        stdout().flush().unwrap();
    }
}
