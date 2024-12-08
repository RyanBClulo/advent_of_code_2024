use std::fs::File;
use std::io::{self, BufRead};

fn main() -> io::Result<()> {
    #[derive(Debug)]
    struct Equation {
        solution: i64,
        coefficients: Vec<i64>,
    }

    let path = "./input.txt";
    let file = File::open(&path)?;
    let reader = io::BufReader::new(file);

    let equations: Vec<Equation> = reader
        .lines()
        .filter_map(|line| line.ok())
        .map(|line| {
            line.split_once(':')
                .map(|(left, right)| {
                    let key = left.trim().parse::<i64>();
                    let values = right
                        .trim()
                        .split_whitespace()
                        .map(|num| num.parse::<i64>().unwrap())
                        .collect();
                    Equation {
                        solution: key.unwrap(),
                        coefficients: values,
                    }
                })
                .unwrap()
        })
        .collect();

    //Required hint from ChatGPT
    fn all_sum_multiply_combos(input: &Vec<i64>) -> Vec<i64> {
        fn combo(rest: Vec<i64>, current: i64, soultions: &mut Vec<i64>) {
            if rest.is_empty() {
                soultions.push(current);
                return;
            }
            let (num, rest_sub) = rest.split_first().unwrap();
            combo(rest_sub.to_vec(), num + current, soultions);
            combo(rest_sub.to_vec(), num * current, soultions);
            combo(rest_sub.to_vec(), combine_numbers(current, *num), soultions);
            
        }
        let mut solutions = Vec::<i64>::new();
        combo(input.clone()[1..].to_vec(), input[0], &mut solutions);
        solutions
    }

    let mut sum: i64 = 0;
    for eq in equations {
        let solutions = all_sum_multiply_combos(&eq.coefficients);
        println!("{:?} {:?}", eq, solutions);

        if solutions.contains(&eq.solution) {
            println!("{:?}", eq.coefficients);
            sum += eq.solution;
        }
    }

    println!("{}", sum);
    Ok(())
}

fn combine_numbers(a: i64, b: i64) -> i64 {
    let combined = format!("{}{}", a, b);
    combined.parse::<i64>().unwrap() 
}

#[test]
fn test_combine_numbers() {
    assert_eq!(combine_numbers(12, 34), 1234);
}