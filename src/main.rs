// mod union_find;
// use union_find::*;

use std::fmt;

// Monte Carlo

/* Percolation problem
 * Make a square grid of white and black squares. To squares are
 * assumed to be connected if they are both white and share sides (but
 * not diagonals). What proportions need to be white in order for the any of
 * the upper squares to be connected to one of the lower squares?
 */

const N: usize = 4;

#[derive(Copy, Clone)]
enum Square {
    White,
    Black,
}

impl fmt::Debug for Square {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Square::White => write!(f, " "),
            Square::Black => write!(f, "█"),
        }
    }
}

#[derive(Copy, Clone)]
struct Row {
    sqs: [Square; N],
}

impl fmt::Debug for Row {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f,"|");
        for sq in self.sqs.iter() {
            write!(f, "{:?}", sq);
        }
        write!(f,"|")
    }
}

#[derive(Copy, Clone)]
struct Grid {
    rows: [Row; N],
}

impl fmt::Debug for Grid {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "\n");
        for i in 0..self.rows.len() {
            write!(f, "{:?}", self.rows[i]);
            if i != (self.rows.len() - 1) {
                write!(f, "\n");
            }
        }
        write!(f, "\n")
    }
}

fn main() {
    // Make grid
    let mut grid: Grid = Grid { rows: [Row { sqs: [Square::Black; N]}; N] };
    println!("{:?}",grid);
}
