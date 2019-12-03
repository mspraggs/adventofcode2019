use std::collections;
use std::env;
use std::fs;


fn build_coordinates(routing: &str) -> collections::HashMap::<(i32, i32), usize> {
    let mut coordinates = collections::HashMap::<(i32, i32), usize>::new();
    let mut coordinate: (i32, i32) = (0, 0);
    let mut distance = 0;
    coordinates.insert(coordinate, distance);

    for route in routing.split(",") {
        
        let (dir, dist_raw) = route.split_at(1);
        let dist = dist_raw.parse::<usize>().unwrap();
        
        let offset = match dir {
            "R" => (1, 0),
            "L" => (-1, 0),
            "U" => (0, -1),
            "D" => (0, 1),
            &_ => {
                panic!("Unexpected direction!");
            },
        };
        
        for _ in 0..dist {
            coordinate.0 += offset.0;
            coordinate.1 += offset.1;
            distance += 1;
            coordinates.insert(coordinate, distance);
        }
    }

    return coordinates;
}


fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        println!("Usage: {} <input file>", args[0]);
        return;
    }

    let contents = fs::read_to_string(&args[1]);
    let contents_unwrapped = match contents {
        Ok(s) => s,
        Err(e) => {
            panic!("Problem reading file: {:?}", e)
        },
    };

    let mut wires = Vec::<collections::HashMap::<(i32, i32), usize>>::new();

    for line in contents_unwrapped.as_str().lines() {
        wires.push(build_coordinates(&line));
    }

    let mut coordinates = wires[0].clone();
    coordinates.remove(&(0, 0));

    for wire in &wires[1..] {
        coordinates.retain(|k, _v| wire.contains_key(k));
    }

    let min_distance = coordinates.iter().map(
        |(k, _v)| k.0.abs() as usize + k.1.abs() as usize
    ).min().unwrap();

    let total_min_time: usize = coordinates.iter().map(
        |c| wires.iter().map(
            |w| w.get(c.0).unwrap()
        ).sum::<usize>()
    ).min().unwrap();

    println!("=== Part One ===");
    println!("Min distance = {}", min_distance);

    println!("=== Part Two ===");
    println!("Min pulse length = {}", total_min_time);
}
