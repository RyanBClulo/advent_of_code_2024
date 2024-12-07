use std::fmt::{self};
use std::fs::File;
use std::io::{self, BufRead};

fn main() -> io::Result<()> {
    #[derive(Clone, Debug)]
    struct State {
        grid: Vec<Vec<char>>,
        position: (i32, i32),
        direction: (i32, i32),
        running: bool,
    }

    impl fmt::Display for State {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            writeln!(f, "{:?}", self.position)?;
            writeln!(f, "{:?}", self.direction)?;
            for i in 0..self.grid.len() {
                for j in 0..self.grid.len() {
                    write!(f, "{}", self.grid[i][j])?;
                }
                writeln!(f, "")?;
            }
            Ok(())
        }
    }

    let path = "./input.txt";
    let file = File::open(&path)?;
    let reader = io::BufReader::new(file);

    let grid: Vec<Vec<char>> = reader
        .lines()
        .filter_map(|line| line.ok())
        .map(|line| line.chars().collect())
        .collect();

    let mut state = State {
        grid: grid.clone(),
        position: get_position(&grid),
        direction: (-1, 0),
        running: true,
    };

    fn get_position(grid: &Vec<Vec<char>>) -> (i32, i32) {
        for i in 0..grid.len() {
            for j in 0..grid[0].len() {
                if grid[i][j] == '^' {
                    return (i as i32, j as i32);
                }
            }
        }
        (0, 0)
    }

    fn next_direction(direction: (i32, i32)) -> (i32, i32) {
        if direction == (-1, 0) {
            return (0, 1);
        } 
        else if direction == (0, 1) {
            return (1, 0);
        } 
        else if direction == (1, 0) {
            return (0, -1);
        }
        (-1, 0)
    }

    fn time_step(state: &mut State)  {
        state.grid[state.position.0 as usize][state.position.1 as usize] = 'o';
        state.position = (
            state.position.0 + state.direction.0,
            state.position.1 + state.direction.1,
        );
        if state.position.0 < 0
            || state.position.1 < 0
            || state.position.0 >= state.grid.len() as i32
            || state.position.1 >= state.grid[0].len() as i32
        {
            state.running = false;
            return;
        }
        if state.grid[state.position.0 as usize][state.position.1 as usize] == '#' {
            state.position = (
                state.position.0 - state.direction.0,
                state.position.1 - state.direction.1,
            );
            state.direction = next_direction(state.direction);
        } else {
            state.grid[state.position.0 as usize][state.position.1 as usize] = '^';
        }
    }

    println!("{}", state);
    while state.running {
        time_step(&mut state);
        // println!("{}", state);
    }

    println!("{}", state);

    let mut sum = 0;
    for row in state.grid {
        for col in row.iter() {
            if *col == 'o' {
                sum +=1;
            }
        }
    }

    println!("Sum = {}", sum);

    Ok(())
}
