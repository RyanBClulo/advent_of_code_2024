use std::{fs, io};

fn main() -> io::Result<()>{
    let disk_map = fs::read_to_string("./sample.txt")?;
    let blocks: Vec<i8> = disk_map.chars().map(|c| c as i8 - 0x30).collect();
    let expanded: &mut Vec<i32> = &mut expand(blocks);


    // println!("{}", disk_map);
    // println!("{:?}", blocks);
    // println!("{:?}", expanded);
    compress_blocks(expanded);
    // println!("{:?}", expanded);
    println!("{}", compute_checksum(expanded));

    Ok(())
}

fn expand(blocks: Vec<i8>) -> Vec<i32> {
    let mut expand: Vec<i32> = Vec::new();
    for i in 0..blocks.len() {
        for _ in 0..blocks[i] {
            if i % 2 == 0 {
                // print!("{}", i/2);
                expand.push(i as i32 / 2);
            } 
            else {
                // print!(".");
                expand.push(-1);
            }
        }
    }
    // println!();
    expand
}

fn compress_blocks(disk_ids: &mut Vec<i32>) {
    let mut empty = next_empty(disk_ids, 0);
    let mut last = disk_ids.len() - 1;

    while empty < last {
        disk_ids[empty] = disk_ids[last];
        disk_ids[last] = -1;
        last -=1;
        empty = next_empty(disk_ids, empty);
        // println!("{} {} {:?}", empty, last, disk_ids)
    }
    println!("Last {} {} {:?}", empty, last, disk_ids)
}

#[warn(dead_code)] // Part 2 to be continued
fn compress_files() {

}

fn next_empty(disk_ids: &mut Vec<i32>, empty: usize) -> usize {
    for i in empty..disk_ids.len() {
        if disk_ids[i] == -1 {
            return i;
        }
    }
    disk_ids.len()
}

fn compute_checksum(disk_ids: &Vec<i32>) -> i64 {
    let mut sum: i64 = 0;
    for i in 0..disk_ids.len() {
        if disk_ids[i] != -1 {
            sum += (i as i32 * disk_ids[i] as i32) as i64;
        }
    }
    sum
}