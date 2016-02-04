// mod union_find;
// use union_find::*;

extern crate rand; // used for testing
use rand::random;

use std::fmt;

// Monte Carlo

/* Percolation problem
 * Make a square grid of white and black squares. To squares are
 * assumed to be connected if they are both white and share sides (but
 * not diagonals). What proportions need to be white in order for the any of
 * the upper squares to be connected to one of the lower squares?
 */

const L: usize = 4;

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
    sqs: [Square; L],
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
    rows: [Row; L],
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

fn get_color(grid: Grid, n: usize, m: usize) -> Square {
    assert!((n < L) & (m < L));
    grid.rows[n].sqs[m]
}

fn set_white(grid: &mut Grid, n: usize, m: usize) {
    grid.rows[n].sqs[m] = Square::White;
}

fn random_upto(n: usize) -> usize {
    random::<usize>() % n
}


fn main() {
    // Make grid
    let mut grid: Grid = Grid { rows: [Row { sqs: [Square::Black; L]}; L] };
    const N: usize = 3; // Number of white squares
    for i in 0..N {
        set_white(&mut grid, random_upto(L), random_upto(L));
    }
    println!("{:?}",grid);
}
