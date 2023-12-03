use std::collections::HashMap;

const TEST_INPUT: &str = "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..
";

const INPUT: &str = include_str!("./input.txt");

fn part1(input: &str) -> i32 {
    let lines: Vec<_> = input.lines().map(|line| line.as_bytes()).collect();
    let mut sum: i32 = 0;
    for (y, line) in lines.iter().enumerate() {
        let mut start = 0; // column number
        while start < line.len() {
            if line[start].is_ascii_digit() {
                let mut end = start + 1;
                while end < line.len() && line[end].is_ascii_digit() {
                    end += 1
                }
                let num: i32 = std::str::from_utf8(&line[start..end])
                    .unwrap()
                    .parse()
                    .unwrap();

                let mut is_part_number = false;

                if start > 0 && line[start - 1] != b'.' {
                    is_part_number = true;
                }
                if end != line.len() && line[end] != b'.' {
                    is_part_number = true;
                }

                let before_start = start.saturating_sub(1);
                let after_end = (end + 1).min(line.len());

                if y > 0 {
                    let last_line = lines[y - 1];
                    for x in before_start..after_end {
                        if last_line[x] != b'.' {
                            is_part_number = true;
                        }
                    }
                }
                if y + 1 < line.len() {
                    let next_line = lines[y + 1];
                    for x in before_start..after_end {
                        if next_line[x] != b'.' {
                            is_part_number = true;
                        }
                    }
                }

                if is_part_number {
                    sum += num;
                }

                start = end;
            } else {
                start += 1;
            }
        }
    }
    sum
}

type Coord = (usize, usize);

fn add_gear_number(gears: &mut HashMap<Coord, Vec<i32>>, coord: Coord, num: i32) {
    gears.entry(coord).or_insert_with(Vec::new).push(num);
}

fn part2(input: &str) -> i32 {
    let mut gears = HashMap::new();
    let lines: Vec<_> = input.lines().map(|line| line.as_bytes()).collect();
    for (y, line) in lines.iter().enumerate() {
        let mut start = 0; // column number
        while start < line.len() {
            if line[start].is_ascii_digit() {
                let mut end = start + 1;
                while end < line.len() && line[end].is_ascii_digit() {
                    end += 1
                }
                let num: i32 = std::str::from_utf8(&line[start..end])
                    .unwrap()
                    .parse()
                    .unwrap();

                let before_start = start.saturating_sub(1);

                if line[before_start] == b'*' {
                    add_gear_number(&mut gears, (before_start, y), num);
                }
                if end != line.len() && line[end] == b'*' {
                    add_gear_number(&mut gears, (end, y), num);
                }

                let after_end = (end + 1).min(line.len());
                if y > 0 {
                    let last_line = lines[y - 1];
                    for x in before_start..after_end {
                        if last_line[x] == b'*' {
                            add_gear_number(&mut gears, (x, y - 1), num);
                        }
                    }
                }
                if y + 1 < line.len() {
                    let next_line = lines[y + 1];
                    for x in before_start..after_end {
                        if next_line[x] == b'*' {
                            add_gear_number(&mut gears, (x, y + 1), num);
                        }
                    }
                }
                start = end;
            } else {
                start += 1;
            }
        }
    }

    let mut sum = 0;
    for numbers in gears.values() {
        if numbers.len() == 2 {
            sum += numbers[0] * numbers[1];
        }
    }

    sum
}

fn main() {
    assert_eq!(part1(TEST_INPUT), 4361);
    assert_eq!(part1(INPUT), 527364);
    assert_eq!(part2(TEST_INPUT), 467835);
    assert_eq!(part2(INPUT), 79026871);
}
