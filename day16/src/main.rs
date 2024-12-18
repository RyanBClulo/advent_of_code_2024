use std::collections::BTreeMap;
use std::fs::File;
use std::io::{self, BufRead};

fn main() -> io::Result<()>{
    let mut maze: BTreeMap<(i32, i32), Cell> = BTreeMap::new();
    let mut start = (-1, -1);
    let mut end = (-1, -1);

    let reader = io::BufReader::new(File::open("./sample1.txt")?);
    for (y, line) in reader.lines().enumerate() {
        for (x, symbol) in line?.chars().enumerate() {
            maze.insert((x as i32, y as i32), Cell{symbol: symbol});
            if symbol == 'S' {
                start = (x as i32 ,y as i32)
            }
            if symbol == 'E' {
                end = (x as i32, y as i32)
            }
        }
    }

    

    println!("{:?}", start);
    println!("{:?}", end);
    display_maze(maze);
    Ok(())
}

struct Cell {
    symbol: char
}

fn display_maze(maze: BTreeMap<(i32, i32), Cell>) {
    for i in 0..i32::MAX {
        for j in 0..i32::MAX {
            if let Some(cell) = maze.get(&(j, i)) {
                print!("{}", cell.symbol);
            }
            else {
                break;
            }
        }
        println!();
        if let None = maze.get(&(0, i)) {
            break;
        }
    }
}