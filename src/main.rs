// mod union_find;
// use union_find::*;

extern crate rand; // used for testing
use rand::random;

mod union_find;
use union_find::*;

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

#[derive(Debug,Copy, Clone)]
struct Coords(usize, usize);

enum Dir {
    Left,
    Right,
    Up,
    Down,
}

const DIRS: [Dir; 4] = [Dir::Left, Dir::Right, Dir::Up, Dir::Down];

fn neighbour(c: &Coords, dir: &Dir) -> Coords {
    let bound = L - 1;
    let coords = *c;
    match *dir {
        Dir::Left => if coords.0 == 0 {
            coords.clone()
        } else {
            Coords(coords.0 - 1,coords.1)
        },
        Dir::Right => if coords.0 == bound {
            coords.clone()
        } else {
            Coords(coords.0 + 1,coords.1)
        },
        Dir::Up => if coords.1 == 0 {
            coords.clone()
        } else {
            Coords(coords.0, coords.1 - 1)
        },
        Dir::Down => if coords.1 == bound {
            coords.clone()
        } else {
            Coords(coords.0, coords.1 + 1)
        },
    }
}

fn get_color(grid: &Grid, coords: &Coords) -> Square {
    assert!((coords.0 < L) & (coords.1 < L));
    grid.rows[coords.0].sqs[coords.1]
}

fn set_white(grid: &mut Grid, coords: &Coords) {
    grid.rows[coords.0].sqs[coords.1] = Square::White;
}

// Mix Domains

fn coord_to_idx(coords: &Coords) -> usize {
    let out = coords.0 + coords.1 * L;
    assert!(out < (L * L));
    out
}

fn percolates(grid: &Grid, sets: &mut Sets) -> bool {
    let mut out = false;
    for i in 0..L {
        let top_coords = Coords(0, i);
        match get_color(grid, &top_coords) {
            Square::White => for j in 0..L {
                let low_coords = Coords(L-1, j);
                let top_idx = coord_to_idx(&top_coords);
                let low_idx = coord_to_idx(&low_coords);
                match get_color(grid, &low_coords) {
                    Square::White => if pc_quick_union::connected(sets, top_idx, low_idx) {
                        out = true;
                        break
                    },
                    Square::Black => (),
                }
            },
            Square::Black => (),
        }
        if out {
            break
        }
    }
    out
}

type Sets = [Node; (L * L)];

// Helper
fn random_upto(n: usize) -> usize {
    let out = random::<usize>() % n;
    assert!(out < n);
    out
}

fn main() {
    /* We will build the sets and the grid simultaneously and stop
     * when it percolates
     */

    // Start empty set
    let mut sets: Sets = [Node::Root(1); (L * L)];

    // Make grid
    let mut grid: Grid = Grid { rows: [Row { sqs: [Square::Black; L]}; L] };

    const N: usize = 10; // Number of white squares (to be changed)
    for i in 0..N {
        let coords = Coords(random_upto(L), random_upto(L));
        let idx = coord_to_idx(&coords);
        set_white(&mut grid, &coords);
        for d in DIRS.iter() {
            let d_coords = neighbour(&coords, d);
            match get_color(&grid, &d_coords) {
                Square::White => pc_quick_union::union(&mut sets, idx, coord_to_idx(&d_coords)),
                Square::Black => ()
            }
        }
    }
    println!("{:?}", percolates(&grid, &mut sets));
    println!("{:?}",grid);

}
