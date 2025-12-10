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

fn parse_input(input: &str) -> (Vec<(i64, i64)>, Vec<i64>) {
    let parts: Vec<&str> = input.split("\n\n").collect();

    let mut fresh_items: Vec<(i64, i64)> = parts[0]
        .split("\n")
        .map(|range_line| {
            let range: Vec<&str> = range_line.split("-").collect();
            (
                range[0].parse::<i64>().expect("Invalid range"),
                range[1].parse::<i64>().expect("Invalid range"),
            )
        })
        .collect();

    fresh_items.sort();

    let inventory = parts[1]
        .split("\n")
        .map(|item| item.parse::<i64>().expect("Invalid item"))
        .collect();

    (fresh_items, inventory)
}

fn count_stale_items(input: &str) -> i64 {
    let (fresh_items, inventory) = parse_input(input);
    let mut count = 0;

    for item_id in inventory {
        let idx = fresh_items.partition_point(|&(start, _)| start <= item_id);
        for (start, end) in &fresh_items[0..idx] {
            if *start <= item_id && item_id <= *end {
                count += 1;
                break;
            }
        }
    }

    count
}

fn count_fresh_items(input: &str) -> i64 {
    let (fresh_items, _) = parse_input(input);

    // walk though the fresh item ranges and merge them if they overlap
    let mut merged_ranges = Vec::new();
    let mut current_range = fresh_items[0];

    for next_range in fresh_items.iter().skip(1) {
        if current_range.1 + 1 >= next_range.0 {
            current_range.1 = current_range.1.max(next_range.1);
        } else {
            merged_ranges.push(current_range);
            current_range = (next_range.0, next_range.1);
        }
    }
    merged_ranges.push(current_range);

    // count all the idx in all the ranges
    let mut count = 0;
    for &(start, end) in &merged_ranges {
        count += (end - start + 1) as i64;
    }

    count
}

fn main() {
    let input = load_input_file();
    println!("Stale items: {}", count_stale_items(&input));
    println!("Fresh items: {}", count_fresh_items(&input));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_count_stale_items() {
        let input = "3-5\n10-14\n16-20\n12-18\n\n1\n5\n8\n11\n17\n32";
        let output = 3;
        assert_eq!(count_stale_items(input), output);
    }

    #[test]
    fn test_count_fresh_items() {
        let input = "3-5\n10-14\n16-20\n12-18\n\n1\n5\n8\n11\n17\n32";
        let output = 14;
        assert_eq!(count_fresh_items(input), output);
    }
}
