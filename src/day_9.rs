use std::fs::read_to_string;

const TEST: &str = "0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45";

pub fn part_1() -> isize {
    let contents = read_to_string("input/day-9.txt").unwrap();
    contents
        .lines()
        .map(|line| {
            line.trim()
                .split_ascii_whitespace()
                .map(|x| x.trim().parse().unwrap())
                .collect::<Vec<isize>>()
        })
        .map(|mut oasis| {
            let mut sum = 0;
            while oasis.iter().any(|x| *x != 0) {
                sum += oasis.last().unwrap();
                oasis = oasis.windows(2).map(|win| win[1] - win[0]).collect();
            }
            sum
        })
        .sum::<isize>()
}

pub fn part_2() -> isize {
    let contents = read_to_string("input/day-9.txt").unwrap();
    contents
        .lines()
        .map(|line| {
            line.trim()
                .split_ascii_whitespace()
                .map(|x| x.trim().parse().unwrap())
                .collect::<Vec<isize>>()
        })
        .map(|mut oasis| {
            let mut sum = 0;
            let mut first = Vec::new();
            while oasis.iter().any(|x| *x != 0) {
                first.push(*oasis.first().unwrap());
                oasis = oasis.windows(2).map(|win| win[1] - win[0]).collect();
            }
            for i in first.into_iter().rev() {
                sum = i - sum;
            }
            sum
        })
        .sum::<isize>()
}
