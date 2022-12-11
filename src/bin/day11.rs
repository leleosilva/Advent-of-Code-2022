use std::{collections::VecDeque};

struct Monkey {
    items: VecDeque<u64>,
    operation: fn(u64) -> u64,
    test: u64,
    monkey_if_true: u64,
    monkey_if_false: u64,
    items_inspected: u64,
}

impl Monkey {
    fn new() -> Monkey { // Creates a new instance of Monkey
        Monkey {
            items: VecDeque::new(),
            operation: |_| -> u64 { 0 },
            test: 1,
            monkey_if_true: 0,
            monkey_if_false: 0,
            items_inspected: 0,
        }
    }
}

// Parsing each Monkey into a vector of Monkeys
fn parse_monkey_data(input: &str) -> Vec<Monkey> {
    
    let mut monkeys = Vec::<Monkey>::new();

    let input = input.split("\n\n"); // Splitting empty lines

    input.enumerate().for_each(|(idx, monkey)| { // Parsing lines for each Monkey
        monkeys.push(Monkey::new()); // For each new index, push a new Monkey into the vector

        for line in monkey.lines() {
            let (attribute, _) = line.split_once(":").expect("The line doesn't have ':'");
            match attribute {
                "  Starting items" => parse_starting_items(line, &mut monkeys[idx]),
                "  Operation" => parse_operations(line, &mut monkeys[idx]),
                "  Test" => parse_tests(line, &mut monkeys[idx]),
                "    If true" => parse_monkey_to_throw(line, &mut monkeys[idx], true),
                "    If false" => parse_monkey_to_throw(line, &mut monkeys[idx], false),
                _ => (),
            }
        }
    });
    monkeys
}

// Parsing items into vector of items
fn parse_starting_items(input: &str, monkey: &mut Monkey) {
    let items = input.strip_prefix("  Starting items: ").expect("Items line formatted incorrectly");
    let items = items.split(", ").map(|x| x.parse::<u64>().unwrap());
    monkey.items = VecDeque::from_iter(items); // Creates VecDeque from iteration of items
}

// Parsing operations using closures for each Monkey
fn parse_operations(input: &str, monkey: &mut Monkey) {
    let (_, op) = input.split_once(": ").expect("Operation formatted incorrectly");
    
    monkey.operation =
    match op {
        "new = old * 3" => |old: u64| -> u64{ old * 3 },
        "new = old + 8" => |old: u64| -> u64{ old + 8 },
        "new = old * old" => |old: u64| -> u64{ old * old },
        "new = old + 2" => |old: u64| -> u64{ old + 2 },
        "new = old + 3" => |old: u64| -> u64{ old + 3 },
        "new = old * 17" => |old: u64| -> u64{ old * 17 },
        "new = old + 6" => |old: u64| -> u64{ old + 6 },
        "new = old + 1" => |old: u64| -> u64{ old + 1 },
        "new = old * 19" => |old: u64| -> u64{ old * 19 },
        _ => |_: u64| -> u64{ 0},
    };
}

// Parsing tests for each Monkey
fn parse_tests(input: &str, monkey: &mut Monkey) {
    monkey.test = input
        .split(' ').last().expect("Last word in the Test should be a numeric value") // Getting last word
        .parse().expect("Numeric value expected"); // Parsing word to u64
}

// Parsing monkeys that will receive items thrown by the current Monkey
fn parse_monkey_to_throw(input: &str, monkey: &mut Monkey, monkey_bool: bool) {
    if monkey_bool { // If test is true
        monkey.monkey_if_true = input
            .split(' ').last().expect("Last word in the 'If true' line should be a numeric value")
            .parse().expect("Numeric value expected");
    }
    else { // If test is false
        monkey.monkey_if_false = input
            .split(' ').last().expect("Last word in the 'If false' line should be a numeric value")
            .parse().expect("Numeric value expected");
    }
}

// Running 'n' monkey rounds
fn monkey_rounds(monkeys: &mut Vec<Monkey>, n: u64, relief: bool) -> u64 {
    
    /* To avoid having very large worry levels, we can use modulo arithmetic to lower them,
     * using lowest column multiple (LCM) of all their divisors (test numbers).
     *
     * But, because each worry level is divisible by a different number (test numbers) and all of them
     * are prime (they are coprime relative to one another), we can multiply all of them to get the LCM.
     * 
     * Question: doesn't our worry operations affect the modulo operation?
     * 
     * In our case, the worry operation only multiplies or adds to the worry level.
     * And modulo congruence is preserved for any multiplication or addition operations.
     * 
     * However, division does not preserve modulo congruence. Therefore, if 'relief' is true it will
     * affect our result.
     * 
     * Detailed explanation: https://aoc.just2good.co.uk/2022/11#part-2 */
    let modulus: u64 = monkeys.iter().map(|m| m.test).product(); // Multiplying all test values

    for _ in 0..n { // Runs the loop for 'n' rounds
        for idx in 0..monkeys.len() { // Runs each round for each Monkey

            while monkeys[idx].items.len() > 0 { // Inspect each item from vector of items of each Monkey
                let item = monkeys[idx].items.pop_front().unwrap();
                let mut worry_lvl = (monkeys[idx].operation)(item);
                
                if relief { // Divides worry level by three if you are relieved
                    worry_lvl = worry_lvl / 3;
                }
                
                if worry_lvl % monkeys[idx].test == 0 { // Test is true, throw to true_idx monkey
                    let true_monkey_idx = monkeys[idx].monkey_if_true as usize;
                    monkeys[true_monkey_idx].items.push_back(worry_lvl % modulus);
                }
                else { // Test is false, throw to false_idx monkey
                    let false_monkey_idx = monkeys[idx].monkey_if_false as usize;
                    monkeys[false_monkey_idx].items.push_back(worry_lvl % modulus);
                }
                monkeys[idx].items_inspected += 1; // Increments the number of items inspected for each Monkey
            }
        }
    }
    // Creates vector of number of items inspected, and sort it in descending order
    let mut inspections = monkeys.iter().map(|m| m.items_inspected).collect::<Vec<u64>>();
    inspections.sort_by(|a, b| b.cmp(a));

    inspections[0] * inspections[1] // Returns product of the two most active Monkeys
}

fn main() {
    println!(" --- Day 11: Monkey in the Middle --- \n");
    println!("\t --- Part One --- \n");

    let monkey_data = include_str!("../../input/day11.txt");

    let mut monkeys = parse_monkey_data(monkey_data);
    let monkey_business = monkey_rounds(&mut monkeys, 20, true);
    println!("The level of monkey business after 20 rounds is {}.\n", monkey_business);

    println!("\t --- Part Two --- \n");

    let mut monkeys = parse_monkey_data(monkey_data);
    let monkey_business = monkey_rounds(&mut monkeys, 10000, false);
    println!("The level of monkey business after 10000 rounds is {}.\n", monkey_business);
}

#[cfg(test)]
mod tests {
    use super::*;

    /* Auxiliar function that removes prefix from string if it exists.
     *
     * If the prefix exists, change string and return it.
     * If it doesn't exist, return unchanged string */
    fn remove_prefix<'a>(s: &'a str, prefix: &str) -> &'a str {
        match s.strip_prefix(prefix) {
            Some(s) => s,
            None => s
        }
    }
    
    #[test]
    fn test_rounds_with_relief() {
        let input = INPUT_DATA
            .lines().skip(1) // Skipping the first line (it's just a '\n')

            // Remove prefix and add \n at the end of line (because lines() consume the existent '\n' for each line)
            .map(|l| remove_prefix(l, "  ").to_string() + &"\n".to_owned())
            .collect::<String>();
            
        let mut monkeys = parse_monkey_data(&input);
        assert_eq!(monkey_rounds(&mut monkeys, 20, true), 10605);
    }

    #[test]
    fn test_rounds_without_relief() {
        let input = INPUT_DATA
            .lines().skip(1) // Skipping the first line (it's just a '\n')

            // Remove prefix and add \n at the end of line (because lines() consume the existent '\n' for each line)
            .map(|l| remove_prefix(l, "  ").to_string() + &"\n".to_owned())
            .collect::<String>();
        println!("{}", input);
            
        let mut monkeys = parse_monkey_data(&input);
        assert_eq!(monkey_rounds(&mut monkeys, 10000, false), 2713310158);
    }

    const INPUT_DATA: &str =
    "
  Monkey 0:
    Starting items: 79, 98
    Operation: new = old * 19
    Test: divisible by 23
      If true: throw to monkey 2
      If false: throw to monkey 3
  
  Monkey 1:
    Starting items: 54, 65, 75, 74
    Operation: new = old + 6
    Test: divisible by 19
      If true: throw to monkey 2
      If false: throw to monkey 0
  
  Monkey 2:
    Starting items: 79, 60, 97
    Operation: new = old * old
    Test: divisible by 13
      If true: throw to monkey 1
      If false: throw to monkey 3
  
  Monkey 3:
    Starting items: 74
    Operation: new = old + 3
    Test: divisible by 17
      If true: throw to monkey 0
      If false: throw to monkey 1";
}