use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};

fn main() -> io::Result<()> {

    let reader = io::BufReader::new(File::open("./sample.txt")?);

    let mut patterns: Vec<String> = Vec::new();
    let mut designs: Vec<String> = Vec::new();

    let mut found_break = false;
    for read_line in reader.lines() {
        let line = read_line?;

        if line.trim().is_empty() {
            found_break = true; 
            continue;
        }

        if found_break {
            designs.push(line);
        } 
        else {
            let pattern_split : Vec<&str> = line.split(", ").collect();
            for pattern in pattern_split {
                patterns.push(pattern.to_string());
            }
        }

    }
    
    patterns.sort_by(|a, b| b.len().cmp(&a.len()));

    // Part 1
    // let mut count = 0;
    // for design in designs {
    //     if check_design(design.clone(), &patterns) {
    //         println!("Found {}", design);
    //         count +=1;
    //     }
    // }
    // println!("{}", count);

    let mut pattern_ways: HashMap<String, i32> = HashMap::new();
    for pattern in patterns.clone() {
        pattern_ways.insert(pattern.clone(), find_pattern_costs(pattern, &patterns));
    }
    for e in pattern_ways.iter() {
        println!("{:?}", e);
    }

    Ok(())
}

fn check_design(design: String, patterns: &Vec<String>) -> bool {
    if design.is_empty() {
        return true;
    }
    let mut found = false;
    for pattern in patterns {
        if design.starts_with(*&pattern) {
            found |= check_design(design.replacen(pattern, "", 1), patterns);
            if found {
                break;
            }
        }
    }
    found
}

fn find_pattern_costs(design: String, patterns: &Vec<String>) -> i32 {
    // println!("{}", design);
    if design.is_empty() {
        return 1;
    }
    let mut found = 0;
    for pattern in patterns {
        if design.starts_with(*&pattern) {
            found += find_pattern_costs(design.replacen(pattern, "", 1), patterns);
        }
    }
    found
}