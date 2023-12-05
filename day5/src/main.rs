const TEST_INPUT: &str = include_str!("test_input.txt");
const INPUT: &str = include_str!("input.txt");

fn parse_numbers(str: &str) -> impl Iterator<Item = isize> + '_ {
    str.split(" ").filter_map(|word| word.parse().ok())
}

fn part1(input: &str) -> isize {
    let mut iter = input
        .split("\r\n\r\n")
        .flat_map(|s| s.split("\n\n")) // Line ending madness
        .map(|s| s.split_once(":").unwrap().1.trim());
    let seeds: Vec<isize> = parse_numbers(iter.next().unwrap()).collect();

    let maps: Vec<_> = iter
        .map(|s| {
            s.lines()
                .map(parse_numbers)
                .filter_map(|numbers| -> Option<[isize; 3]> {
                    let mut numbers = numbers.take(3);
                    Some([numbers.next()?, numbers.next()?, numbers.next()?])
                })
                .collect::<Vec<_>>()
        })
        .collect();

    let mut location_numbers = vec![];
    for seed in seeds {
        let mut src = seed;
        let mut dest = src;
        for map in &maps {
            dest = src;
            for [dest_start, src_start, length] in map {
                if src >= *src_start && src < src_start + length {
                    dest = dest_start + (src - src_start);
                    break;
                }
            }
            src = dest;
        }
        location_numbers.push(dest);
    }
    *location_numbers.iter().min().unwrap()
}

fn part2(input: &str) -> isize {
    let mut iter = input
        .split("\r\n\r\n")
        .flat_map(|s| s.split("\n\n")) // Line ending madness
        .map(|s| s.split_once(":").unwrap().1.trim());
    let seed_numbers: Vec<isize> = parse_numbers(iter.next().unwrap()).collect();
    let mut seeds = vec![];
    for mut seed_range in seed_numbers.chunks(2) {
        let (begin, length) = (seed_range[0], seed_range[1]);
        for i in begin..begin + length {
            seeds.push(i);
        }
    }

    let maps: Vec<_> = iter
        .map(|s| {
            s.lines()
                .map(parse_numbers)
                .filter_map(|numbers| -> Option<[isize; 3]> {
                    let mut numbers = numbers.take(3);
                    Some([numbers.next()?, numbers.next()?, numbers.next()?])
                })
                .collect::<Vec<_>>()
        })
        .collect();

    let mut location_numbers = vec![];
    for (i, &seed) in seeds.iter().enumerate() {
        let mut src = seed;
        let mut dest = src;
        for map in &maps {
            dest = src;
            for [dest_start, src_start, length] in map {
                if src >= *src_start && src < src_start + length {
                    dest = dest_start + (src - src_start);
                    break;
                }
            }
            src = dest;
        }
        location_numbers.push(dest);
    }
    *location_numbers.iter().min().unwrap()
}

fn main() {
    assert_eq!(part1(TEST_INPUT), 35);
    assert_eq!(part1(INPUT), 178159714);
    assert_eq!(part2(TEST_INPUT), 46);
    // assert_eq!(part2(INPUT), 100165128);
}
