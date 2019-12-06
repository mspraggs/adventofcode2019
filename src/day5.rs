use std::env;
use std::fs;


fn run_program(mut state: Vec<i32>, input: i32) -> i32 {
    let mut i = 0;
    let mut output = 0;

    while i < state.len() {
        let opcode = state[i];
        i += 1;

        let modes: Vec::<i32> = [100, 1000, 10000].iter().map(
            |n| (opcode / *n) % 10
        ).collect();
        let instruction = opcode % 100;

        if instruction == 99 {
            break;
        }

        let num_args = match instruction {
            1 => 3,
            2 => 3,
            3 => 1,
            4 => 1,
            5 => 2,
            6 => 2,
            7 => 3,
            8 => 3,
            i => {
                panic!("Unknown instruction: {}", i);
            },
        };

        let mut args: Vec::<i32> = vec![0; num_args];
        args.copy_from_slice(&state[i..i + num_args]);
        i += num_args;

        if instruction == 1 || instruction == 2 ||
           instruction == 5 || instruction == 6 ||
           instruction == 7 || instruction == 8  {
            for i in 0..2 {
                if modes[i] == 0 {
                    args[i] = state[args[i] as usize];
                }
            }
        }
        else if instruction == 4 {
            if modes[0] == 0 {
                args[0] = state[args[0] as usize];
            }
        }

        if instruction == 1 {
            state[args[2] as usize] = args[0] + args[1];
        }
        else if instruction == 2 {
            state[args[2] as usize] = args[0] * args[1];
        }
        else if instruction == 3 {
            state[args[0] as usize] = input;
        }
        else if instruction == 4 {
            output = args[0];
            println!("{}", output);
        }
        else if instruction == 5 {
            if args[0] != 0 {
                i = args[1] as usize;
            }
        }
        else if instruction == 6 {
            if args[0] == 0 {
                i = args[1] as usize;
            }
        }
        else if instruction == 7 {
            state[args[2] as usize] = if args[0] < args[1] { 1 } else { 0 };
        }
        else if instruction == 8 {
            state[args[2] as usize] = if args[0] == args[1] { 1 } else { 0 };
        }
        else {
            panic!("Unknown instruction");
        }
    }

    return output;
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        println!("Usage: {} <input file>", args[0]);
        return;
    }

    let contents = fs::read_to_string(&args[1]);
    let unwrapped_contents = match contents {
        Ok(s) => s,
        Err(e) => {
            panic!("Problem reading file: {:?}", e)
        },
    };

    let values: Vec<i32> = unwrapped_contents.trim().split(",").map(
        |v| v.parse::<i32>().unwrap()
    ).collect();

    let result1 = run_program(values.clone(), 1);

    println!("=== Part One ===");
    println!("Result: {}", result1);

    let result2 = run_program(values.clone(), 5);

    println!("=== Part Two ===");
    println!("Result: {}", result2);
}