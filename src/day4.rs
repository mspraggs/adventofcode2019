fn is_candidate(num: usize, disjoint_pairs: bool) -> bool {
    let mut digits = Vec::<usize>::new();
    let mut working_num = num;

    while working_num > 0 {
        digits.push(working_num % 10);
        working_num /= 10;
    }

    digits.reverse();

    let mut counts = [0; 10];

    for (i, n) in digits.iter().enumerate() {
        counts[*n] += 1;

        if i == 0 {
            continue;
        }

        if *n < digits[i - 1] {
            return false;
        }
    }

    if disjoint_pairs {
        return counts.iter().any(|n| *n == 2);
    }
    return counts.iter().any(|n| *n > 1);
}


fn main() {

    let range = 356261..846303;

    let candidate_count_part_1 = range.clone().filter(
        |n| is_candidate(*n, false)
    ).count();
    let candidate_count_part_2 = range.clone().filter(
        |n| is_candidate(*n, true)
    ).count();
    
    println!("=== Part One ===");
    println!("Num candidates = {}", candidate_count_part_1);
    println!("=== Part Two ===");
    println!("Num candidates = {}", candidate_count_part_2);
}
