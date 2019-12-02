use std::env;
use std::fs;


fn run_program(mut state: Vec<usize>, noun: usize, verb: usize) -> usize {
    let mut i = 0;
    state[1] = noun;
    state[2] = verb;

    while i < state.len() {
        let instruction = state[i];

        if instruction == 99 {
            break;
        }

        let mut idxs: [usize; 3] = [0; 3];
        idxs.copy_from_slice(&state[i + 1..i + 4]);
        i += 4;

        if instruction == 1 {
            state[idxs[2]] = state[idxs[0]] + state[idxs[1]];
        }
        else if instruction == 2 {
            state[idxs[2]] = state[idxs[0]] * state[idxs[1]];
        }
        else {
            panic!("Unknown instruction");
        }
    }

    return state[0];
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

    let values: Vec<usize> = unwrapped_contents.trim().split(",").map(
        |v| v.parse::<usize>().unwrap()
    ).collect();

    let result1 = run_program(values.clone(), 12, 2);
    let mut result2 = 0;

    'outer: for n in 0..100 {
        'inner: for v in 0..100 {
            if run_program(values.clone(), n, v) == 19690720 {
                result2 = 100 * n + v;
                break 'outer;
            }
        }
    }

    println!("=== Part One ===");
    println!("Result: {}", result1);

    println!("=== Part One ===");
    println!("Result: {}", result2);

}