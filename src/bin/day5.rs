use std::collections::VecDeque;

// Parsing stacks of crates from Strings to VecDeques
fn parse_crates(layers: &str) -> Vec<VecDeque<char>> {
    let mut number_of_stacks = 0;
    let mut crates_per_layer = 0;

    let mut stacks: Vec<VecDeque<char>> = Vec::new();

    for layer in layers.lines() {
        if layer.is_empty() { // If the line is empty, there are no more stacks to parse
            break;
        }
        
        // Skipping the first element ( '[' ), and iterating by 4 to get each crate value
        for stack_element in layer.chars().skip(1).step_by(4) {
            
            crates_per_layer += 1; // Incrementing number of elements per layer, be it a whitespace or a char

            if crates_per_layer > number_of_stacks { // There are more crates than stacks, creating a new stack
                stacks.push(VecDeque::new());
                number_of_stacks += 1;
            }
            if stack_element.is_ascii_alphabetic() { // If the element is a char, push it to its correspondent stack
                stacks[crates_per_layer - 1].push_front(stack_element);
            }
        }
        crates_per_layer = 0; // New layer, setting variable to 0 again
    }
    stacks
}

// Moving crates between stacks 
fn move_crates(stacks: &mut [VecDeque<char>], crates_to_move: i32, movement_position: (usize, usize), same_order: bool) {

    let (from, to) = (movement_position.0, movement_position.1);

    for crate_number in 0..crates_to_move {

        if same_order { // The crates should stay in the same order when they move
            let element = stacks[from - 1]
                .pop_back()
                .expect("Couldn't pop a crate from the stack");
                // For each crate that moves, add it to index (len() - crate_number) to ensure the order doesn't change
                stacks[to - 1].insert(stacks[to - 1].len() - crate_number as usize, element);
        }
        else { // Each crate moves individually and the order can change
            let element = stacks[from - 1]
                .pop_back()
                .expect("Couldn't pop a crate from the stack");
                stacks[to - 1].push_back(element);
        }
    }
}

// Finding the message that corresponds to which crates will end up on top of the stacks
fn find_crates_at_top(stacks: &[VecDeque<char>]) -> String {
    let mut top_crates: String = "".to_owned();

    for stack in stacks {
        if !stack.is_empty() {
            let crate_label = stack.get(stack.len() - 1).unwrap(); // Getting char from the back position
            top_crates.push_str(&crate_label.to_string());
        }
    }
    top_crates
}

fn main() {
    println!(" --- Day 5: Supply Stacks --- \n");
    println!("\t --- Part One --- \n");

    let drawing = std::fs::read_to_string("../../input/day5.txt").expect("File or directory not found");

    let (stacks, instructions) = drawing.split_once("\n\n").expect("The file is not formatted correctly.");

    let mut stacks_of_crates_1: Vec<VecDeque<char>> = parse_crates(&stacks);
    let mut stacks_of_crates_2: Vec<VecDeque<char>> = stacks_of_crates_1.clone();

    // Parsing each instruction
    for instruction in instructions.lines() {
        
        // Getting number of crates that will move
        let (crates_to_move, positions) = instruction[5..]
            .split_once(" from ")
            .expect("Instruction formatted incorrectly.");
        let crates_to_move: i32 = crates_to_move.parse().expect("A numeric value is needed");

        // Getting movement position (from, to)
        let position = positions
            .split_once(" to ")
            .expect("Instruction formatted incorrectly.");

        // Parsing position from &str to usize
        let position: (usize, usize) = (
            position.0.parse()
            .expect("A numeric value is needed for the initial position"),
            position.1.parse()
            .expect("A numeric value is needed for the final position"));

        // Moving crates without keeping the order
        move_crates(stacks_of_crates_1.as_mut_slice(), crates_to_move, position, false);
        
        // Moving crates without changing the order
        move_crates(stacks_of_crates_2.as_mut_slice(), crates_to_move, position, true);
    }

    let message_1 = find_crates_at_top(&stacks_of_crates_1);
    println!("After the rearrangement procedure completes, the crates that end up on top of each stack form the string {message_1}.\n");
    
    println!("\t --- Part Two --- \n");

    let message_2 = find_crates_at_top(&stacks_of_crates_2);
    println!("Using CrateMover 9001, the crates that end up on top of each stack form the string {message_2}.");    
}