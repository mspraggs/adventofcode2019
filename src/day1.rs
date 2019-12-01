use std::env;
use std::fs;

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

    let mut sum1: i32 = 0;
    let mut sum2: i32 = 0;

    for line in unwrapped_contents.as_str().lines() {
        let value = line.parse::<i32>();
        let unwrapped_value = match value {
            Ok(v) => v,
            Err(e) => {
                panic!("Problem parsing int: {:?}", e)
            },
        };

        let mut fuel_sum = unwrapped_value / 3 - 2;
        sum1 += fuel_sum;
        sum2 += fuel_sum;

        loop {
            fuel_sum = fuel_sum / 3 - 2;
            if fuel_sum <= 0 {
                break;
            }
            sum2 += fuel_sum;
        }
    }

    println!("=== Part One ===");
    println!("Total mass = {}", sum1);

    println!("=== Part Two ===");
    println!("Total mass = {}", sum2);
}