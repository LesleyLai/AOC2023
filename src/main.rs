const INPUT: &'static str = include_str!("./input.txt");

const NUMBERS: [&[u8]; 9] = [
    b"one", b"two", b"three", b"four", b"five", b"six", b"seven", b"eight", b"nine",
];

fn lex(s: &str) -> Vec<u32> {
    let mut res: Vec<u32> = vec![];

    let bytes = s.as_bytes();

    let mut pos: usize = 0;
    while pos < bytes.len() {
        let remaining = &bytes[pos..];

        if bytes[pos].is_ascii_digit() {
            res.push((bytes[pos] - b'0') as u32);
        }

        for (i, number) in NUMBERS.iter().enumerate() {
            if remaining.starts_with(number) {
                res.push(i as u32 + 1);
                break;
            }
        }
        pos += 1;
    }

    res
}

fn calibrate(numbers: &[u32]) -> u32 {
    let first = numbers.first().unwrap();
    let last = numbers.last().unwrap();
    first * 10 + last
}

fn main() {
    let lines = INPUT.lines().filter(|line| !line.is_empty());

    let result1: u32 = lines
        .clone()
        .map(|line| {
            let numbers: Vec<_> = line.chars().filter_map(|c: char| c.to_digit(10)).collect();
            calibrate(&numbers)
        })
        .sum();
    println!("Part 1: {}", result1);

    let result2: u32 = lines
        .map(|line| {
            let numbers: Vec<_> = lex(line);
            calibrate(numbers.as_slice())
        })
        .sum();
    println!("Part 2: {}", result2);
}
