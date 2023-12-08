use std::cmp::Ordering;

const TEST_INPUT: &str = "32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483";

const INPUT: &str = include_str!("input.txt");

fn label_to_strength<const IS_PART2: bool>(label: u8) -> isize {
    let labels = if IS_PART2 {
        [
            b'J', b'2', b'3', b'4', b'5', b'6', b'7', b'8', b'9', b'T', b'Q', b'K', b'A',
        ]
    } else {
        [
            b'2', b'3', b'4', b'5', b'6', b'7', b'8', b'9', b'T', b'J', b'Q', b'K', b'A',
        ]
    };
    labels.iter().position(|x| x == &label).unwrap() as isize
}

#[derive(Ord, PartialOrd, Eq, PartialEq)]
struct HandKind(isize);

impl HandKind {
    const FIVE_OF_A_KIND: HandKind = HandKind(6);
    const FOUR_OF_A_KIND: HandKind = HandKind(5);
    const FULL_HOUSE: HandKind = HandKind(4);
    const THREE_OF_A_KIND: HandKind = HandKind(3);
    const TWO_PAIRS: HandKind = HandKind(2);
    const ONE_PAIR: HandKind = HandKind(1);
    const HIGH_CARD: HandKind = HandKind(0);
}

impl std::fmt::Display for HandKind {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self.0 {
            6 => write!(f, "Five of a kind"),
            5 => write!(f, "Four of a kind"),
            4 => write!(f, "Full House"),
            3 => write!(f, "Three of a Kind"),
            2 => write!(f, "Two Pairs"),
            1 => write!(f, "One Pair"),
            _ => write!(f, "High Card"),
        }
    }
}
impl std::fmt::Debug for HandKind {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self)
    }
}

fn hand_kind<const IS_PART2: bool>(hand: &[u8; 5]) -> HandKind {
    let mut label_counts: [isize; 13] = [0; 13];
    for c in hand {
        let single_label_strength = label_to_strength::<IS_PART2>(*c);
        label_counts[single_label_strength as usize] += 1;
    }

    let mut count_of_counts: [usize; 6] = Default::default();
    for (label, count) in label_counts.iter().enumerate() {
        /* exclude j (label = 0 here) for part 2 */
        if *count > 0 && (!IS_PART2 || label != 0) {
            count_of_counts[*count as usize] += 1;
        }
    }

    let no_joker_hand_kind = || {
        if count_of_counts[5] == 1 {
            HandKind::FIVE_OF_A_KIND
        } else if count_of_counts[4] == 1 {
            HandKind::FOUR_OF_A_KIND
        } else if count_of_counts[3] == 1 {
            if count_of_counts[2] == 1 {
                HandKind::FULL_HOUSE
            } else {
                HandKind::THREE_OF_A_KIND
            }
        } else if count_of_counts[2] == 2 {
            HandKind::TWO_PAIRS
        } else if count_of_counts[2] == 1 {
            HandKind::ONE_PAIR
        } else {
            HandKind::HIGH_CARD
        }
    };

    if !IS_PART2 {
        no_joker_hand_kind()
    } else {
        let j_count = label_counts[0];

        if j_count == 0 {
            no_joker_hand_kind()
        } else if j_count == 1 {
            if count_of_counts[4] == 1 {
                HandKind::FIVE_OF_A_KIND
            } else if count_of_counts[3] == 1 {
                HandKind::FOUR_OF_A_KIND
            } else if count_of_counts[2] == 2 {
                HandKind::FULL_HOUSE
            } else if count_of_counts[2] == 1 {
                HandKind::THREE_OF_A_KIND
            } else {
                HandKind::ONE_PAIR
            }
        } else if j_count == 2 {
            if count_of_counts[3] == 1 {
                HandKind::FIVE_OF_A_KIND
            } else if count_of_counts[2] == 1 {
                HandKind::FOUR_OF_A_KIND
            } else {
                HandKind::THREE_OF_A_KIND
            }
        } else if j_count == 3 {
            if count_of_counts[2] == 1 {
                HandKind::FIVE_OF_A_KIND
            } else {
                HandKind::FOUR_OF_A_KIND
            }
        } else {
            HandKind::FIVE_OF_A_KIND
        }
    }
}

fn compare_hand_strength<const IS_PART2: bool>(hand1: &[u8; 5], hand2: &[u8; 5]) -> Ordering {
    match hand_kind::<IS_PART2>(hand1).cmp(&hand_kind::<IS_PART2>(hand2)) {
        Ordering::Equal => {
            for (&c1, &c2) in hand1.iter().zip(hand2) {
                match label_to_strength::<IS_PART2>(c1).cmp(&label_to_strength::<IS_PART2>(c2)) {
                    Ordering::Equal => {}
                    greater_or_less => return greater_or_less,
                }
            }
            Ordering::Equal
        }
        greater_or_less => greater_or_less,
    }
}

fn common<const IS_PART2: bool>(input: &str) -> isize {
    let mut hands: Vec<([u8; 5], isize)> = input
        .lines()
        .filter_map(|line| line.split_once(" "))
        .map(|(hand, bid)| {
            (
                hand.as_bytes().try_into().unwrap(),
                bid.parse::<isize>().unwrap(),
            )
        })
        .collect();

    hands.sort_by(|(hand1, _), (hand2, _)| compare_hand_strength::<IS_PART2>(hand1, hand2));

    let mut winning = 0;
    for (i, (_, bid)) in hands.iter().enumerate() {
        winning += (i as isize + 1) * bid;
    }
    winning
}

fn part1(input: &str) -> isize {
    common::<false>(input)
}

fn part2(input: &str) -> isize {
    common::<true>(input)
}

fn main() {
    assert_eq!(part1(TEST_INPUT), 6440);
    assert_eq!(part1(INPUT), 246912307);

    assert_eq!(part2(TEST_INPUT), 5905);
    assert_eq!(part2(INPUT), 246894760);
}
