enum Method {
    Default,
    Click, // 0x434C49434B
}

fn solve(input: &str, method: Method) -> i32 {
    match method {
        Method::Default => {
            let mut pointer = 50;
            let mut zero_count = 0;

            for line in input.lines() {
                // split direction and distance
                let (direction, amount) = parse_line(line);

                match direction {
                    "R" => pointer += amount,
                    "L" => pointer -= amount,
                    _ => panic!("invalid direction"),
                }

                pointer = pointer.rem_euclid(100);

                if pointer == 0 {
                    zero_count += 1;
                }
            }

            zero_count
        }
        Method::Click => {
            let mut pointer = 50;
            let mut zero_passes = 0;

            for line in input.lines() {
                let (direction, amount) = parse_line(line);

                match direction {
                    "R" => {
                        pointer += amount;
                        zero_passes += pointer / 100;
                    }
                    "L" => {
                        let start = pointer;
                        pointer -= amount;
                        if start > 0 && amount >= start {
                            zero_passes += 1 + (amount - start) / 100;
                        } else if start == 0 {
                            zero_passes += amount / 100;
                        }
                    }
                    _ => panic!("invalid direction"),
                }

                pointer = pointer.rem_euclid(100);
            }

            zero_passes
        }
    }
}

fn parse_line(line: &str) -> (&str, i32) {
    // split direction and distance
    let direction = &line[0..1];
    let amount = line[1..].parse::<i32>().expect("not a number");

    (direction, amount)
}

fn main() {
    let args: Vec<String> = std::env::args().collect();

    let filename = args.get(1).expect("No filename provided");
    let method = match args.get(2).map(String::as_str) {
        Some("click") | Some("0x434C49434B") => Method::Click,
        _ => Method::Default,
    };

    // read the input file into a string
    let input = std::fs::read_to_string(filename).expect("Failed to read file");

    match method {
        Method::Default => {
            let result = solve(&input, method);
            println!(
                "The number of times the pointer stopped at zero is {}",
                result
            );
        }
        Method::Click => {
            let result = solve(&input, method);
            println!(
                "The number of times the pointer passed by zero is {}",
                result
            );
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_part1() {
        let test_input = "L68\nL30\nR48\nL5\nR60\nL55\nL1\nL99\nR14\nL82";
        assert_eq!(solve(test_input, Method::Default), 3);
    }

    #[test]
    fn test_part2() {
        let test_input = "L68\nL30\nR48\nL5\nR60\nL55\nL1\nL99\nR14\nL82";
        assert_eq!(solve(test_input, Method::Click), 6);
    }
}
