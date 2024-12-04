// include the latest version of the regex crate in your Cargo.toml
extern crate regex;

use regex::Regex;
use std::fs;
use std::io;

fn main() -> io::Result<()> {
    let _ = part1();
    let _ = part2();
    Ok(())
}

fn part2() -> io::Result<()> {
    let path = "/Users/ryanclulo/Personal Documents/advent_of_code/day3/input.txt";
    let string = fs::read_to_string(path)?;

    // let string = "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";
    let regex_data = Regex::new(r"(?m)do\(\)|don't\(\)|mul\(\d{1,3},\d{1,3}\)").unwrap();
    let regex_mul = Regex::new(r"(?m)mul\((?P<num1>\d{1,3}),(?P<num2>\d{1,3})\)").unwrap();

    let mut on = true;
    let mut sum = 0;
    for (name, []) in regex_data.captures_iter(&string).map(|x| x.extract()) {
        // println!("{}", name);
        if name == "don't()" {
            on = false;
        } else if name == "do()" {
            on = true;
        } else {
            let mul_capture = regex_mul.captures(name).unwrap();
            if let (Ok(parsed1), Ok(parsed2)) = (
                &mul_capture["num1"].parse::<i32>(),
                &mul_capture["num2"].parse::<i32>(),
            ) {
                if on {
                    sum += parsed1 * parsed2;
                    // println!("{} {} {}", parsed1, parsed2, sum);
                }
            }
        }
    }
    println!("Part 2: {}", sum);
    Ok(())
}

fn part1() -> io::Result<()> {
    let path = "/Users/ryanclulo/Personal Documents/advent_of_code/day3/input.txt";

    let regex = Regex::new(r"(?m)mul\((?P<num1>\d{1,3}),(?P<num2>\d{1,3})\)").unwrap();
    // let string = "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";
    let string = fs::read_to_string(path)?;

    // result will be an iterator over tuples containing the start and end indices for each match in the string
    let result = regex.captures_iter(&string);

    let mut sum = 0;
    for (_, [num1, num2]) in result.map(|x| x.extract()) {
        if let (Ok(parsed1), Ok(parsed2)) = (num1.parse::<i32>(), num2.parse::<i32>()) {
            sum += parsed1 * parsed2;
            // println!("{} {} {}", parsed1, parsed2, sum);
        }
    }
    println!("Part 1: {}", sum);
    Ok(())
}

// fn part2_failure(read: &str) {
//     // let read = "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";
//     let string = "do()".to_owned() + &read + "don't()";

//     let data_regex = Regex::new(r"(?m)do\(\)(?P<data>.*?)don't\(\)").unwrap();
//     let mul_regex = Regex::new(r"(?m)mul\((?P<num1>\d{1,3}),(?P<num2>\d{1,3})\)").unwrap();

//     let datas = data_regex.captures_iter(&string);

//     let mut sum = 0;
//     for (_, [data]) in datas.map(|d| d.extract()) {
//         // println!("{}\n", data);
//         for (_, [num1, num2]) in mul_regex.captures_iter(data).map(|x| x.extract()) {
//             if let (Ok(parsed1), Ok(parsed2)) = (num1.parse::<i32>(), num2.parse::<i32>()) {
//                 sum += parsed1 * parsed2;
//                 println!("{} {} {}", parsed1, parsed2, sum);
//             }
//         }
//     }
// }
