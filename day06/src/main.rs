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
    println!("Grand total: {}", part_two(&input));
}

fn part_one(input: &str) -> i64 {
    let lines = input
        .lines()
        .map(|line| line.split_whitespace().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    (0..lines[0].len())
        .map(|idx| {
            let numbers = lines[0..lines.len() - 1]
                .iter()
                .map(|line| line[idx].parse::<i64>().expect("Failed to parse number"))
                .collect::<Vec<_>>();
            let operator = lines[lines.len() - 1][idx];

            let result = match operator {
                "*" => numbers.iter().product::<i64>(),
                "+" => numbers.iter().sum::<i64>(),
                _ => panic!("Invalid operator"),
            };

            result
        })
        .sum::<i64>()
}

fn part_two(input: &str) -> i64 {
    let lines = input.lines().collect::<Vec<_>>();

    let digit_columns = (0..lines[0].len())
        .map(|col_idx| {
            lines[0..lines.len() - 1]
                .iter()
                .map(|line| match line.chars().nth(col_idx).unwrap_or(' ') {
                    ' ' => String::new(),
                    c => c.to_string(),
                })
                .collect::<String>()
        })
        .collect::<Vec<_>>();

    let problems = digit_columns
        .split(|value| value.is_empty())
        .collect::<Vec<_>>();

    let operators = lines.last().unwrap().split_whitespace().collect::<Vec<_>>();

    problems
        .iter()
        .enumerate()
        .map(|(idx, problem_numbers)| {
            let numbers = problem_numbers
                .iter()
                .map(|s| s.parse::<i64>().unwrap())
                .collect::<Vec<_>>();

            let operator = operators[idx];
            let result = match operator {
                "+" => numbers.iter().sum::<i64>(),
                "*" => numbers.iter().product::<i64>(),
                _ => panic!("Invalid operator"),
            };

            result
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = [
            "123 328  51 64 ",
            " 45 64  387 23 ",
            "  6 98  215 314",
            "*   +   *   +  ",
        ]
        .join("\n");
        let expected_output = 4277556;

        assert_eq!(part_one(&input), expected_output);
    }

    #[test]
    fn test_part_two() {
        let input = [
            "123 328  51 64 ",
            " 45 64  387 23 ",
            "  6 98  215 314",
            "*   +   *   +  ",
        ]
        .join("\n");
        let expected_output = 3263827;

        assert_eq!(part_two(&input), expected_output);
    }
}
