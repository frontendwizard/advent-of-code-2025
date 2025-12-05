fn main() {
    // read the input file into a string
    let input = std::fs::read_to_string("day01/input.txt").expect("Failed to read file");
    let mut pointer = 50;
    let mut zero_occurrences = 0;

    for line in input.lines() {
        // split direction and distance
        let direction = &line[0..1];
        let amount = line[1..].parse::<i32>().expect("not a number");

        match direction {
            "R" => pointer += amount,
            "L" => pointer -= amount,
            _ => panic!("invalid direction"),
        }

        // wrap the pointer 0-99
        pointer = pointer.rem_euclid(100);

        // check if the pointer is at zero
        if pointer == 0 {
            zero_occurrences += 1;
        }
    }

    println!("Zero occurrences: {}", zero_occurrences);
}
