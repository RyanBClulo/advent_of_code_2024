use std::collections::{BTreeMap, HashSet};
use std::fs::File;
use std::io::{self, BufRead};

fn main() -> io::Result<()> {
    let reader = io::BufReader::new(File::open("./input.txt")?);

    let mut connections: BTreeMap<String, Vec<String>> = BTreeMap::new();

    for reader_line in reader.lines() {
        let line = reader_line?;
        let (first, second) = line.split_once('-').expect("Failed to split string");

        connections.entry(first.to_string()).or_insert(Vec::new()).push(second.to_string());
        connections.entry(second.to_string()).or_insert(Vec::new()).push(first.to_string());
    }

    find_cycles(&connections);

    brute_force_part1(&connections);

    Ok(())
}

fn find_cycles(connections: &BTreeMap<String, Vec<String>>) {
    for computer in connections.into_iter() {
        let mut visited: Vec<String> = Vec::new();
        let cycle = help_find_cycles(&computer.0, &connections, &mut visited);
        // println!("{} {:?}", computer.0, visited)
    }
}

fn help_find_cycles(computer: &String, connections: &BTreeMap<String, Vec<String>>, visited: &mut Vec<String>) -> bool {
    if visited.contains(computer) {
        visited.push(computer.clone());
        return true;
    }
    visited.push(computer.clone());

    if let Some(connection) = connections.get(computer) {
        for node in connection.iter() {
            if help_find_cycles(node, connections, visited) {
                return true;
            }
        }
    }

    false
}

fn brute_force_part1(connections: &BTreeMap<String, Vec<String>>) {
    let mut collections: Vec<HashSet<String>> = Vec::new();
    for computer1 in connections.iter() {
        for computer2 in computer1.1 {
            if let Some(conn) = connections.get(computer2) {
                for computer3 in conn {
                    if !computer3.eq(computer1.0) && connections.get(computer3).unwrap().contains(computer1.0) {
                        // println!("{} {} {}",computer1.0, computer2, computer3);
                        let mut collection = HashSet::new();
                        collection.insert(computer1.0.clone());
                        collection.insert(computer2.clone());
                        collection.insert(computer3.clone());
                        if !collections.contains(&collection) {
                            collections.push(collection);
                        }
                    }
                }
            }
        }
    }

    let reduced: Vec<&HashSet<String>> = collections.iter().filter(|c| c.iter().any(|e| e.starts_with('t'))).collect();

    // for collection in reduced {
    //     println!("{:?}", collection);
    // }
    println!("{}", reduced.len())
}