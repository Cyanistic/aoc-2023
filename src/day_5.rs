use std::fs::read_to_string;

const TEST: &str = "seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4";

pub fn part_1() -> usize {
    let contents = read_to_string("input/day-5.txt").unwrap();
    let mut seeds: Vec<usize> = Vec::new();
    let lines: Vec<_> = contents.lines().collect();
    let mut transforms: Vec<Vec<usize>> = Vec::new();

    let mut ind = 0;

    while ind < lines.len() {
        let mut line = lines[ind];
        if line.is_empty() || line.chars().next().unwrap().is_alphabetic() {
            if ind == 0 {
                seeds = line
                    .split_once(':')
                    .unwrap()
                    .1
                    .split_ascii_whitespace()
                    .map(|x| x.parse().unwrap())
                    .collect();
            }
            ind += 1;
            continue;
        }
        while lines.get(ind).is_some_and(|x| !x.is_empty()) {
            line = lines[ind];
            transforms.push(
                line.split_ascii_whitespace()
                    .map(|x| x.parse().unwrap())
                    .collect(),
            );
            ind += 1;
        }
        for seed in seeds.iter_mut() {
            for trans in &transforms {
                if *seed >= trans[1] && *seed < trans[1] + trans[2] {
                    *seed = trans[0] + *seed - trans[1];
                    break;
                }
            }
        }
        transforms.clear();
        ind += 1;
    }
    *seeds.iter().min().unwrap()
}

pub fn part_2() -> usize {
    let contents = read_to_string("input/day-5.txt").unwrap();
    let mut seed_ranges: Vec<(usize, usize)> = Vec::new();
    let lines: Vec<_> = contents.lines().collect();
    let mut transforms: Vec<Vec<usize>> = Vec::new();

    let mut ind = 0;

    while ind < lines.len() {
        let mut line = lines[ind];
        if line.is_empty() || line.chars().next().unwrap().is_alphabetic() {
            if ind == 0 {
                let seed_combos: Vec<usize> = line
                    .split_once(':')
                    .unwrap()
                    .1
                    .split_ascii_whitespace()
                    .map(|x| x.parse().unwrap())
                    .collect();
                for combo in (0..seed_combos.len()).step_by(2) {
                    seed_ranges.push((
                        seed_combos[combo],
                        seed_combos[combo] + seed_combos[combo + 1] - 1,
                    ));
                }
            }
            ind += 1;
            continue;
        }
        while lines.get(ind).is_some_and(|x| !x.is_empty()) {
            line = lines[ind];
            transforms.push(
                line.split_ascii_whitespace()
                    .map(|x| x.parse().unwrap())
                    .collect(),
            );
            ind += 1;
        }
        let mut i = 0;
        while i < seed_ranges.len() {
            let (min, max) = seed_ranges[i];
            for trans in &transforms {
                if (max >= trans[1] && max < trans[1] + trans[2])
                    || (min >= trans[1] && min < trans[1] + trans[2])
                {
                    assert!(max >= min);
                    assert!(min < trans[1] + trans[2]);
                    assert!(max >= trans[1]);
                    let new_min = trans[0] + min.max(trans[1]) - trans[1];
                    let new_max = trans[0] + max.min(trans[1] + trans[2] - 1) - trans[1];
                    seed_ranges[i] = (new_min, new_max);
                    if min < trans[1] {
                        seed_ranges.push((min, trans[1] - 1));
                    }
                    if max >= trans[1] + trans[2] {
                        seed_ranges.push((trans[1] + trans[2], max));
                    }
                    break;
                }
            }
            i += 1;
        }
        transforms.clear();
        ind += 1;
    }
    seed_ranges.iter().min_by_key(|(min, _)| min).unwrap().0
}
