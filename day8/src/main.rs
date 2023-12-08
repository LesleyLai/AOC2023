use num::integer::lcm;
use std::collections::HashMap;

const TEST_INPUT1: &str = "RL

AAA = (BBB, CCC)
BBB = (DDD, EEE)
CCC = (ZZZ, GGG)
DDD = (DDD, DDD)
EEE = (EEE, EEE)
GGG = (GGG, GGG)
ZZZ = (ZZZ, ZZZ)
";

const TEST_INPUT2: &str = "LLR

AAA = (BBB, BBB)
BBB = (AAA, ZZZ)
ZZZ = (ZZZ, ZZZ)";

const TEST_INPUT3: &str = "LR

11A = (11B, XXX)
11B = (XXX, 11Z)
11Z = (11B, XXX)
22A = (22B, XXX)
22B = (22C, 22C)
22C = (22Z, 22Z)
22Z = (22B, 22B)
XXX = (XXX, XXX)
";

const INPUT: &str = include_str!("./input.txt");

fn parse(input: &str) -> (&[u8], HashMap<&str, (&str, &str)>) {
    let mut lines = input.lines();
    let instructions = lines.next().unwrap().as_bytes();

    let mut network = HashMap::new();

    for line in lines.skip(1) {
        let (node, remaining) = line.split_once(" = ").unwrap();
        let left_right = remaining
            .trim_start_matches("(")
            .trim_end_matches(")")
            .split_once(", ")
            .unwrap();
        network.insert(node, left_right);
    }
    (instructions, network)
}

fn single_step<'a>(
    current: &'a str,
    step_count: usize,
    instructions: &'a [u8],
    network: &'a HashMap<&'a str, (&'a str, &'a str)>,
) -> &'a str {
    match instructions[step_count % instructions.len()] {
        b'L' => network[current].0,
        _ => network[current].1,
    }
}

fn part1(input: &str) -> usize {
    let (instructions, network) = parse(input);

    let mut steps = 0;
    let mut current = "AAA";
    while current != "ZZZ" {
        current = single_step(current, steps, instructions, &network);
        steps += 1;
    }

    steps
}

fn part2(input: &str) -> usize {
    let (instructions, network) = parse(input);

    let mut starts: Vec<&str> = network
        .keys()
        .filter(|node| node.ends_with("A"))
        .map(|node| *node)
        .collect();
    let mut steps_to_z: Vec<Option<usize>> = vec![None; starts.len()];
    for i in 0..starts.len() {
        let mut current = starts[i];
        let mut step_to_z = 0;
        while !current.ends_with("Z") {
            current = single_step(current, step_to_z, instructions, &network);
            step_to_z += 1;
        }
        steps_to_z[i] = Some(step_to_z);
    }
    let steps_to_z: Vec<_> = steps_to_z.iter().map(|opt| opt.unwrap()).collect();
    steps_to_z
        .iter()
        .fold(1, |acc, step_to_z| lcm(acc, *step_to_z))
}

fn main() {
    assert_eq!(part1(TEST_INPUT1), 2);
    assert_eq!(part1(TEST_INPUT2), 6);
    assert_eq!(part1(INPUT), 16409);

    assert_eq!(part2(TEST_INPUT3), 6);
    assert_eq!(part2(INPUT), 11795205644011);
}
