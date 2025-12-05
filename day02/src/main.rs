use std::thread;
use std::time::Instant;

fn parse_id_range(id_range: &str) -> (u64, u64) {
    let parts: Vec<&str> = id_range.split('-').collect();
    (
        parts[0].trim().parse::<u64>().unwrap(),
        parts[1].trim().parse::<u64>().unwrap(),
    )
}

fn sum_all_invalid_ids_part1(input: &str) -> u64 {
    input
        .split(',')
        .map(|id_range| {
            let (start, end) = parse_id_range(id_range);

            (start..=end)
                .filter(|id| {
                    let id_str = id.to_string();
                    id_str.len() % 2 == 0
                        && id_str[..id_str.len() / 2] == id_str[id_str.len() / 2..]
                })
                .sum::<u64>()
        })
        .sum()
}

fn sum_all_invalid_ids_part2(input: &str) -> u64 {
    input
        .split(',')
        .map(|id_range| {
            let (start, end) = parse_id_range(id_range);

            (start..=end).filter(is_repeated_pattern).sum::<u64>()
        })
        .sum()
}

fn sum_all_invalid_ids_threaded_part1(input: &str) -> u64 {
    let ranges: Vec<&str> = input.split(',').collect();

    let handles: Vec<_> = ranges
        .into_iter()
        .map(|id_range| {
            let id_range = id_range.to_string(); // copy to own the data

            thread::spawn(move || {
                let (start, end) = parse_id_range(&id_range);

                (start..=end)
                    .filter(|id| {
                        let s = id.to_string();
                        s.len() % 2 == 0 && s[..s.len() / 2] == s[s.len() / 2..]
                    })
                    .sum::<u64>()
            })
        })
        .collect();

    handles.into_iter().map(|h| h.join().unwrap()).sum()
}

fn sum_all_invalid_ids_threaded_part2(input: &str) -> u64 {
    let ranges: Vec<&str> = input.split(',').collect();

    let handles: Vec<_> = ranges
        .into_iter()
        .map(|id_range| {
            let id_range = id_range.to_string(); // copy to own the data

            thread::spawn(move || {
                let (start, end) = parse_id_range(&id_range);

                (start..=end).filter(is_repeated_pattern).sum::<u64>()
            })
        })
        .collect();

    handles.into_iter().map(|h| h.join().unwrap()).sum()
}

fn is_repeated_pattern(n: &u64) -> bool {
    let s = n.to_string();

    (1..=s.len() / 2)
        .find(|&pattern_len| {
            if s.len() % pattern_len != 0 {
                return false;
            }
            let pattern = &s[..pattern_len];

            s.as_bytes()
                .chunks(pattern_len)
                .all(|chunk| chunk == pattern.as_bytes())
        })
        .is_some()
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

    let start = Instant::now();
    let result = sum_all_invalid_ids_part1(&input);
    println!("Serial: {} in {:?}", result, start.elapsed());

    let start = Instant::now();
    let result = sum_all_invalid_ids_threaded_part1(&input);
    println!("Threaded: {} in {:?}", result, start.elapsed());

    let start = Instant::now();
    let result = sum_all_invalid_ids_part2(&input);
    println!("Serial: {} in {:?}", result, start.elapsed());

    let start = Instant::now();
    let result = sum_all_invalid_ids_threaded_part2(&input);
    println!("Threaded: {} in {:?}", result, start.elapsed());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works_serial_part1() {
        let input = "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124";
        assert_eq!(sum_all_invalid_ids_part1(input), 1227775554);
    }

    #[test]
    fn it_works_threaded_part1() {
        let input = "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124";
        assert_eq!(sum_all_invalid_ids_threaded_part1(input), 1227775554);
    }

    #[test]
    fn it_works_serial_part2() {
        let input = "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124";
        assert_eq!(sum_all_invalid_ids_part2(input), 43287141963);
    }

    #[test]
    fn it_works_threaded_part2() {
        let input = "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124";
        assert_eq!(sum_all_invalid_ids_threaded_part2(input), 43287141963);
    }
}
