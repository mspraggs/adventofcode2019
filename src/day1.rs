use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    
    if args.len() != 2 {
        println!("Usage: {} <input file>", args[0]);
        return;
    }

    let contents = fs::read_to_string(&args[1]).unwrap();
    let mut sum1: i32 = 0;
    let mut sum2: i32 = 0;

    for line in contents.as_str().lines() {
        let value = line.parse::<i32>().unwrap();
        let mut fuel_sum = value / 3 - 2;
        println!("{}", fuel_sum);

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