const START_POSITION: i32 = 50;
const DIAL_SIZE: i32 = 100;

fn part1(input: &str) {
    let mut dial = START_POSITION;
    let mut activation_count = 0;

    for line in input.lines() {
        let (direction, distance_str) = line.split_at(1);
        let distance: i32 = distance_str.trim().parse().expect("Invalid distance");

        dial = match direction {
            "L" => (dial - distance).rem_euclid(DIAL_SIZE),
            "R" => (dial + distance).rem_euclid(DIAL_SIZE),
            _ => panic!("Invalid direction"),
        };

        if dial == 0 {
            activation_count += 1;
        }
    }

    println!("Part 1: {}", activation_count);
}

fn part2(input: &str) {
    let mut dial = START_POSITION;
    let mut activation_count = 0;

    for line in input.lines() {
        let (direction, distance_str) = line.split_at(1);
        let distance: i32 = distance_str.trim().parse().expect("Invalid distance");

        for _ in 0..distance {
            match direction {
                "L" => {
                    dial -= 1;
                    if dial < 0 {
                        dial = DIAL_SIZE - 1;
                    }
                }
                "R" => {
                    dial += 1;
                    if dial >= DIAL_SIZE {
                        dial = 0;
                    }
                }
                _ => panic!("Invalid direction"),
            }

            if dial == 0 {
                activation_count += 1;
            }
        }
    }

    println!("Part 2: {}", activation_count);
}

pub fn solution(input: &str) {
    part1(input);
    part2(input);
}
