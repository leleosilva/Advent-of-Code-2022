use std::{collections::HashSet};

const LOWERCASE_ASCII: i32 = 97;
const UPPERCASE_ASCII: i32 = 65;

const ALPHABET: i32 = 26;

const LOWERCASE_PRIORITY: i32 = LOWERCASE_ASCII - 1;
const UPPERCASE_PRIORITY: i32 = (UPPERCASE_ASCII - 1) - ALPHABET;

/* -- VERSION 1 OF PART ONE SOLUTION (commented code below) --
 * Because inserting to a HashSet is an expensive operation, a Hashset is created from the shorter
 * string (it doesn't matter in this exercise, both strings are the same size) and its elements are
 * compared to an Iterator from the chars of the longer string */

/* fn find_intersection_element(str1: &str, str2: &str) -> char {
    
    let (shorter_str, longer_str) = if str1.len() < str2.len() {
        (str1, str2)
    } else {
        (str2, str1)
    };

    let chars_set: HashSet<char> = shorter_str.chars().collect();

    longer_str.chars().find(|c| chars_set.contains(&c)).expect("The strings don't have any character in common.")
} */

// Finds the common element in both strings.
// In this exercise, it's guaranteed that only one char is common for both strings
fn find_intersection_element(str1: &str, str2: &str) -> char {
    
    let str1_set: HashSet<char> = str1.chars().collect();
    let str2_set: HashSet<char> = str2.chars().collect();

    *str1_set.intersection(&str2_set).next().expect("The strings don't have any character in common.")   
}

// Calculates the character priority depending on if it is uppercase or lowercase
fn calculate_char_priority(char: char) -> i32 {
    if char.is_lowercase() {
        (char as i32) - LOWERCASE_PRIORITY
    }
    else if char.is_ascii_uppercase() {
        (char as i32) - UPPERCASE_PRIORITY
    } else {
        0
    }
}

// Finds the group's badge char and returns its priority
fn find_group_badge(str1: &str, str2: &str, str3: &str) -> i32 {
    let str1_set: HashSet<char> = str1.chars().collect();
    let str2_set: HashSet<char> = str2.chars().collect();

    let mut priority = 0;
    for common_char in str1_set.intersection(&str2_set) {
        if str3.contains(common_char.clone()) {
            priority = calculate_char_priority(*common_char);
        }
    }
    priority
}

fn main() {
    println!(" --- Day 3: Rucksack Reorganization --- \n");
    println!("\t --- Part One --- \n");

    let rucksack = std::fs::read_to_string("../../input/day3.txt").expect("File or directory not found");

    let mut total_priority = 0;
    let mut group_priority = 0;
    let mut group_items: Vec<String> = Vec::with_capacity(3);
    for item in rucksack.lines() {

        // Splitting item in its two compartments
        let (compartment1, compartment2) = item.split_at(item.len() / 2);

        let common_char = find_intersection_element(compartment1, compartment2);

        total_priority += calculate_char_priority(common_char);

        /* Code for part two */
        group_items.push(String::from(item));
        if group_items.capacity() == group_items.len() { // Vec has reached its capacity, find group's badge and clear vec
            group_priority += find_group_badge(&group_items[0], &group_items[1], &group_items[2]);
            group_items.clear();
        }
    }
    println!("The sum of the priorities is {}.\n", total_priority);

    println!("\t --- Part Two --- \n");

    println!("The sum of the priorities of each three-Elf group is {}.", group_priority);

}