
// Part 1
static A: i32 = 64012472;
static B: i32 = 0;
static C: i32 = 0;
static PROGRAM: &str = "2,4,1,7,7,5,0,3,1,7,4,1,5,5,3,0";

// Part 2
// static A: i32 = 117440;
// static B: i32 = 0;
// static C: i32 = 0;
// static PROGRAM: &str = "0,3,5,4,3,0";

fn main() {
    println!("{:?}", run(PROGRAM, A, B, C));
}

fn run(program_str: &str, mut a: i32, mut b: i32, mut c: i32) -> String {
    let program: Vec<i32> = program_str.split(',').map(|c| c.parse::<i32>().unwrap()).collect();
    let mut ip = 0;
    let mut result: Vec<i32> = Vec::new();

    loop {
        if ip >= program.len() as i32 {
            break;
        }
        
        let mut out = 0;
        let opcode = program[ip as usize];
        let operand: i32 = match program[(ip + 1) as usize] {
            0 => 0,
            1 => 1,
            2 => 2,
            3 => 3,
            4 => a,
            5 => b,
            6 => c,
            _ => program[(ip + 1) as usize]
        };

        // println!("{:?}", program);
        print!("ip={}, a={}, b={}, c={}, opcode={}, operand={} ", ip, a, b, c, opcode, operand);

        match opcode {
            0 => { //Opcode 0 (adv)
                a = a / i32::pow(2, operand.try_into().unwrap());
            }
            1 => { //Opcode 0 (bxl)
                b = b ^ operand;
            }
            2 => { //Opcode 2 (bst)
                b = operand % 8;
            }
            3 => { //Opcode 3 (jnz)
                if a !=0 {
                    ip = operand;
                    continue;
                }
            }
            4 => { //Opcode 4 (bxc)
                b = b ^ c;
            }
            5 => { //Opcode 5 (out)
                out = operand % 8;
                result.push(out);
            }
            6 => { //Opcode 6 (bdv)
                b = a / i32::pow(2, operand.try_into().unwrap());
            }
            7 => { //Opcode 7 (cdv)
                c = a / i32::pow(2, operand.try_into().unwrap());
            }
            _ => println!("{} - Error", opcode)
        }

        println!("| a={}, b={}, c={}, out={}", a, b, c, out);   
        ip+=2;
    }
    
    result.iter().map(|n| n.to_string()).collect::<Vec<String>>().join(",")
}

#[test]
fn tests() {
    assert_eq!(run("0,1,5,4,3,0", 729, 0, 0), "4,6,3,5,6,3,5,2,1,0");
    assert_eq!(run("0,1,5,4,3,0", 2024, 0, 0), "4,2,5,6,7,7,7,7,3,1,0");
}