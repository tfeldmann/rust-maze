// Algorithm based on:
// https://weblog.jamisbuck.org/2011/1/27/maze-generation-growing-tree-algorithm

extern crate rand;

use rand::seq::SliceRandom;
use rand::thread_rng;

pub const UP: u8 = 0b0001;
pub const RIGHT: u8 = 0b0010;
pub const DOWN: u8 = 0b0100;
pub const LEFT: u8 = 0b1000;

struct Point {
    x: i64,
    y: i64,
}

struct Direction {
    passage: u8,
    opposite: u8,
    delta: Point,
}

const DIRECTIONS: [Direction; 4] = [
    Direction {
        passage: UP,
        opposite: DOWN,
        delta: Point { x: 0, y: 1 },
    },
    Direction {
        passage: RIGHT,
        opposite: LEFT,
        delta: Point { x: 1, y: 0 },
    },
    Direction {
        passage: DOWN,
        opposite: UP,
        delta: Point { x: 0, y: -1 },
    },
    Direction {
        passage: LEFT,
        opposite: RIGHT,
        delta: Point { x: -1, y: 0 },
    },
];

pub fn maze(width: i64, height: i64) -> Vec<Vec<u8>> {
    let mut grid: Vec<Vec<u8>> = vec![vec![0; width as usize]; height as usize];
    let mut cells: Vec<Point> = Vec::with_capacity(height as usize * width as usize);
    let mut dir_indices: Vec<u8> = (0..4).collect();

    // entry
    cells.push(Point { x: 0, y: 0 });
    grid[0][0] |= DOWN;

    while !cells.is_empty() {
        // we always start from the last cell
        let cell = &cells[cells.len() - 1];

        // shuffle directions
        dir_indices.shuffle(&mut thread_rng());

        let mut found = false;
        for &dir_index in dir_indices.iter() {
            let dir = &DIRECTIONS[dir_index as usize];

            let nx = cell.x + dir.delta.x;
            let ny = cell.y + dir.delta.y;

            // if new cell is unvisited carve passage
            if nx >= 0
                && ny >= 0
                && nx < width
                && ny < height
                && grid[ny as usize][nx as usize] == 0
            {
                grid[cell.y as usize][cell.x as usize] |= dir.passage;
                grid[ny as usize][nx as usize] |= dir.opposite;
                cells.push(Point { x: nx, y: ny });
                found = true;
                break;
            }
        }

        if !found {
            cells.pop();
        }
    }
    return grid;
}

pub fn print_grid(grid: Vec<Vec<u8>>) {
    for row in grid.iter() {
        for cell in row.iter() {
            print!("██{}", if (cell & DOWN) != 0 { "  " } else { "██" });
        }
        println!(
            "{}",
            if (row[row.len() - 1] & RIGHT) != 0 {
                "  "
            } else {
                "██"
            }
        );
        for cell in row.iter() {
            print!("{}  ", if (cell & LEFT) != 0 { "  " } else { "██" });
        }
        println!("██");
    }
    for _ in 0..grid[0].len() {
        print!("████");
    }
    println!("██");
}
