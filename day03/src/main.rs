fn max_voltage_two_digits(banks: &str) -> u64 {
    banks
        .lines()
        .map(|bank| -> u64 {
            let max_decimal = bank.chars().take(bank.len() - 1).max().unwrap();
            let max_decimal_position = bank.chars().position(|c| c == max_decimal).unwrap();
            let max_integer = bank.chars().skip(max_decimal_position + 1).max().unwrap();
            max_decimal.to_digit(10).unwrap() as u64 * 10 + max_integer.to_digit(10).unwrap() as u64
        })
        .sum()
}

fn max_voltage_n_digits(banks: &str, n: Option<usize>) -> u64 {
    let n = n.unwrap_or(2);
    banks
        .lines()
        .map(|bank| -> u64 {
            let mut joltage = 0;
            let mut start = 0;
            for i in 1..=n {
                let power = 10u64.pow((n - i) as u32);
                let remaining = n - i + 1; // digits still to pick (including current)
                let end = bank.len() - remaining + 1; // last valid position (exclusive)
                let window_size = end - start;
                let max_digit = bank.chars().skip(start).take(window_size).max().unwrap();
                let relative_position = bank
                    .chars()
                    .skip(start)
                    .take(window_size)
                    .position(|c| c == max_digit)
                    .unwrap();
                start = start + relative_position + 1;
                joltage += max_digit.to_digit(10).unwrap() as u64 * power;
            }
            joltage
        })
        .sum()
}

fn main() {
    let args: Vec<String> = std::env::args().collect();

    let filename = args
        .get(1)
        .expect("No filename provided")
        .trim()
        .to_string();

    // read the input file into a string
    let input = std::fs::read_to_string(filename)
        .expect("Failed to read file")
        .trim()
        .to_string();

    println!(
        "max voltage (2 digits): {}",
        max_voltage_n_digits(&input, Some(2))
    );

    println!(
        "max voltage (12 digits): {}",
        max_voltage_n_digits(&input, Some(12))
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_one() {
        let banks = "987654321111111\n811111111111119\n234234234234278\n818181911112111";
        assert_eq!(max_voltage_two_digits(&banks), 357);
    }

    #[test]
    fn part_one_generalized() {
        let banks = "987654321111111\n811111111111119\n234234234234278\n818181911112111";
        assert_eq!(max_voltage_n_digits(&banks, Some(2)), 357);
    }

    #[test]
    fn part_two() {
        let banks = "987654321111111\n811111111111119\n234234234234278\n818181911112111";
        assert_eq!(max_voltage_n_digits(&banks, Some(12)), 3121910778619);
    }
}
