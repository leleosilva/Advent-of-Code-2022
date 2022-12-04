// Checks if a limit fully contain the other
fn check_fully_overlapping_sections(limits_1: (i32, i32), limits_2: (i32, i32)) -> bool {
    if limits_1.0 >= limits_2.0 && limits_1.1 <= limits_2.1 { // Limit 1 is contained in limit 2
        true
    }
    else if limits_2.0 >= limits_1.0 && limits_2.1 <= limits_1.1 { // Limit 2 is contained in limit 1
        true
    }
    else { // The limits don't fully overlap
        false
    }
}

// Checks if limits overlap at all
fn check_overlapping_sections(limits_1: (i32, i32), limits_2: (i32, i32)) -> bool {
    if check_fully_overlapping_sections(limits_1, limits_2) { // First, checking if there is a complete overlap
        true
    }
    else {
        // For each value of limit 1, check if it is contained in range of limit 2
        for value in limits_1.0..(limits_1.1 + 1) {
            if (limits_2.0..(limits_2.1 + 1)).contains(&value) {
                return true;
            }
        }
        false // The limits don't overlap at all
    }
}

fn main() {
    println!(" --- Day 4: Camp Cleanup --- \n");
    println!("\t --- Part One --- \n");

    let assignment_pairs = std::fs::read_to_string("../../input/day4.txt").expect("File or directory not found");

    let mut fully_overlapping = 0;
    let mut overlapping = 0;
    for pair in assignment_pairs.lines() {

        // Getting the interval of sections for each Elf of the pair of Elves
        let (elf_1, elf_2) = pair
            .split_once(",")
            .expect("A pair should have no more than two Elves.");
        
        // Getting the limit section values for each Elf, and parsing from a string to i32
        let elf_1_limit_sections = elf_1
            .split_once("-")
            .unwrap();
        let limit_sections_1: (i32, i32) =
            (elf_1_limit_sections.0.parse().unwrap(),
             elf_1_limit_sections.1.parse().unwrap());

        let elf_2_limit_sections = elf_2
            .split_once("-")
            .unwrap();
        let limit_sections_2: (i32, i32) =
            (elf_2_limit_sections.0.parse().unwrap(),
             elf_2_limit_sections.1.parse().unwrap());
        

        if check_fully_overlapping_sections(limit_sections_1, limit_sections_2) {
            fully_overlapping += 1;
        }

        if check_overlapping_sections(limit_sections_1, limit_sections_2) {
            overlapping += 1;
        }
    }
    
    println!("The number of assignment pairs in which one range fully contain the other is {}.\n", fully_overlapping);
    
    println!("\t --- Part Two --- \n");

    println!("The number of assignment pairs in which the ranges overlap is {}.", overlapping);
}