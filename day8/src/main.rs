use std::collections::HashMap;
use std::collections::HashSet;
use std::fs::File;
use std::io::{self, BufRead};

fn main() -> io::Result<()> {
    let grid = read_file("./input.txt")?;
    let mut antenna: HashMap<char, Vec<(i32, i32)>> = HashMap::new();
    let mut antinodes: HashSet<(i32, i32)> = HashSet::new();

    for i in 0..grid.len() {
        for j in 0..grid[0].len() {
            print!("{}", grid[i][j]);
            if grid[i][j] != '.' {
                antenna.entry(grid[i][j]).or_insert_with(Vec::new)
                    .push((i as i32, j as i32));
            }
        }
        println!()
    }
    println!("{:?}", antenna);

    for entry in antenna {
        for i in 0..entry.1.len() {
            for j in i+1..entry.1.len() {
                let nodes = calc_antinodes(entry.1[i], entry.1[j]);
                println!("{:?} {:?} {:?}", entry.1[i], entry.1[j], nodes);
                if is_valid_antinode(nodes.0, grid.len() as i32, grid[0].len() as i32) {
                    antinodes.insert(nodes.0);
                }
                if is_valid_antinode(nodes.1, grid.len() as i32, grid[0].len() as i32) {
                    antinodes.insert(nodes.1);
                }
            }
        }
    }

    println!("{:?}", antinodes);
    println!("{}", antinodes.len());
    let _ = print_sample_solution_antinodes();

    Ok(())
}

fn is_valid_antinode(node: (i32, i32), x: i32, y: i32) -> bool {
    node.0 >= 0 && node.1 >= 0 && node.0 < x && node.1 < y
}

fn calc_antinodes(first: (i32, i32), second: (i32, i32)) -> ((i32, i32), (i32, i32)){
    let rise = first.0 - second.0;
    let run = first.1 - second.1;
    // println!("{} {}", rise, run);
    // println!("{} {}", first.0 + rise, first.1 + run);
    // println!("{} {}", second.0 - rise, second.1 - run);
    ((first.0 + rise, first.1 + run), (second.0 - rise, second.1 - run))
}

#[test]
fn test_calc_antinodes() {
    assert_eq!(calc_antinodes((3,4), (5,5)), ((1, 3), (7, 6)));
    assert_eq!(calc_antinodes((1,8), (2,5)), ((0, 11), (3, 2)));
}

fn read_file(path: &str) -> io::Result<Vec<Vec<char>>> {
    let file = File::open(&path)?;
    let reader = io::BufReader::new(file);

    Ok(reader
        .lines()
        .filter_map(|line| line.ok())
        .map(|line| line.chars().collect())
        .collect())
}

fn print_sample_solution_antinodes() -> io::Result<()> {
    let grid = read_file("./sample_solution.txt")?;
    let mut antinodes = HashSet::new();
    for i in 0..grid.len() {
        for j in 0..grid[0].len() {
            if grid[i][j] == '#' {
                antinodes.insert((i, j));
            }
        }
    }
    println!("{:?}", antinodes);
    Ok(())
}