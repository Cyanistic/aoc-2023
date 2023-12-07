use std::{cmp::Ordering, collections::HashMap, fs::read_to_string};

const TEST: &str = "32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483";

pub fn part_1() -> usize {
    let contents = read_to_string("input/day-7.txt").unwrap();
    let mut pairs = contents
        .lines()
        .map(|line| {
            let (cards, bid) = line.split_once(' ').unwrap();
            (
                cards.chars().collect::<Box<[char]>>(),
                bid.trim().parse::<usize>().unwrap(),
            )
        })
        .collect::<Vec<(Box<[char]>, usize)>>();
    pairs.sort_unstable_by(|a, b| {
        let mut k_map: HashMap<char, usize> = HashMap::with_capacity(17);
        for char in a.0.iter() {
            k_map.entry(*char).and_modify(|x| *x += 1).or_insert(1);
        }
        let values: Vec<usize> = k_map.into_values().collect();
        use HandType as H;
        let a_rank: HandType = match values {
            _ if values.contains(&5) => H::Five,
            _ if values.contains(&4) => H::Four,
            _ if values.contains(&3) => {
                if values.contains(&2) {
                    H::FullHouse
                } else {
                    H::Three
                }
            }
            _ if values.iter().filter(|&x| *x == 2).count() == 2 => H::Two,
            _ if values.contains(&2) => H::One,
            _ => H::High,
        };

        k_map = HashMap::with_capacity(17);

        for char in b.0.iter() {
            k_map.entry(*char).and_modify(|x| *x += 1).or_insert(1);
        }

        let values: Vec<usize> = k_map.into_values().collect();
        let b_rank: HandType = match values {
            _ if values.contains(&5) => H::Five,
            _ if values.contains(&4) => H::Four,
            _ if values.contains(&3) => {
                if values.contains(&2) {
                    H::FullHouse
                } else {
                    H::Three
                }
            }
            _ if values.iter().filter(|&x| *x == 2).count() == 2 => H::Two,
            _ if values.contains(&2) => H::One,
            _ => H::High,
        };
        match a_rank.partial_cmp(&b_rank).unwrap() {
            Ordering::Equal => {
                for i in 0..a.0.len() {
                    match a.0[i].cmp(&b.0[i]) {
                        Ordering::Equal => (),
                        _ => {
                            let a_val = match a.0[i] {
                                '2' => 0,
                                '3' => 1,
                                '4' => 2,
                                '5' => 3,
                                '6' => 4,
                                '7' => 5,
                                '8' => 6,
                                '9' => 7,
                                'T' => 8,
                                'J' => 9,
                                'Q' => 10,
                                'K' => 11,
                                'A' => 12,
                                _ => unreachable!(),
                            };
                            let b_val = match b.0[i] {
                                '2' => 0,
                                '3' => 1,
                                '4' => 2,
                                '5' => 3,
                                '6' => 4,
                                '7' => 5,
                                '8' => 6,
                                '9' => 7,
                                'T' => 8,
                                'J' => 9,
                                'Q' => 10,
                                'K' => 11,
                                'A' => 12,
                                _ => unreachable!(),
                            };
                            return a_val.cmp(&b_val);
                        }
                    }
                }
                Ordering::Equal
            }

            o => o,
        }
    });

    pairs
        .into_iter()
        .enumerate()
        .map(|(r, (_, b))| (r + 1) * b)
        .sum::<usize>()
}

#[repr(C)]
#[derive(PartialEq, PartialOrd)]
enum HandType {
    High = 1,
    One = 2,
    Two = 3,
    Three = 4,
    FullHouse = 5,
    Four = 6,
    Five = 7,
}

pub fn part_2() -> usize {
    let contents = read_to_string("input/day-7.txt").unwrap();
    let mut pairs = contents
        .lines()
        .map(|line| {
            let (cards, bid) = line.split_once(' ').unwrap();
            (
                cards.chars().collect::<Box<[char]>>(),
                bid.trim().parse::<usize>().unwrap(),
            )
        })
        .collect::<Vec<(Box<[char]>, usize)>>();
    pairs.sort_unstable_by(|a, b| {
        let mut k_map: HashMap<char, usize> = HashMap::with_capacity(17);
        for char in a.0.iter() {
            k_map.entry(*char).and_modify(|x| *x += 1).or_insert(1);
        }

        if let Some(j) = k_map.remove(&'J') {
            match k_map.iter_mut().max_by_key(|x| *x.1) {
                Some(max) => {
                    *max.1 += j;
                }
                None => {
                    k_map.insert('J', j);
                }
            }
        }

        let values: Vec<usize> = k_map.into_values().collect();
        use HandType as H;
        let a_rank: HandType = match values {
            _ if values.contains(&5) => H::Five,
            _ if values.contains(&4) => H::Four,
            _ if values.contains(&3) => {
                if values.contains(&2) {
                    H::FullHouse
                } else {
                    H::Three
                }
            }
            _ if values.iter().filter(|&x| *x == 2).count() == 2 => H::Two,
            _ if values.contains(&2) => H::One,
            _ => H::High,
        };

        k_map = HashMap::with_capacity(17);

        for char in b.0.iter() {
            k_map.entry(*char).and_modify(|x| *x += 1).or_insert(1);
        }

        if let Some(j) = k_map.remove(&'J') {
            match k_map.iter_mut().max_by_key(|x| *x.1) {
                Some(max) => {
                    *max.1 += j;
                }
                None => {
                    k_map.insert('J', j);
                }
            }
        }

        let values: Vec<usize> = k_map.into_values().collect();
        let b_rank: HandType = match values {
            _ if values.contains(&5) => H::Five,
            _ if values.contains(&4) => H::Four,
            _ if values.contains(&3) => {
                if values.contains(&2) {
                    H::FullHouse
                } else {
                    H::Three
                }
            }
            _ if values.iter().filter(|&x| *x == 2).count() == 2 => H::Two,
            _ if values.contains(&2) => H::One,
            _ => H::High,
        };
        match a_rank.partial_cmp(&b_rank).unwrap() {
            Ordering::Equal => {
                for i in 0..a.0.len() {
                    match a.0[i].cmp(&b.0[i]) {
                        Ordering::Equal => (),
                        _ => {
                            let a_val = match a.0[i] {
                                'J' => 0,
                                '2' => 1,
                                '3' => 2,
                                '4' => 3,
                                '5' => 4,
                                '6' => 5,
                                '7' => 6,
                                '8' => 7,
                                '9' => 8,
                                'T' => 9,
                                'Q' => 10,
                                'K' => 11,
                                'A' => 12,
                                _ => unreachable!(),
                            };
                            let b_val = match b.0[i] {
                                'J' => 0,
                                '2' => 1,
                                '3' => 2,
                                '4' => 3,
                                '5' => 4,
                                '6' => 5,
                                '7' => 6,
                                '8' => 7,
                                '9' => 8,
                                'T' => 9,
                                'Q' => 10,
                                'K' => 11,
                                'A' => 12,
                                _ => unreachable!(),
                            };
                            return a_val.cmp(&b_val);
                        }
                    }
                }
                Ordering::Equal
            }

            o => o,
        }
    });

    pairs
        .into_iter()
        .enumerate()
        .map(|(r, (_, b))| (r + 1) * b)
        .sum::<usize>()
}
