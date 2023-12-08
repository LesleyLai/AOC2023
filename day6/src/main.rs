const TEST_INPUT: &str = "Time:      7  15   30
Distance:  9  40  200
";

const INPUT: &str = "Time:        38     67     76     73
Distance:   234   1027   1157   1236";

fn ways_to_win(time: isize, record_distance: isize) -> usize {
    // Even bruth-force seems to be fast enough for this problem to finish instantaneously
    // (0..=time)
    //     .filter(|charge_time| (time - charge_time) * charge_time > record_distance)
    //     .count()

    // Root finding (x is charge_time)
    // (time - x) * x = record_distance;
    // x^2 - time x + record_distance = 0;
    let a = 1.;
    let b = -time as f64;
    let c = record_distance as f64;
    let delta = (b.powi(2) - (4. * a * c)).sqrt();
    let x_1 = (-b - delta) / (2. * a);
    let x_2 = (-b + delta) / (2. * a);

    let lower_bound: usize = x_1.ceil() as usize + (x_1 == x_1.ceil()) as usize;
    let upper_bound: usize = x_2.floor() as usize - (x_2 == x_2.floor()) as usize;

    upper_bound - lower_bound + 1
}

fn part1(input: &str) -> usize {
    let mut lines = input.lines().map(|line| {
        line.split_once(":")
            .unwrap()
            .1
            .split_whitespace()
            .filter_map(|token| token.parse().ok())
            .collect::<Vec<isize>>()
    });
    let times = lines.next().unwrap();
    let distances = lines.next().unwrap();

    times
        .iter()
        .zip(distances)
        .fold(1, |acc, (&time, distance)| {
            acc * ways_to_win(time, distance)
        })
}

fn part2(input: &str) -> usize {
    let mut lines = input
        .lines()
        .filter_map(|line| line.split_once(":")?.1.replace(" ", "").parse().ok());
    let time: isize = lines.next().unwrap();
    let distance: isize = lines.next().unwrap();

    ways_to_win(time, distance)
}

fn main() {
    assert_eq!(part1(TEST_INPUT), 288);
    assert_eq!(part1(INPUT), 303600);
    assert_eq!(part2(TEST_INPUT), 71503);
    assert_eq!(part2(INPUT), 23654842);
}
