fn part1(num: &str) -> bool {
    // All odd lengths are valid
    if !num.len().is_multiple_of(2) {
        return false;
    }

    let (left, right) = num.split_at(num.len() / 2);
    left == right
}

fn part2(num: &str) -> bool {
    let len = num.len();
    if len < 2 {
        // Cannot repeat at least twice
        return false;
    }

    // Check all possible base pattern lengths from 1 to len/2
    for i in 1..=(len / 2) {
        if len.is_multiple_of(i) {
            let pattern = &num[0..i];

            let repeat_count = len / i;
            let check_string = pattern.repeat(repeat_count);

            if check_string == num {
                return true;
            }
        }
    }

    false
}

pub fn solution(input: &str) {
    let ranges = input.split(',').map(|r| {
        let (start, end) = r.split_once('-').unwrap();
        (start.parse::<u64>().unwrap(), end.parse::<u64>().unwrap())
    });

    let mut part1_total = 0;
    let mut part2_total = 0;

    for (start, end) in ranges {
        for num in start..=end {
            let text = num.to_string();

            if part1(&text) {
                part1_total += num;
            }
            if part2(&text) {
                part2_total += num;
            }
        }
    }

    println!("Part 1: {}", part1_total);
    println!("Part 2: {}", part2_total);
}
