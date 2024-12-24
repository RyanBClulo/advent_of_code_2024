use std::collections::HashMap;

const NUMERICS: &[char] = &['7', '8', '9', '4', '5', '6', '1', '2', '3', '.', '0', 'A'];
const DIRECTIONS: &[char] = &['.', '^', 'A', '<', 'v', '>'];

fn main() {
    let numeric_lookup = generate_lookup(NUMERICS);
    let direction_lookup = generate_lookup(DIRECTIONS);

    let sample_sequences = vec!["379A"];
    // let sample_sequences = vec!["029A", "980A", "179A" ,"456A", "379A"];

    let mut total_complexity = 0;
    for input in sample_sequences {
        println!("{}", input);
        let mut next = convert_input(input, &numeric_lookup);
        println!("{}", next);
        next = convert_input(&next, &direction_lookup);
        println!("{}", next);
        next = convert_input(&next, &direction_lookup);
        println!("{}", next);
        let complexity = next.len() as i32 * input.replace("A", "").parse::<i32>().unwrap();
        println!("{} {} {}\n", next.len(), input, complexity);
        total_complexity += complexity
    }
    println!("{}", total_complexity);

    // _print_lookup(&direction_lookup);

}

fn convert_input(input: &str, lookup: &HashMap<(char, char), String>) -> String {
    let mut sbuild = "".to_string();
    let mut point = 'A';

    for c in input.chars() {
        sbuild += lookup.get(&(point, c)).unwrap();
        println!("={:?} {}", (point, c), lookup.get(&(point, c)).unwrap());
        point = c;
    }

    sbuild
}

fn _print_lookup(lookup: &HashMap<(char, char), String>) {
    for entry in lookup{
        println!("{:?}, {}", entry.0, entry.1)
    }
}

fn generate_lookup(keypad: &[char]) -> HashMap<(char, char), String> {
    let mut lookup : HashMap<(char, char), String> = HashMap::new();
    for dir1 in keypad {
        for dir2 in keypad {
            if *dir1 != '.' && *dir2 != '.' {
                lookup.insert((*dir1, *dir2), shortest_sequence(keypad, *dir1, *dir2));
            }
        }
    }
    lookup
}

fn shortest_sequence(keypad: &[char], start: char, end: char) -> String {
    let mut starts_pos = (0, 0);
    let mut end_pos = (0, 0);
    for i in 0..keypad.len() {
        if keypad[i] == start {
            starts_pos = (i as i32 % 3, i as i32 / 3);
        }
        if keypad[i] == end {
            end_pos = (i as i32 % 3, i as i32 / 3);
        }
    }

    let dif = (starts_pos.0 - end_pos.0, starts_pos.1 - end_pos.1);

    let mut sbuild: String = "".to_string();
    if dif.0 <= 0 {
        sbuild += &">".repeat(dif.0.abs() as usize)
    }
    if dif.1 > 0 {
        sbuild += &"^".repeat(dif.1 as usize)
    } 
    if dif.1 <=0 {
        sbuild += &"v".repeat(dif.1.abs() as usize)
    }
    if dif.0 > 0 {
        sbuild += &"<".repeat(dif.0 as usize)
    }

    // println!("{}", sbuild.clone() + "A");
    sbuild + "A"
}

#[test]
fn test_single_sequence() {
    let numeric_lookup = generate_lookup(NUMERICS);
    let direction_lookup = generate_lookup(DIRECTIONS);
    
    let mut next = convert_input("029A", &numeric_lookup);
    assert_eq!(next, "<A^A>^^AvvvA");
    next = convert_input(&next, &direction_lookup);
    assert_eq!(next, "v<<A>>^A<A>AvA<^AA>A<vAAA>^A");
    next = convert_input(&next, &direction_lookup);
    assert_eq!(next.len(), 68);
    // assert_eq!(next, "<vA<AA>>^AvAA<^A>A<v<A>>^AvA^A<vA>^A<v<A>^A>AAvA^A<v<A>A>^AAAvA<^A>A");
}