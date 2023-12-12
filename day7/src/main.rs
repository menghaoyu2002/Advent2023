use std::{
    cmp::Ordering,
    collections::HashMap,
    fs::File,
    io::{self, BufRead, BufReader},
};

#[derive(Debug)]
enum PossibleHands {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

impl Clone for PossibleHands {
    fn clone(&self) -> Self {
        match self {
            FiveOfAKind => FiveOfAKind,
            FourOfAKind => FourOfAKind,
            FullHouse => FullHouse,
            ThreeOfAKind => ThreeOfAKind,
            TwoPair => TwoPair,
            OnePair => OnePair,
            HighCard => HighCard,
        }
    }
}

impl Copy for PossibleHands {}

fn to_card_value(char: &char) -> u32 {
    match char {
        'A' => 14,
        'K' => 13,
        'Q' => 12,
        // 'J' => 11,
        'J' => 0, // part 2
        'T' => 10,
        '9' => 9,
        '8' => 8,
        '7' => 7,
        '6' => 6,
        '5' => 5,
        '4' => 4,
        '3' => 3,
        '2' => 2,
        '1' => 1,
        _ => unreachable!(),
    }
}

use PossibleHands::*;

fn get_card_map(hand: &str) -> HashMap<char, u32> {
    let mut map = HashMap::new();

    for card in hand.chars() {
        match map.get(&card) {
            Some(count) => map.insert(card, count + 1),
            None => map.insert(card, 1),
        };
    }

    map
}

fn get_hand_type(map: &HashMap<char, u32>, num_of_jokers: u32) -> PossibleHands {
    if num_of_jokers == 5 {
        return FiveOfAKind;
    }

    let mut best_hand = HighCard;
    let mut num_of_pairs = 0;
    let mut num_of_triples = 0;

    for (card, count) in map {
        if *card == 'J' {
            continue;
        }

        let found_hand;
        match count {
            5 => found_hand = FiveOfAKind,
            4 => found_hand = FourOfAKind,
            3 => {
                num_of_triples += 1;

                found_hand = if num_of_pairs == 1 {
                    FullHouse
                } else {
                    ThreeOfAKind
                };
            }
            2 => {
                num_of_pairs += 1;

                found_hand = if num_of_triples == 1 {
                    FullHouse
                } else if num_of_pairs == 2 {
                    TwoPair
                } else {
                    OnePair
                };
            }
            1 => found_hand = HighCard,
            _ => unreachable!(),
        };

        if (found_hand as u8) > (best_hand as u8) {
            best_hand = found_hand;
        }
    }

    for i in 1..=num_of_jokers {
        for (char, count) in map {
            if *char == 'J' {
                continue;
            }
            let mut new_map = map.clone();
            new_map.insert(*char, count + i);

            let joker_hands = get_hand_type(&new_map, num_of_jokers - i);

            if (joker_hands as u8) > (best_hand as u8) {
                best_hand = joker_hands;
            }
        }
    }

    best_hand
}

fn main() -> io::Result<()> {
    let file = File::open("./input")?;
    let reader = BufReader::new(file);

    let mut buckets = vec![vec![]; 7];

    for line in reader.lines() {
        let [hand, wager] = line.as_ref().unwrap().split(" ").collect::<Vec<&str>>()[..] else {
            unreachable!()
        };

        let wager = wager.parse::<u32>().unwrap();

        let map = get_card_map(&hand);
        let num_of_jokers = match map.get(&'J') {
            Some(num) => *num,
            None => 0,
        };

        let bucket = &mut buckets[get_hand_type(&map, num_of_jokers) as usize];

        bucket.push((hand.to_string(), wager));
    }

    for bucket in &mut buckets {
        bucket.sort_by(|(a, _), (b, _)| {
            for i in 0..5 {
                if to_card_value(&a.chars().collect::<Vec<char>>()[i])
                    > to_card_value(&b.chars().collect::<Vec<char>>()[i])
                {
                    return Ordering::Greater;
                }

                if to_card_value(&a.chars().collect::<Vec<char>>()[i])
                    < to_card_value(&b.chars().collect::<Vec<char>>()[i])
                {
                    return Ordering::Less;
                }
            }

            Ordering::Equal
        });
    }

    // println!("{:?}", buckets);

    // println!(
    //     "expecting two pair, got {:?}",
    //     get_hand_type(&get_card_map("QJJQ2"), 0)
    // );

    let mut rank = 1;
    let mut sum = 0;
    for bucket in buckets {
        for (_, bid) in bucket {
            sum += bid * rank;
            rank += 1;
        }
    }

    println!("{}", sum);

    Ok(())
}
