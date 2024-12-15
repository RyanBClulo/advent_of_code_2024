
extern crate regex;

use regex::Regex;
use std::fmt::{self, Formatter};
use std::fs::File;
use std::io::{self, BufRead};
use std::collections::HashMap;

// static MAX_X: i32 = 11;
// static MAX_Y: i32 = 7;
// static INPUT: &str = "./sample.txt";

static MAX_X: i32 = 101;
static MAX_Y: i32 = 103;
static INPUT: &str = "./input.txt";

static GEN: i32 = 100000;

fn main() -> io::Result<()> {
    let stdin = io::stdin();
    let mut handle = stdin.lock();

    let file = File::open(&INPUT)?;
    let reader = io::BufReader::new(file);

    let regex = Regex::new(r"(?m)p=(?<pos_x>-?\d+),(?<pos_y>-?\d+) v=(?<vel_x>-?\d+),(?<vel_y>-?\d+)").unwrap();

    let mut robots: Vec<Robot> = reader
        .lines()
        .filter_map(|line| line.ok())
        .map(|line| {
            let captures = regex.captures(&line).unwrap();
            Robot {
                pos_x: *&captures["pos_x"].parse::<i32>().unwrap(),
                pos_y: *&captures["pos_y"].parse::<i32>().unwrap(),
                vel_x: *&captures["vel_x"].parse::<i32>().unwrap(),
                vel_y: *&captures["vel_y"].parse::<i32>().unwrap()
            }
        })
        .collect();

    // display_robots(&robots);
    for timestep in 0..GEN {
        for robot in &mut robots {
            robot.move_robot();
        }
        // display_robots(&robots);
        display_state(&robots);
        println!("Timestep: {}", timestep);
        if find_xmas_tree_candiate(&robots) {
            handle.read_line(&mut String::new()).expect("na");
        }

    }
    display_state(&robots);

    Ok(())
}

#[derive(Clone, Debug)]
struct Robot {
    pos_x: i32,
    pos_y: i32,
    vel_x: i32,
    vel_y: i32
}

impl fmt::Display for Robot {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        writeln!(f, "p={},{} v={},{}", self.pos_x, self.pos_y, self.vel_x, self.vel_y)
    }
}

impl Robot {
    fn move_robot(&mut self) {
        self.pos_x = self.pos_x + self.vel_x;
        if self.pos_x < 0 {
            self.pos_x = MAX_X + self.pos_x;
        }
        else if self.pos_x >= MAX_X {
            self.pos_x = 0 + (self.pos_x - MAX_X);
        }

        self.pos_y = self.pos_y + self.vel_y;

        if self.pos_y < 0 {
            self.pos_y = MAX_Y + self.pos_y;
        }
        else if self.pos_y >= MAX_Y {
            self.pos_y = 0 + (self.pos_y - MAX_Y);
        }
    }
}

fn display_robots(robots: &Vec<Robot>) {
    for robot in robots {
        println!("{}", robot);
    }
}

fn display_state(robots: &Vec<Robot>) {
    let mut state: HashMap<(i32, i32), Vec<Robot>> = HashMap::new();
    for robot in robots {
        state.entry((robot.pos_x, robot.pos_y)).or_insert_with(Vec::new)
            .push(robot.clone());
    }

    let mut quad1 = 0;
    let mut quad2 = 0;
    let mut quad3 = 0;
    let mut quad4 = 0;

    // println!("{:?}", state);
    for i in 0..MAX_Y {
        for j in 0..MAX_X {
            if let Some(pos) = state.get(&(j, i)) {
                print!("{}", pos.len());
                if j < MAX_X / 2 && i < MAX_Y / 2 {
                    quad1 += pos.len();
                }
                if j > MAX_X / 2 && i < MAX_Y / 2 {
                    quad2 += pos.len();
                }
                if j < MAX_X / 2 && i > MAX_Y / 2 {
                    quad3 += pos.len();
                }
                if j > MAX_X / 2 && i > MAX_Y / 2 {
                    quad4 += pos.len();
                }
            } else {
                print!(".");
            }
        }
        println!();
    }
    println!("Quad 1: {}", quad1);
    println!("Quad 2: {}", quad2);
    println!("Quad 3: {}", quad3);
    println!("Quad 4: {}", quad4);
    println!("Safety Factor: {}", quad1*quad2*quad3*quad4)

}

fn find_xmas_tree_candiate(robots: &Vec<Robot>) -> bool {
    let mut column_count: HashMap<i32, i32> = HashMap::new();
    let mut row_count: HashMap<i32, i32> = HashMap::new();
    for robot in robots {
        *column_count.entry(robot.pos_x).or_insert(0) += 1;
        *row_count.entry(robot.pos_y).or_insert(0) += 1;
    }
    println!("colums {:?}", column_count);
    println!("rows {:?}", row_count);

    column_count.values().any(|v| *v > 20) && row_count.values().any(|v| *v > 20)
}