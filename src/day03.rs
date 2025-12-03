pub fn solution(input: &str) {
    let part1: u32 = input
        .lines()
        .map(|line| {
            let chars: Vec<u32> = line.chars().map(|c| c.to_digit(10).unwrap()).collect();
            let mut best = 0;

            for i in 0..chars.len() {
                for j in i + 1..chars.len() {
                    best = best.max(chars[i] * 10 + chars[j]);
                }
            }
            best
        })
        .sum();

    println!("Part 1: {}", part1);

    let part2: u64 = input
        .lines()
        .map(|line| {
            let digits: Vec<u32> = line.chars().map(|c| c.to_digit(10).unwrap()).collect();

            let k = 12;
            let mut remove = digits.len() - k;
            let mut stack: Vec<u32> = Vec::with_capacity(digits.len());

            for &d in &digits {
                while remove > 0 && !stack.is_empty() && *stack.last().unwrap() < d {
                    stack.pop();
                    remove -= 1;
                }
                stack.push(d);
            }

            // Truncate to 12 digits
            stack.truncate(k);
            stack.iter().fold(0_u64, |acc, &d| acc * 10 + d as u64)
        })
        .sum();

    println!("Part 2: {}", part2);
}
