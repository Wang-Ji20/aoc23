use std::{cmp::Ordering, collections::HashMap};

fn main() {
    let v = std::fs::read_to_string("./day7.in").unwrap();
    let mut hands = Vec::new();
    let mut bids = Vec::new();
    for line in v.split("\n") {
        let l = line.split(" ").collect::<Vec<&str>>();
        if l.len() < 2 {
            continue;
        }
        hands.push(String::from(l[0]));
        bids.push(l[1].parse::<u64>().unwrap());
    }
    println!("{}", calculate_bids(parse_inputs(hands, bids)));
}

type Card = u32;

fn parse_card(c: char) -> Card {
    match c {
        '2'..='9' => c.to_digit(10).unwrap(),
        'T' => 10,
        'J' => 11,
        'Q' => 12,
        'K' => 13,
        'A' => 14,
        _ => panic!("invalid card number"),
    }
}

type CardSet = [Card; 5];

fn parse_cardset(s: &str) -> CardSet {
    if s.len() != 5 {
        panic!("bad input");
    }
    [
        parse_card(s.chars().nth(0).unwrap()),
        parse_card(s.chars().nth(1).unwrap()),
        parse_card(s.chars().nth(2).unwrap()),
        parse_card(s.chars().nth(3).unwrap()),
        parse_card(s.chars().nth(4).unwrap()),
    ]
}

#[test]
fn test_parse_cardset() {
    let ex = "32T3K";
    println!("{:?}", parse_cardset(ex));
}

type CardType = u32;

fn detect_card_type(cs: &CardSet) -> CardType {
    let mut card_map = HashMap::new();
    for card in cs {
        match card_map.get_mut(&card) {
            Some(card_num) => {
                *card_num += 1;
            }
            None => {
                card_map.insert(card, 1);
            }
        }
    }
    let mut v = card_map.values().cloned().collect::<Vec<i32>>();
    v.sort();
    v.reverse();
    if v[0] == 1 {
        return 1;
    }
    if v[0] == 2 && v[1] != 2 {
        return 2;
    }
    if v[0] == 2 && v[1] == 2 {
        return 3;
    }
    if v[0] == 3 && v[1] == 1 {
        return 4;
    }
    if v[0] == 3 && v[1] == 2 {
        return 5;
    }
    if v[0] == 4 {
        return 6;
    }
    7
}

#[test]
fn test_detect() {
    let strs = ["32T3K", "T55J5", "KK677", "KTJJT", "QQQJA"];

    for s in strs {
        println!("{}", detect_card_type(&parse_cardset(s)));
    }
}

fn compare_card_set(cs1: &CardSet, cs2: &CardSet) -> Ordering {
    if detect_card_type(cs1) > detect_card_type(cs2) {
        return Ordering::Greater;
    } else if detect_card_type(cs2) > detect_card_type(cs1) {
        return Ordering::Less;
    }
    let mut result = Ordering::Equal;
    for (c1, c2) in cs1.iter().zip(cs2) {
        match c1.cmp(&c2) {
            o @ Ordering::Less | o @ Ordering::Greater => {
                result = o;
                break;
            }
            _ => continue,
        }
    }
    result
}

fn calculate_bids(mut sets: Vec<(CardSet, u64)>) -> u64 {
    sets.sort_by(|a, b| compare_card_set(&a.0, &b.0));
    sets.iter()
        .map(|a| a.1)
        .zip(1..)
        .fold(0, |acc, (rank, bid)| acc + rank * bid)
}

#[test]
fn test_calculate_bids() {
    let strs = ["32T3K", "T55J5", "KK677", "KTJJT", "QQQJA"];
    let bids = [765, 684, 28, 220, 483];
    let mut v = Vec::new();
    for (s, i) in strs.into_iter().zip(bids) {
        v.push((parse_cardset(s), i))
    }
    println!("{}", calculate_bids(v));
}

fn parse_inputs(strs: Vec<String>, bids: Vec<u64>) -> Vec<(CardSet, u64)> {
    let mut v = Vec::new();
    for (s, i) in strs.into_iter().zip(bids) {
        v.push((parse_cardset(&s), i))
    }
    v
}
