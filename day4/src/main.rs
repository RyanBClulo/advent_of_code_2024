use std::fs::File;
use std::io::{self, BufRead};

fn main() {
    fn read_file() -> io::Result<Vec<Vec<char>>> {
        let path = "./input.txt";
        let file = File::open(&path)?;
        let reader = io::BufReader::new(file);

        let grid: Vec<Vec<char>> = reader
            .lines()
            .filter_map(|line| line.ok())
            .map(|line| line.chars().collect())
            .collect();

        Ok(grid)
    }

    fn valid_coord(x: i32, y: i32, m: i32, n: i32) -> bool {
        return x >= 0 && y >= 0 && x < m && y < n
    }

    // fn next_xmas_letter(c: char) -> char {
    //     let find: Vec<char> = "XMAS!".chars().collect();
    //     for search in  0..find.len() {
    //         if find[search] == c {
    //             return find[search+1];
    //         }
    //     }
    //     '!'
    // }

    fn find_word(grid: &Vec<Vec<char>>, word: Vec<char>, index: i32, x: i32, y: i32, dx: i32, dy: i32) -> bool {
        if index == word.len() as i32 {
            return true;
        }
        if valid_coord(x, y, grid.len() as i32, grid[0].len() as i32) && word[index as usize] == grid[x as usize][y as usize] {
            return find_word(grid, word, index+1, x+dx, y+dy, dx, dy)
        }
        false
    }

    let grid = read_file().unwrap();

    let dxs = [ -1, -1, -1, 0, 0, 1, 1, 1 ];
    let dxy = [ -1, 0, 1, -1, 1, -1, 0, 1 ];

    let mut count_words = 0;
    for row in 0..grid.len() {
        for col in 0..grid[0].len() {
            // print!("{}", col);
            for dir in 0..8 {
                if find_word(&grid, "XMAS".chars().collect(), 0, row as i32, col as i32, dxs[dir], dxy[dir]) {
                    count_words +=1;
                }
            }
        }
        // println!()
    }
    println!("{}", count_words);

}
