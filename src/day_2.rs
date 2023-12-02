use std::fs::read_to_string;

pub fn part_1() -> usize {
    let contents = read_to_string("input/day-2.txt").unwrap();
    contents
        .lines()
        .filter_map(|line| {
            let (game, rest) = line.split_once(':').unwrap();
            let rounds = rest.split(';');
            let mut valid = true;
            let mut blue = 0;
            let mut green = 0;
            let mut red = 0;

            for i in rounds {
                let items = i.split(',');
                for item in items {
                    let (num, color) = item.rsplit_once(' ').unwrap();
                    dbg!(num, color);
                    let num = num.trim().parse::<usize>().unwrap();
                    if valid {
                        match color.trim() {
                            "blue" => valid = num <= 14,
                            "green" => valid = num <= 13,
                            "red" => valid = num <= 12,
                            _ => unreachable!(),
                        }
                    }
                }
            }

            match valid {
                true => Some(
                    game[game.rfind(' ').unwrap()..]
                        .trim()
                        .parse::<usize>()
                        .unwrap(),
                ),
                false => None,
            }
        })
        .sum::<usize>()
}

pub fn part_2() -> usize {
    let contents = read_to_string("input/day-2.txt").unwrap();
    contents
        .lines()
        .map(|line| {
            let (game, rest) = line.split_once(':').unwrap();
            let rounds = rest.split(';');
            let mut blue = 0;
            let mut green = 0;
            let mut red = 0;

            for i in rounds {
                let items = i.split(',');
                for item in items {
                    let (num, color) = item.rsplit_once(' ').unwrap();
                    dbg!(num, color);
                    let num = num.trim().parse::<usize>().unwrap();
                    match color.trim() {
                        "blue" => blue = blue.max(num),
                        "green" => green = green.max(num),
                        "red" => red = red.max(num),
                        _ => unreachable!(),
                    }
                }
            }
            red * blue * green
        })
        .sum::<usize>()
}
