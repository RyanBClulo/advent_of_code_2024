use std::fs::File;
use std::io::{self, BufRead};

fn main() -> io::Result<()> {
    // Specify the path to the file
    let path = "./input.txt";
    // Open the file
    let file = File::open(&path)?;
    // Use a BufReader to read the file line by line
    let reader = io::BufReader::new(file);

    let mut list1 = Vec::<i32>::new();
    let mut list2 = Vec::<i32>::new();

    // Function to insert while maintaining order
    fn insert_sorted(vec: &mut Vec<i32>, value: i32) {
        let pos = vec.binary_search(&value).unwrap_or_else(|e| e);
        vec.insert(pos, value);
    }
    
    for line in reader.lines() {
        let unwrapped = line?;
        let mut numbers = unwrapped.split_whitespace();
        if let (Some(first), Some(second)) = (numbers.next(), numbers.next()) {
            if let (Ok(num1), Ok(num2)) = (first.parse::<i32>(), second.parse::<i32>()) {
                insert_sorted(&mut list1, num1);
                insert_sorted(&mut list2, num2);
            }
        }
    }

    let mut distance = 0;
    for(num1, num2) in list1.iter().zip(list2.iter()) {
        distance += (num2 - num1).abs();
    }

    println!("Distance {}", distance);

    let mut similarity = 0;
    for num1 in list1 {
        let mut count = 0;
        for num2 in &list2 {
            if num1 == *num2 {
                count+=1;
            }
        }
        similarity += num1 * count;
    }
    println!("Similarity {}", similarity);

    Ok(())
}
