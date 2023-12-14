use std::ops::Range;

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

#[derive(Copy, Clone, Debug)]
struct Interval {
    begin: isize,
    end: isize,
}

impl Interval {
    fn from_begin_and_length(start: isize, length: isize) -> Self {
        Interval {
            begin: start,
            end: start + length,
        }
    }

    fn contains(self: &Self, elem: isize) -> bool {
        self.begin <= elem && elem < self.end
    }

    fn contains_interval(self: &Self, other: &Interval) -> bool {
        self.begin <= other.begin && self.end >= other.end
    }

    fn len(self: &Self) -> isize {
        self.end - self.begin
    }

    fn last(self: &Self) -> isize {
        self.end - 1
    }

    // fn intersection(left: &Self, right: &Self) -> Self {}
}

impl std::fmt::Display for Interval {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[{}, {})", self.begin, self.end)
    }
}

fn print_mapping([dest_begin, src_begin, length]: &[isize; 3]) -> String {
    format!(
        "{} -> {}",
        Interval::from_begin_and_length(*src_begin, *length),
        Interval::from_begin_and_length(*dest_begin, *length),
    )
}

fn part2(input: &str) -> isize {
    let mut iter = input
        .split("\r\n\r\n")
        .flat_map(|s| s.split("\n\n")) // Line ending madness
        .map(|s| s.split_once(":").unwrap().1.trim());
    let seed_numbers: Vec<isize> = parse_numbers(iter.next().unwrap()).collect();
    let mut seeds: Vec<_> = seed_numbers
        .chunks(2)
        .map(|chunk| Interval::from_begin_and_length(chunk[0], chunk[1]))
        .collect();
    seeds.sort_by_key(|seed| seed.begin);

    for seed in &seeds {
        print!("{seed} ");
    }
    println!();

    let mut maps: Vec<_> = iter
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
    for map in &mut maps {
        map.sort_by_key(|[_, src_map_begin, _]| *src_map_begin);
    }

    let mut srcs = seeds;
    //let mut results = vec![];
    let map = &maps[0];
    for mapping in map {
        print!("{}, ", print_mapping(mapping));
    }
    println!();

    for src in srcs {
        match map.binary_search_by_key(&src.begin, |[_, src_map_begin, _]| *src_map_begin) {
            Ok(index) => println!("ok: {}", index),
            Err(index) => {
                if index == 0 { // Before the first
                }
                map[index - 1]
            }
        }
    }

    0
}

fn main() {
    assert_eq!(part1(TEST_INPUT), 35);
    assert_eq!(part1(INPUT), 178159714);

    part2(TEST_INPUT);
    //assert_eq!(part2(TEST_INPUT), 46);
    // assert_eq!(part2(INPUT), 100165128);
}
