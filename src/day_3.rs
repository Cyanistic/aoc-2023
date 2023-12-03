use std::{collections::HashMap, fs::read_to_string};

pub fn part_1() -> usize {
    let contents = read_to_string("input/day-3.txt").unwrap();
    let symbols: Vec<(usize, usize)> = contents
        .lines()
        .enumerate()
        .flat_map(|(k, v)| {
            v.chars()
                .enumerate()
                .filter_map(|(x, y)| match y.is_ascii_digit() || y == '.' {
                    true => None,
                    false => Some((k, x)),
                })
                .collect::<Vec<(usize, usize)>>()
        })
        .collect();
    let mut cur = String::new();
    let mut sum = 0;
    // dbg!(&symbols);
    for (k, line) in contents.lines().enumerate() {
        for (x, char) in line.chars().enumerate() {
            match char {
                '0'..='9' => cur.push(char),
                _ => {
                    if !cur.is_empty() {
                        // dbg!(&cur);
                        'outer: for i in k.saturating_sub(1)..=k + 1 {
                            // let mut new_symbols = symbols.iter().filter(|y| y.0 == i).peekable();
                            let new_symbols: Vec<(usize, usize)> =
                                symbols.clone().into_iter().filter(|y| y.0 == i).collect();
                            if !new_symbols.is_empty() {
                                for j in (x - cur.len()).saturating_sub(1)..=x {
                                    if new_symbols.iter().any(|y| y.1 == j) {
                                        sum += cur.parse::<usize>().unwrap();
                                        break 'outer;
                                    }
                                }
                            }
                        }
                        cur.clear();
                    }
                }
            }
        }
        if !cur.is_empty() {
            // dbg!(&cur);
            'outer: for i in k.saturating_sub(1)..=k + 1 {
                let new_symbols: Vec<(usize, usize)> =
                    symbols.clone().into_iter().filter(|y| y.0 == i).collect();
                if !new_symbols.is_empty() {
                    for j in (line.len() - cur.len()).saturating_sub(1)..=line.len() {
                        if new_symbols.iter().any(|y| y.1 == j) {
                            sum += cur.parse::<usize>().unwrap();
                            break 'outer;
                        }
                    }
                }
            }
            cur.clear();
        }
    }
    sum
}

pub fn part_2() -> usize {
    let contents = read_to_string("input/day-3.txt").unwrap();
    let mut gears: HashMap<(usize, usize), (Option<usize>, Option<usize>)> = HashMap::new();
    let symbols: Vec<(usize, usize)> = contents
        .lines()
        .enumerate()
        .flat_map(|(k, v)| {
            v.chars()
                .enumerate()
                .filter_map(|(x, y)| match y == '*' {
                    true => Some((k, x)),
                    false => None,
                })
                .collect::<Vec<(usize, usize)>>()
        })
        .collect();
    let mut cur = String::new();
    for (k, line) in contents.lines().enumerate() {
        for (x, char) in line.chars().enumerate() {
            match char {
                '0'..='9' => cur.push(char),
                _ => {
                    if !cur.is_empty() {
                        let parsed = cur.parse::<usize>().unwrap();
                        for i in k.saturating_sub(1)..=k + 1 {
                            let new_symbols: Vec<(usize, usize)> =
                                symbols.clone().into_iter().filter(|y| y.0 == i).collect();
                            if !new_symbols.is_empty() {
                                for j in (x - cur.len()).saturating_sub(1)..=x {
                                    for symbol in new_symbols.iter().filter(|y| y.1 == j) {
                                        gears
                                            .entry(*symbol)
                                            .and_modify(|entry| {
                                                *entry = match entry {
                                                    (Some(_), Some(_)) => (None, None),
                                                    (Some(en), _) => (Some(*en), Some(parsed)),
                                                    ref a => **a,
                                                }
                                            })
                                            .or_insert((Some(parsed), None));
                                    }
                                }
                            }
                        }
                        cur.clear();
                    }
                }
            }
        }
        if !cur.is_empty() {
            // dbg!(&cur);
            let parsed = cur.parse::<usize>().unwrap();
            for i in k.saturating_sub(1)..=k + 1 {
                // let mut new_symbols = symbols.iter().filter(|y| y.0 == i).peekable();
                let new_symbols: Vec<(usize, usize)> =
                    symbols.clone().into_iter().filter(|y| y.0 == i).collect();
                if !new_symbols.is_empty() {
                    for j in (line.len() - cur.len()).saturating_sub(1)..=line.len() {
                        for symbol in new_symbols.iter().filter(|y| y.1 == j) {
                            gears
                                .entry(*symbol)
                                .and_modify(|entry| {
                                    *entry = match entry {
                                        (Some(_), Some(_)) => (None, None),
                                        (Some(en), _) => (Some(*en), Some(parsed)),
                                        ref a => **a,
                                    }
                                })
                                .or_insert((Some(parsed), None));
                        }
                    }
                }
            }
            cur.clear();
        }
    }
    gears
        .into_values()
        .filter_map(|(x, y)| match (x, y) {
            (Some(x), Some(y)) => Some(x * y),
            _ => None,
        })
        .sum()
}
