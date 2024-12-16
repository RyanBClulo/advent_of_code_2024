use std::fs::File;
use std::io::{self, BufRead};

static INPUT: &str = "./input.txt";

fn main() {
    let _ = run_input(INPUT);
}

fn run_input(input: &str) -> io::Result<()> {

    let file = File::open(input)?;
    let reader = io::BufReader::new(file);

    let mut state: Vec<Vec<char>> = Vec::new();
    let mut directions: Vec<Direction> = Vec::new();

    for read_line in reader.lines() {
        let line = read_line?;
        if line.contains('#') {
            state.push(line.chars().collect());
        } else {
            for c in line.chars() {
                if let Some(dir) = to_direction(c) {
                    directions.push(dir);
                }
                
            }
        }
    }
    display_state(&state);

    for direction in directions {
        let start = find_start(&state);
        println!("{:?} {:?}", start, direction);
        update_state(&mut state, start, &direction);
        display_state(&state);
    }

    // println!("{:?}", directions);

    println!("GPS Coordinate Sum = {}", sum_coordinates(&state));

    Ok(())
}

fn update_state(state: &mut Vec<Vec<char>>, loc: (i32, i32), dir: &Direction) -> bool {
    let check  = dir.apply(loc);
    if state[check.1][check.0] == '#' {
        return false;
    }
    if state[check.1][check.0] == '.' {
        state[check.1][check.0] = state[loc.1 as usize][loc.0 as usize];
        state[loc.1 as usize][loc.0 as usize] = '.';
        return true;
    }
    if state[check.1][check.0] == 'O' {
        if update_state(state, (check.0 as i32, check.1 as i32), dir) {
            state[check.1][check.0] = state[loc.1 as usize][loc.0 as usize];
            state[loc.1 as usize][loc.0 as usize] = '.';
            return true;
        } else {
            return false;
        }
    }
    false
}

fn find_start(state: &Vec<Vec<char>>) -> (i32, i32) {
    for i in 0..state.len() {
        for j in 0..state[0].len() {
            if state[j][i] == '@' {
                return (i as i32, j as i32);
            }
        }
    }
    (0,0)
}

fn sum_coordinates(state: &Vec<Vec<char>>) -> i32 {
    let mut sum = 0;
    for i in 0..state.len() {
        for j in 0..state[0].len() {
            if state[j][i] == 'O' {
                sum += 100 * j as i32 + i as i32;
            }
        }
    }
    return sum;
}

fn display_state(state: &Vec<Vec<char>>) {
    for row in state {
        for col in row {
            print!("{}", col);
        }
        println!();
    }
    println!();
}

#[derive(Debug)]
enum Direction {
    UP,
    DOWN,
    LEFT,
    RIGHT
}

impl Direction {
    fn apply(&self, coord: (i32, i32)) -> (usize, usize) {
        let (x, y) = match self {
            Direction::UP => (coord.0, coord.1 - 1),
            Direction::DOWN => (coord.0, coord.1 + 1),
            Direction::LEFT => (coord.0 - 1, coord.1),
            Direction::RIGHT => (coord.0 + 1, coord.1),
        };

        let ux: usize = x.try_into().expect("neg X coord not allowed");
        let uy: usize = y.try_into().expect("neg Y coord not allowed");

        (ux, uy)
    }
}

#[test]
fn test_apply_direction() {
    assert_eq!(Direction::UP.apply((1,1)), (1, 0));
    assert_eq!(Direction::DOWN.apply((1,1)), (1, 2));
    assert_eq!(Direction::LEFT.apply((1,1)), (0, 1));
    assert_eq!(Direction::RIGHT.apply((1,1)), (2, 1));
}

fn to_direction(c: char) -> Option<Direction> {
    match c {
        '^' => Some(Direction::UP),
        'v' => Some(Direction::DOWN),
        '<' => Some(Direction::LEFT),
        '>' => Some(Direction::RIGHT),
        _ => None
    }
}