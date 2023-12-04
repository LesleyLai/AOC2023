const TEST_INPUT: &str = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11
";

const INPUT: &str = include_str!("./input.txt");

type Card = (Vec<i32>, Vec<i32>);

fn parse_input(input: &str) -> Vec<Card> {
    let parse_numbers = |str: &str| -> Vec<i32> {
        str.split(" ")
            .filter_map(|word| word.parse().ok())
            .collect()
    };

    input
        .lines()
        .map(|line| {
            let (winning_numbers, numbers) =
                line.split_once(":").unwrap().1.split_once("|").unwrap();
            (parse_numbers(winning_numbers), parse_numbers(numbers))
        })
        .collect()
}

fn count_matching_numbers((winning_numbers, numbers): &Card) -> usize {
    numbers
        .iter()
        .filter(|number| winning_numbers.iter().find(|n| n == number).is_some())
        .count()
}

fn part1(cards: &[Card]) -> i32 {
    cards
        .iter()
        .map(count_matching_numbers)
        .filter_map(|count| (count > 0).then(|| 1 << (count - 1)))
        .sum()
}

fn part2(cards: &[Card]) -> i32 {
    let mut instances = vec![1; cards.len()];
    for (card_num, card) in cards.iter().enumerate() {
        let matching_number_count = count_matching_numbers(card);
        let current_instance_count = instances[card_num];
        for instance in instances
            .iter_mut()
            .skip(card_num + 1)
            .take(matching_number_count)
        {
            *instance += current_instance_count;
        }
    }

    instances.iter().sum()
}

fn main() {
    let test_cards = parse_input(TEST_INPUT);
    let cards = parse_input(INPUT);

    assert_eq!(part1(&test_cards), 13);
    assert_eq!(part1(&cards), 32001);

    assert_eq!(part2(&test_cards), 30);
    assert_eq!(part2(&cards), 5037841);
}
