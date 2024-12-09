use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};

struct Input {
    rules: HashMap<i32, Vec<i32>>,
    updates: Vec<Vec<i32>>
}
fn main() {
    
    fn read_file() -> io::Result<Input> {
        let path = "./sample.txt";
        let file = File::open(&path)?;
        let reader = io::BufReader::new(file);

        let mut input = Input {
            rules : HashMap::new(),
            updates : Vec::new()
        };

        for line in reader.lines() {
            let unwrapped = line?;
            if unwrapped.contains("|") {
                if let Some((key, value)) = unwrapped.split_once('|') {
                    input.rules.entry(key.parse::<i32>().unwrap()).or_insert_with(Vec::new)
                        .push(value.parse::<i32>().unwrap());
                }
            }
            else if unwrapped.contains(",") {
                input.updates.push(unwrapped.split(',').map(|v| v.parse::<i32>().unwrap()).collect());
            }
        }

        Ok(input)
    }

    fn is_valid_update(input: &Input) -> bool {
        true
    }

    let input = read_file().unwrap();
    println!("{:?}", input.rules);
    println!("{:?}", input.updates);

    let mut sum = 0;
    for update in &input.updates {
        for i in 0..update.len() {
            for j in 0..i {

            } 
        }

        if is_valid_update(&input) {
            sum += get_middle(&update).unwrap();
        }
    }

    println!("{}", sum);

}

fn get_middle(vec: &Vec<i32>) -> Option<i32> {
    if vec.is_empty() {
        None
    } else {
        let middle_index = vec.len() / 2;
        Some(vec[middle_index])
    }
}

#[test]
fn test_get_middle() {
    assert_eq!(get_middle(&[1,2,3,4,5].to_vec()).unwrap(), 3);
}