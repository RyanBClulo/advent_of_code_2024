use std::fs::File;
use std::io::{self, BufRead};

fn main() -> io::Result<()> {

    fn analyze_report(path: &str) -> io::Result<i32> {
        let file = File::open(&path)?;
        let reader = io::BufReader::new(file);

        let mut safe = 0;
        for line in reader.lines() {
            let unwrapped = line?;
            let numbers: Vec<i32> = unwrapped.split_whitespace()
                .filter_map(|num| num.parse::<i32>().ok())
                .collect();

            let increasing = numbers.windows(2).all(|pair| pair[0] < pair[1]);
            let decreasing = numbers.windows(2).all(|pair| pair[0] > pair[1]);
            let maximum = numbers.windows(2).all(|pair| (pair[0] - pair[1]).abs() < 4);

            if  (increasing || decreasing) && maximum {
                safe +=1;
            }
        }

        Ok(safe)
    }

    fn analyze_report_dampened(path: &str) -> io::Result<i32> {
        let file = File::open(&path)?;
        let reader = io::BufReader::new(file);

        let mut safe = 0;
        for line in reader.lines() {
            let unwrapped = line?;
            let numbers: Vec<i32> = unwrapped.split_whitespace()
                .filter_map(|num| num.parse::<i32>().ok())
                .collect();

        //     let increasing = numbers.windows(2).filter(|pair| pair[0] < pair[1]).count();
        //     let decreasing = numbers.windows(2).filter(|pair| pair[0] > pair[1]).count();
        //     let maximum = numbers.windows(3).filter(|trip| (trip[0] - trip[1]).abs() > 4
        //         || (trip[0] - trip[2]).abs() > 4).count();

        //     println!("{} {} {}", increasing, decreasing, maximum);
        //     if  (increasing >= numbers.len() - 2 || decreasing >= numbers.len() -2) && maximum <= 1 {
        //         // println!("{} {} {}", increasing, decreasing, maximum);
        //         // println!("{}", unwrapped);
        //         safe +=1;
        //     }

            if check_row(&numbers) {
                safe +=1;
            }
        }

        Ok(safe)
    }

    let sample = "/Users/ryanclulo/Personal Documents/advent_of_code/day2/sample.txt";
    let sample_result: i32 = analyze_report(sample)?;
    println!("Part 1 Sample: {}", sample_result);

    let input = "/Users/ryanclulo/Personal Documents/advent_of_code/day2/input.txt";
    let input_result = analyze_report(input)?;
    println!("Part 1 Input: {}", input_result);

    let sample_result: i32 = analyze_report_dampened(sample)?;
    println!("Part 2 Sample: {}", sample_result);

    let input_result = analyze_report_dampened(input)?;
    println!("Part 2 Input: {}", input_result);

    Ok(())
}

// Stollen from ChatGPT
fn check_row(numbers: &[i32]) -> bool {
    // Helper to check if the sequence is strictly increasing or decreasing
    let is_increasing = |slice: &[i32]| slice.windows(2).all(|w| w[0] < w[1]);
    let is_decreasing = |slice: &[i32]| slice.windows(2).all(|w| w[0] > w[1]);
    let within_difference = |slice: &[i32]| slice.windows(2).all(|w| (w[0] - w[1]).abs() >= 1 && (w[0] - w[1]).abs() <= 3);

    // Check without removing any elements
    if (is_increasing(numbers) || is_decreasing(numbers)) && within_difference(numbers) {
        return true;
    }

    // Check with one element removed
    for i in 0..numbers.len() {
        let mut reduced = numbers.to_vec();
        reduced.remove(i);
        if (is_increasing(&reduced) || is_decreasing(&reduced)) && within_difference(&reduced) {
            return true;
        }
    }

    false
}