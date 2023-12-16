use std::{collections::HashSet, fs::read_to_string, hash::BuildHasher, usize};

const TEST: &str = r#".|...\....
|.-.\.....
.....|-...
........|.
..........
.........\
..../.\\..
.-.-/..|..
.|....-|.\
..//.|...."#;

pub fn part_1() -> usize {
    let contents = read_to_string("input/day-16.txt").unwrap();
    let grid: Vec<Vec<char>> = contents
        .lines()
        .map(|line| line.chars().collect())
        .collect();
    traverse_grid(&grid, (0, 0), Direction::East)
}

fn traverse_grid(grid: &[Vec<char>], (row, col): (usize, usize), direction: Direction) -> usize {
    let mut traveled: HashSet<(usize, usize)> = HashSet::new();
    let mut cache: HashSet<(isize, isize, Direction)> = HashSet::new();
    let mut beams: Vec<(isize, isize, Direction)> = vec![(row as isize, col as isize, direction)];
    while !beams.is_empty() {
        let (ref mut row, ref mut col, ref mut direction) = beams.first_mut().unwrap();
        let mut waiting: Option<(isize, isize, Direction)> = None;
        if *row < usize::MIN as isize
            || *col < usize::MIN as isize
            || *row >= grid.len() as isize
            || *col >= grid[*row as usize].len() as isize
        {
            beams.swap_remove(0);
            continue;
        }
        traveled.insert((*row as usize, *col as usize));
        match grid[*row as usize][*col as usize] {
            '\\' => match direction {
                Direction::North => *direction = Direction::West,
                Direction::South => *direction = Direction::East,
                Direction::East => *direction = Direction::South,
                Direction::West => *direction = Direction::North,
            },
            '/' => match direction {
                Direction::South => *direction = Direction::West,
                Direction::North => *direction = Direction::East,
                Direction::West => *direction = Direction::South,
                Direction::East => *direction = Direction::North,
            },
            '-' => match direction {
                Direction::North | Direction::South => {
                    waiting = Some((*row, *col - 1, Direction::West));
                    *direction = Direction::East;
                }
                _ => (),
            },
            '|' => match direction {
                Direction::East | Direction::West => {
                    waiting = Some((*row - 1, *col, Direction::North));
                    *direction = Direction::South;
                }
                _ => (),
            },
            '.' => (),
            _ => unreachable!(),
        }
        match direction {
            Direction::North => *row -= 1,
            Direction::South => *row += 1,
            Direction::East => *col += 1,
            Direction::West => *col -= 1,
        }
        if cache.contains(&(*row, *col, direction.clone())) {
            beams.swap_remove(0);
            continue;
        }
        cache.insert((*row, *col, direction.clone()));
        if let Some(beam) = waiting {
            beams.push(beam);
        }
    }
    traveled.len()
}

#[derive(PartialEq, Hash, Eq, Clone)]
enum Direction {
    North,
    South,
    East,
    West,
}

pub fn part_2() -> usize {
    let contents = read_to_string("input/day-16.txt").unwrap();
    let grid: Vec<Vec<char>> = contents
        .lines()
        .map(|line| line.chars().collect())
        .collect();
    let mut max = 0;
    for i in 0..grid.len() {
        for j in [0, grid[i].len() - 1] {
            let direction = if j == 0 {
                Direction::East
            } else {
                Direction::West
            };
            max = match i {
                0 => max
                    .max(traverse_grid(&grid, (i, j), direction))
                    .max(traverse_grid(&grid, (i, j), Direction::South)),
                _ if i == grid.len() - 1 => max
                    .max(traverse_grid(&grid, (i, j), direction))
                    .max(traverse_grid(&grid, (i, j), Direction::North)),
                _ => max.max(traverse_grid(&grid, (i, j), direction)),
            }
        }
    }
    for i in [0, grid.len() - 1] {
        for j in 1..grid[i].len() - 1 {
            let direction = if i == 0 {
                Direction::South
            } else {
                Direction::North
            };
            max = max.max(traverse_grid(&grid, (i, j), direction))
        }
    }
    max
}
