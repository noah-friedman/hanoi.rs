use std::io::{stdout, Write};
use termion::{cursor::DetectCursorPos, screen::IntoAlternateScreen};
use hanoi_rs::{Pole, Disk};

const TITLE: &'static str = "Welcome to hanoi.rs
Created by Noah Friedman | © 2023
";

fn main() {
    // Create Pole objects in an array
    let mut p: [Pole; 3] = [Pole::new(), Pole::new(), Pole::new()];

    // Fill first pole with 8 disks
    p[0].fill(8);

    // Check if cursor position is obtainable, if not then switch to alternate screen
    let (mut y_offset, mut out): (u16, Box<dyn Write>) = match stdout().cursor_pos() {
        Ok((_, y)) => (y, Box::new(stdout())),
        Err(e) => {
            eprintln!("failed to detect cursor position with error \"{}\"; falling back to alternate screen", e);

            (1, Box::new(stdout().into_alternate_screen().unwrap()))
        }
    };

    // Adjust for title message in y_offset
    y_offset += TITLE.lines().count() as u16;

    // Print title message
    write!(out, "{}", TITLE).unwrap();

    
}
