use std::io::{Stdout, Write};

use crossterm::{
    cursor::MoveTo,
    style::{Color, SetBackgroundColor},
    terminal::{Clear, ClearType},
    QueueableCommand,
};

use crate::frame::Frame;

pub fn render(stdout: &mut Stdout, prev_frame: &Frame, new_frame: &Frame, force: bool) {
    if force {
        stdout.queue(SetBackgroundColor(Color::Cyan)).unwrap();
        stdout.queue(Clear(ClearType::All)).unwrap();
        stdout.queue(SetBackgroundColor(Color::Black)).unwrap();
    }

    for (x, col) in new_frame.iter().enumerate() {
        for (y, s) in col.iter().enumerate() {
            if *s != prev_frame[x][y] || force {
                stdout.queue(MoveTo(x as u16, y as u16)).unwrap();
                println!("{}", *s)
            }
        }
    }
    stdout.flush().unwrap();
}
