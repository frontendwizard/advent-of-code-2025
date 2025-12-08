use std::{sync::Arc, thread, time::Instant};

fn file_input_to_string() -> String {
    let args: Vec<String> = std::env::args().collect();

    let filename = args
        .get(1)
        .expect("No filename provided")
        .trim()
        .to_string();

    // read the input file into a string
    std::fs::read_to_string(filename)
        .expect("Failed to read file")
        .trim()
        .to_string()
}

fn get_neighbour_rolls_count(grid: &Vec<Vec<char>>, position: (usize, usize)) -> i32 {
    let (row, col) = position;
    let width = grid[0].len();
    let height = grid.len();
    let directions: [(isize, isize); 8] = [
        (-1, -1),
        (-1, 0),
        (-1, 1),
        (0, -1),
        (0, 1),
        (1, -1),
        (1, 0),
        (1, 1),
    ];

    directions
        .iter()
        .filter_map(|(dr, dc)| {
            let nr = row as isize + dr;
            let nc = col as isize + dc;
            if nr >= 0 && nc >= 0 && (nr as usize) < height && (nc as usize) < width {
                Some((nr as usize, nc as usize))
            } else {
                None
            }
        })
        .filter(|(r, c)| grid[*r][*c] == '@')
        .count() as i32
}

fn find_accessible_rolls(input: &str) -> (String, i32) {
    let grid = parse_input(&input);
    let mut accessible_rolls = String::new();
    let mut x_count = 0;

    for row in 0..grid.len() {
        for col in 0..grid[row].len() {
            if grid[row][col] == '@' {
                let neighbour_rolls_count = get_neighbour_rolls_count(&grid, (row, col));
                if neighbour_rolls_count < 4 {
                    accessible_rolls.push('x');
                    x_count += 1;
                } else {
                    accessible_rolls.push(grid[row][col]);
                }
            } else {
                accessible_rolls.push(grid[row][col]);
            }
        }
        // if not the last row
        if row < grid.len() - 1 {
            accessible_rolls.push('\n');
        }
    }

    (accessible_rolls, x_count)
}

fn find_accessible_rolls_thread_per_cell(input: &str) -> (String, i32) {
    let grid = Arc::new(parse_input(&input));
    let height = grid.len();
    let width = grid[0].len();

    let handles: Vec<_> = (0..height)
        .flat_map(|row| {
            let grid = Arc::clone(&grid);
            (0..width).filter_map(move |col| {
                let grid = Arc::clone(&grid);
                if grid[row][col] == '@' {
                    Some(thread::spawn(move || {
                        let count = get_neighbour_rolls_count(&grid, (row, col));
                        if count < 4 { Some((row, col)) } else { None }
                    }))
                } else {
                    None
                }
            })
        })
        .collect();

    let accessible_rolls: Vec<_> = handles
        .into_iter()
        .filter_map(|h| h.join().unwrap())
        .collect();
    // count x
    let count = accessible_rolls.len() as i32;

    // rebuild grid
    let mut new_grid = (*grid).clone();
    for (row, col) in accessible_rolls {
        new_grid[row][col] = 'x';
    }

    // grid to string
    let grid_str = new_grid
        .iter()
        .map(|row| row.iter().collect::<String>())
        .collect::<Vec<String>>()
        .join("\n");

    (grid_str, count)
}

fn find_accessible_rolls_thread_per_row(input: &str) -> (String, i32) {
    let grid = Arc::new(parse_input(&input));
    let height = grid.len();
    let width = grid[0].len();

    let handles: Vec<_> = (0..height)
        .map(|row| {
            let grid = Arc::clone(&grid);
            thread::spawn(move || {
                (0..width)
                    .map(|col| {
                        if grid[row][col] == '@' {
                            let count = get_neighbour_rolls_count(&grid, (row, col));
                            if count < 4 { 'x' } else { '@' }
                        } else {
                            grid[row][col]
                        }
                    })
                    .collect::<Vec<_>>()
            })
        })
        .collect();

    let results: Vec<_> = handles.into_iter().map(|h| h.join().unwrap()).collect();
    // count x
    let x_count = results.iter().flatten().filter(|&&c| c == 'x').count() as i32;

    // grid to string
    let grid_str = results
        .iter()
        .map(|row| row.iter().collect::<String>())
        .collect::<Vec<String>>()
        .join("\n");

    (grid_str, x_count)
}

fn parse_input(input: &str) -> Vec<Vec<char>> {
    input.lines().map(|line| line.chars().collect()).collect()
}

fn main() {
    let input = file_input_to_string();
    let start = Instant::now();
    let (_, x_count) = find_accessible_rolls(&input);
    println!(
        "Number of accessible rolls (serial): {} in {:?}",
        x_count,
        start.elapsed()
    );

    let start = Instant::now();
    let (_, x_count) = find_accessible_rolls_thread_per_cell(&input);
    println!(
        "Number of accessible rolls (thread per cell): {} in {:?}",
        x_count,
        start.elapsed()
    );

    let start = Instant::now();
    let (_, x_count) = find_accessible_rolls_thread_per_row(&input);
    println!(
        "Number of accessible rolls (thread per row): {} in {:?}",
        x_count,
        start.elapsed()
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let input = "..@@.@@@@.
@@@.@.@.@@
@@@@@.@.@@
@.@@@@..@.
@@.@@@@.@@
.@@@@@@@.@
.@.@.@.@@@
@.@@@.@@@@
.@@@@@@@@.
@.@.@@@.@.";
        let output = String::from(
            "..xx.xx@x.
x@@.@.@.@@
@@@@@.x.@@
@.@@@@..@.
x@.@@@@.@x
.@@@@@@@.@
.@.@.@.@@@
x.@@@.@@@@
.@@@@@@@@.
x.x.@@@.x.",
        );
        assert_eq!(find_accessible_rolls(&input), (output, 13 as i32));
    }
}
