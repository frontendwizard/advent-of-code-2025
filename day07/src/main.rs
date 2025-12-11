use std::collections::{HashMap, HashSet};

fn load_input_file() -> String {
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

fn main() {
    let input = load_input_file();
    println!("The beam splits {} times", part_one(&input));
    println!("The number of possible timelines is {}", part_two(&input));
}

fn part_one(input: &str) -> i32 {
    let lines = input.lines().collect::<Vec<&str>>();
    let grid: Vec<Vec<char>> = lines.iter().map(|line| line.chars().collect()).collect();
    let mut split_count = 0;

    let mut beam_columns_idx: HashSet<usize> = HashSet::new();
    beam_columns_idx.insert(
        grid[0]
            .iter()
            .position(|c| *c == 'S')
            .expect("No starting beam found"),
    );

    for row_idx in 1..grid.len() {
        let mut next: HashSet<usize> = HashSet::new();
        for col_idx in &beam_columns_idx {
            match grid[row_idx][*col_idx] {
                '^' => {
                    next.insert(col_idx + 1);
                    next.insert(col_idx - 1);
                    split_count += 1;
                }
                _ => {
                    next.insert(*col_idx);
                }
            }
        }
        beam_columns_idx = next;
    }

    split_count
}

fn part_two(input: &str) -> i64 {
    let lines = input.lines().collect::<Vec<&str>>();
    let grid: Vec<Vec<char>> = lines.iter().map(|line| line.chars().collect()).collect();

    let mut timelines: HashMap<usize, i64> = HashMap::new();
    timelines.insert(
        grid[0]
            .iter()
            .position(|c| *c == 'S')
            .expect("No starting beam found"),
        1,
    );

    for row in 1..grid.len() {
        let mut next: HashMap<usize, i64> = HashMap::new();

        for (&col, &count) in &timelines {
            match grid[row][col] {
                '^' => {
                    *next.entry(col - 1).or_insert(0) += count;
                    *next.entry(col + 1).or_insert(0) += count;
                }
                _ => {
                    *next.entry(col).or_insert(0) += count;
                }
            }
        }

        timelines = next;
    }

    timelines.values().sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = [
            ".......S.......",
            "...............",
            ".......^.......",
            "...............",
            "......^.^......",
            "...............",
            ".....^.^.^.....",
            "...............",
            "....^.^...^....",
            "...............",
            "...^.^...^.^...",
            "...............",
            "..^...^.....^..",
            "...............",
            ".^.^.^.^.^...^.",
            "...............",
        ]
        .join("\n");
        let expected_output = 21;

        assert_eq!(part_one(&input), expected_output);
    }

    #[test]
    fn test_part_two() {
        let input = [
            ".......S.......",
            "...............",
            ".......^.......",
            "...............",
            "......^.^......",
            "...............",
            ".....^.^.^.....",
            "...............",
            "....^.^...^....",
            "...............",
            "...^.^...^.^...",
            "...............",
            "..^...^.....^..",
            "...............",
            ".^.^.^.^.^...^.",
            "...............",
        ]
        .join("\n");
        let expected_output = 40;

        assert_eq!(part_two(&input), expected_output);
    }
}
