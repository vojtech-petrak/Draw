use std::{
    cmp::max,
    io::{stdout, Write},
};
use crossterm::{QueueableCommand, cursor};
use crate::event::event_capture;

pub const X: usize = 0;
pub const Y: usize = 1;

// input
pub static mut SCREEN_SIZE: [usize; 2] = [0; 2];
pub enum PrintType {
    Input,
    Output,
}

const FILE_TYPE: &str = ".draw";
const FILE_TYPE_LENGHT: usize = FILE_TYPE.len();

pub fn print(string: &str, purpose: PrintType) {
    stdout().queue(cursor::MoveTo(0, unsafe { SCREEN_SIZE[Y] } as u16 - purpose as u16 + 1)).unwrap();
    print!("                                                                \r{}", string);
    stdout().flush().unwrap();
}
pub fn input_usize(property: &str, max: usize) -> Option<usize> {
    loop {
        print(&(property.to_owned() + " = "), PrintType::Input);
        if let Some(input) = event_capture(Some("".to_owned())) {

            match input.parse::<usize>() {
                Ok(number) => {
                    if number <= max {
                        return Some(number);
                    } else {
                        print(&(number.to_string() + " is out of bound"), PrintType::Output);
                    }
                },
                Err(_) => print(&(input + " is an invalid value"), PrintType::Output),
            }
        } else {
            return None;
        }
    }
}
pub fn input_file_name() -> Option<String> {
    print("file name = ", PrintType::Input);
    if let Some(mut file_name) = event_capture(Some("".to_owned())) {
        let file_name_lenght = file_name.len();
        
        if file_name[max(file_name_lenght, FILE_TYPE_LENGHT) - FILE_TYPE_LENGHT..file_name_lenght].to_string()
        != FILE_TYPE.to_string() {
            file_name += FILE_TYPE;
        }
        return Some(file_name);
    } else { None }
}

// conversions
pub fn point_to_index(canvas_width: usize, point: &[usize; 2]) -> usize {
    return point[Y] * canvas_width + point[X];
}
pub fn bytes_to_bits(bytes: &Vec<u8>) -> Vec<bool> {
    let byte_count: usize = bytes.len();
    let mut bits: Vec<bool> = vec![false; byte_count << 3];
    /*
    for byte_index in 0..byte_count {
        for bit_index in 0..8 {
            bits[(byte_index << 3) + bit_index] = match (bytes[byte_index] >> bit_index) & 1 {
                1 => true,
                _ => false,
            };
        }
    }
    */
    for bit_index in 0..bits.len() {
        bits[bit_index] = match (bytes[bit_index >> 3] >> (bit_index & 7)) & 1 {
            1 => true,
            _ => false,
        }
    }
    return bits;
}
pub fn bits_to_bytes(bits: &Vec<bool>) -> Vec<u8> {
    let byte_count: usize = (bits.len() + 7) >> 3;
    let mut bytes: Vec<u8> = vec![0; byte_count];
    /*
    for byte_index in 0..byte_count {
        for bit_index in 0..8 {
            bytes[byte_index] |= (bits[(byte_index << 3) + bit_index] as u8) << bit_index;
        }
    }
    */
    for bit_index in 0..bits.len() {
        bytes[bit_index >> 3] |= (bits[bit_index] as u8) << (bit_index & 7);
    }
    return bytes;
}
pub fn print_help() {
    stdout().queue(cursor::MoveTo(0, 0)).unwrap();
    print!("Welcome to Draw!\n");
    print!("\nShortcuts:\n");
    print!("    New canvas: Ctrl + N\n");
    print!("    Open canvas: Ctrl + O\n");
    print!("    Save canvas: Ctrl + S\n");
    print!("    Close canvas: Ctrl + W\n");
    print!("    Exit: Ctrl + F4\n");
    print!("\nControls:\n");
    print!("    Draw: Insert\n");
    print!("    Erase: Delete\n");
    print!("    Invert: Space\n");
    print!("    Move cursor: Arrows\n");
    print!("    Move canvas: Ctrl + Arrows\n");
    print!("\nInput:\n");
    print!("    Confirm: Enter\n");
    print!("    Cancel: Esc\n");
    stdout().flush().unwrap();
}
