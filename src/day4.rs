use std::{
    char,
    error::Error,
    path::Path,
};

use crate::util::read_lines;

const DELTAS: [(isize, isize); 8] = [
    (0, 1),
    (0, -1),
    (1, 0),
    (-1, 0),
    (1, 1),
    (-1, 1),
    (1, -1),
    (-1, -1),
];
const WORD: &str = "XMAS";

pub fn part1<P>(filename: P) -> Result<usize, Box<dyn Error>>
where
    P: AsRef<Path>,
{
    let mut grid: Vec<Vec<char>> = Vec::new();

    for line in read_lines(filename)? {
        grid.push(line?.chars().collect());
    }

    let rows = grid.len();
    if rows == 0 {
        return Ok(0);
    }
    let cols = grid[0].len();

    let word_chars: Vec<char> = WORD.chars().collect();
    let word_length = word_chars.len();
    let mut count = 0;

    for r in 0..rows {
        for c in 0..cols {
            if grid[r][c] == 'X' {
                for &(dx, dy) in DELTAS.iter() {
                    let mut match_found = true;
                    let mut current_r = r as isize;
                    let mut current_c = c as isize;

                    for i in 1..word_length {
                        current_r += dx;
                        current_c += dy;

                        if current_r < 0
                            || current_r >= rows as isize
                            || current_c < 0
                            || current_c >= cols as isize
                        {
                            match_found = false;
                            break;
                        }

                        if grid[current_r as usize][current_c as usize] != word_chars[i] {
                            match_found = false;
                            break;
                        }
                    }

                    if match_found {
                        count += 1;
                    }
                }
            }
        }
    }

    Ok(count)
}

pub fn part2<P>(filename: P) -> Result<usize, Box<dyn Error>>
where
    P: AsRef<Path>
{
    let mut grid: Vec<Vec<char>> = Vec::new();

    for line in read_lines(filename)? {
        grid.push(line.unwrap().chars().collect());
    }

    let rows = grid.len();
    let cols = grid[0].len();

    let mut count = 0;

    for r in 0..rows {
        for c in 0..cols {
            if grid[r][c] == 'A' {
                let diag1_valid = if r >= 1 && c >= 1 && r + 1 < rows && c + 1 < cols {
                    let top_left = grid[r - 1][c - 1];
                    let bottom_right = grid[r + 1][c + 1];
                    (top_left == 'M' && bottom_right == 'S') || (top_left == 'S' && bottom_right == 'M')
                } else {
                    false
                };

                let diag2_valid = if r >= 1 && c + 1 < cols && r + 1 < rows && c >= 1 {
                    let top_right = grid[r - 1][c + 1];
                    let bottom_left = grid[r + 1][c - 1];
                    (top_right == 'M' && bottom_left == 'S') || (top_right == 'S' && bottom_left == 'M')
                } else {
                    false
                };

                if diag1_valid && diag2_valid {
                    count += 1;
                }
            }
        }
    }

    Ok(count)
}
