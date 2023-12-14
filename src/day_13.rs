use std::fs::read_to_string;

const TEST: &str = "#.##..##.
..#.##.#.
##......#
##......#
..#.##.#.
..##..##.
#.#.##.#.

#...##..#
#....#..#
..##..###
#####.##.
#####.##.
..##..###
#....#..#";

pub fn part_1() -> usize {
    let contents = read_to_string("input/day-13.txt").unwrap();
    let groups = contents.trim().split("\n\n").collect::<Box<[&str]>>();
    groups
        .iter()
        .map(|group| {
            let row_grid: Vec<Vec<char>> =
                group.lines().map(|line| line.chars().collect()).collect();
            let mut col_grid: Vec<Vec<char>> = vec![vec!['0'; row_grid.len()]; row_grid[0].len()];
            for i in 0..row_grid[0].len() {
                for j in 0..row_grid.len() {
                    col_grid[i][j] = row_grid[j][i];
                }
            }
            recurse(&row_grid, Vec::new(), 0) * 100 + recurse(&col_grid, Vec::new(), 0)
        })
        .sum::<usize>()
}

fn recurse(grid: &[Vec<char>], mut row: Vec<String>, ind: usize) -> usize {
    let mut found: Option<usize> = None;
    let mut unpopped: Vec<String> = Vec::new();
    for i in ind..grid.len() {
        let mut cur = String::new();
        for j in 0..grid[i].len() {
            cur.push(grid[i][j]);
        }
        if row.last().is_some_and(|x| *x == cur) {
            if found.is_none() {
                found = Some(row.len());
                unpopped.extend_from_slice(&row);
                unpopped.push(cur);
            }
            row.pop();
            if row.is_empty() {
                return found.map_or(0, |k| k.max(recurse(grid, unpopped, k + 1)));
            }
        } else if let Some(k) = found {
            return recurse(grid, unpopped, k + 1);
        } else {
            row.push(cur);
        }
    }
    found.map_or(0, |k| k.max(recurse(grid, unpopped, k + 1)))
}

fn recurse2(grid: &[Vec<char>], mut row: Vec<String>, ind: usize) -> usize {
    let mut found: Option<usize> = None;
    let mut unpopped: Vec<String> = Vec::new();
    let mut smudge_found = false;
    for i in ind..grid.len() {
        let mut cur = String::new();
        for j in 0..grid[i].len() {
            cur.push(grid[i][j]);
        }
        if row.last().is_some_and(|x| {
            *x == cur
                || (!smudge_found
                    && x.chars().zip(cur.chars()).filter(|(y, z)| *y != *z).count() == 1)
        }) {
            if row
                .last()
                .unwrap()
                .chars()
                .zip(cur.chars())
                .filter(|(y, z)| *y != *z)
                .count()
                == 1
            {
                smudge_found = true;
            }
            if found.is_none() {
                found = Some(row.len());
                unpopped.extend_from_slice(&row);
                unpopped.push(cur);
            }
            row.pop();
            if row.is_empty() {
                break;
            }
        } else if let Some(k) = found {
            return recurse2(grid, unpopped, k + 1);
        } else {
            row.push(cur);
        }
    }
    found.map_or(0, |k| {
        if smudge_found { k } else { 0 }.max(recurse2(grid, unpopped, k + 1))
    })
}

pub fn part_2() -> usize {
    let contents = read_to_string("input/day-13.txt").unwrap();
    let groups = contents.trim().split("\n\n").collect::<Box<[&str]>>();
    groups
        .iter()
        .map(|group| {
            let row_grid: Vec<Vec<char>> =
                group.lines().map(|line| line.chars().collect()).collect();
            let mut col_grid: Vec<Vec<char>> = vec![vec!['0'; row_grid.len()]; row_grid[0].len()];
            for i in 0..row_grid[0].len() {
                for j in 0..row_grid.len() {
                    col_grid[i][j] = row_grid[j][i];
                }
            }
            recurse2(&row_grid, Vec::new(), 0) * 100 + recurse2(&col_grid, Vec::new(), 0)
        })
        .sum::<usize>()
}
