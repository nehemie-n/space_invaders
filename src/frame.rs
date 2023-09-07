use crate::{NUM_COLS, NUM_ROWS};

pub type Frame = Vec<Vec<&'static str>>;

pub fn new_frame() -> Frame {
    let mut cols: Frame = Vec::with_capacity(NUM_ROWS);

    for _ in 0..NUM_COLS {
        let mut row = Vec::with_capacity(NUM_ROWS);
        for _ in 0..NUM_ROWS {
            row.push(" ")
        }
        cols.push(row);
    }
    return cols;
}

pub trait Drawable {
    fn draw(&self, frame: &mut Frame);
}
