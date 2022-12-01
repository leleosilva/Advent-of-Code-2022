use std::{fs, collections::BinaryHeap};

fn main() {

    println!("--- Day 1: Calorie Counting --- \n");
    println!("\t --- Part One --- \n");

    // Reading inventory of calories and splitting individual values
    let inventory = fs::read_to_string("../../input/day1.txt").expect("File or directory not found");
    let inventory = inventory.split("\n");

    let mut elves_calories: Vec<i32> = Vec::new();
    
    let mut total_calories = 0;
    for calorie in inventory {
        if calorie.is_empty() { // Empty line, elf changes
            elves_calories.push(total_calories);
            total_calories = 0;
        }
        else {
            total_calories += calorie.parse::<i32>().unwrap();
        }
    }
    
    let most_calories = elves_calories.iter().max().expect("Empty inventory!");
    println!("The Elf carrying the most calories carries {} calories.\n", most_calories);


    println!("\t --- Part Two --- \n");

    // Using Max Heap to find the top three Elves carrying the most calories
    let mut calories_heap: BinaryHeap<i32> = BinaryHeap::from(elves_calories);

    let mut top_three_calories = 0;
    for _ in 0..3 {
        top_three_calories += calories_heap.pop().expect("Empty heap, value not found!"); // Removing and returning the greatest item from the heap
    }

    println!("The top three Elves carrying the most calories carry {} calories.", top_three_calories);
}