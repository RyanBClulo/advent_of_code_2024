use std::str::FromStr;
use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};

fn main() -> io::Result<()> {
    let reader = io::BufReader::new(File::open("./input.txt")?);
    let mut switch = true;

    let mut initial_state = HashMap::new();
    let mut operations = HashMap::new();

    for reader_line in reader.lines() {
        let line = reader_line?;

        if line.is_empty() {
            switch = false;
            continue;
        }

        if switch {
            let (wire, value) = line.split_once(": ").expect("Failed to split state");
            initial_state.insert(wire.to_string(), value.parse::<i32>().unwrap());
        }
        else {
            let (function, result) = line.split_once("-> ").expect("Failed to split op");
            let comp: Vec<&str> = function.split_ascii_whitespace().collect();
            operations.insert(result.to_string(), 
                Operation{in1: comp[0].to_string(), in2: comp[2].to_string(), op: Operator::from_str(comp[1]).unwrap()});
        }
    }

    let mut results = Vec::new();
    for operation in &operations {
        let result = evaluate(operation.0.to_string(), &operations, &initial_state);
        results.push((operation.0.to_string(), result));
    }

    results.sort_by_key(|entry| entry.0.to_string());

    let mut binary_number: i64 = 0;
    for (i, (_, bit)) in results.iter().filter(|e| e.0.starts_with('z')).enumerate() {
        binary_number |= (*bit as i64) << i;
    }
    println!("{}", binary_number);

    Ok(())
}

fn evaluate(input: String, operations: &HashMap<String, Operation>, lookup: &HashMap<String, i32>) -> i32 {
    if let Some(result) = lookup.get(&input) {
        return *result;
    }
    else {
        if let Some(todo) = operations.get(&input) {
            return todo.op.eval(evaluate(todo.in1.clone(), operations, lookup), 
                evaluate(todo.in2.clone(), operations, lookup))
        }
    }
    0
}

#[derive(Debug)]
enum Operator {
    AND,
    OR,
    XOR,
}

impl Operator {
    fn eval(&self, in1: i32, in2: i32) -> i32 {
        return match self {
            Operator::AND => in1 & in2,
            Operator::OR => in1 | in2,
            Operator::XOR => in1 ^ in2
        }
    }
}

impl FromStr for Operator {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "AND" => Ok(Operator::AND),
            "OR" => Ok(Operator::OR),
            "XOR" => Ok(Operator::XOR),
            _ => Err(format!("Invalid operator: {}", s)),
        }
    }
}

#[derive(Debug)]
struct Operation {
    in1: String,
    in2: String,
    op: Operator
}