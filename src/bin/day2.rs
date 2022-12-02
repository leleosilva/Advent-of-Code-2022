#[derive(Copy, Clone)]
enum Decision {
    Undefined, // 0
    Rock, // 1
    Paper, // 2
    Scissors, // 3
}

impl From<Decision> for i32 { // Conversion from Decision to i32 based on discriminant
    fn from(item: Decision) -> Self {
        item as i32
    }
}

#[derive(Copy, Clone)]
enum Outcome {
    Win = 6,
    Draw = 3,
    Loss = 0,
    Undefined,
}

impl From<Outcome> for i32 { // Conversion from Outcome to i32 based on discriminant
    fn from(item: Outcome) -> Self {
        item as i32
    }
}


/* PART ONE FUNCTIONS */

// Returns if the current player chose Rock, Paper or Scissors
fn check_current_play(turn: &str) -> Decision {
    match turn {
        "A" | "X" => Decision::Rock,
        "B" | "Y" => Decision::Paper,
        "C" | "Z" => Decision::Scissors,
        _ => Decision::Undefined,
    }
}

// Checks what is the round outcome based on both players decisions
fn get_round_outcome(your: Decision, opponent: Decision) -> Outcome {
    match your {
        Decision::Paper => {
            match opponent {
                Decision::Paper => Outcome::Draw,
                Decision::Rock => Outcome::Win,
                Decision::Scissors => Outcome::Loss,
                _ => Outcome::Undefined,
            }
        }
        Decision::Rock => {
            match opponent {
                Decision::Paper => Outcome::Loss,
                Decision::Rock => Outcome::Draw,
                Decision::Scissors => Outcome::Win,
                _ => Outcome::Undefined,
            }

        },
        Decision::Scissors => {
            match opponent {
                Decision::Paper => Outcome::Win,
                Decision::Rock => Outcome::Loss,
                Decision::Scissors => Outcome::Draw,
                _ => Outcome::Undefined,
            }

        },
        _ => Outcome::Undefined,
    }
}


/* PART TWO FUNCTIONS */

// Decides if you need to win,lose or end the round in a draw based on 'X', 'Y' and 'Z'
fn check_needed_outcome(label: &str) -> Outcome {
    match label {
        "X" => Outcome::Loss,
        "Y" => Outcome::Draw,
        "Z" => Outcome::Win,
        _ => Outcome::Undefined,
    }
}

// Choose what to play based on the outcome needed
fn choose_next_action(outcome: Outcome, opponent: Decision) -> Decision {
    match outcome {
        Outcome::Win => {
            match opponent {
                Decision::Paper => Decision::Scissors,
                Decision::Rock => Decision::Paper,
                Decision::Scissors => Decision::Rock,
                Decision::Undefined => Decision::Undefined,

            }
        }
        Outcome::Draw => {
            match opponent {
                Decision::Paper => Decision::Paper,
                Decision::Rock => Decision::Rock,
                Decision::Scissors => Decision::Scissors,
                Decision::Undefined => Decision::Undefined,

            }
        }
        Outcome::Loss => {
            match opponent {
                Decision::Paper => Decision::Rock,
                Decision::Rock => Decision::Scissors,
                Decision::Scissors => Decision::Paper,
                Decision::Undefined => Decision::Undefined,

            }
        }
        _ => Decision::Undefined,
    }
}

fn main() {
    println!(" --- Day 2: Rock Paper Scissors --- \n");
    println!("\t --- Part One --- \n");

    let rounds = std::fs::read_to_string("../../input/day2.txt").expect("File or directory not found");

    let mut total_score_1 = 0;
    let mut total_score_2 = 0;
    for round in rounds.lines() {

        let mut round = round.split(" "); // Creating an iterator for the current round

        // Advancing the iterator and getting opponent's/your decision
        let opponent_label = round.next().unwrap();
        let your_label = round.next().unwrap();

        let opponent_turn = check_current_play(&opponent_label);
        let your_turn = check_current_play(&your_label);

        
        /* Code for part one */
        let outcome = get_round_outcome(your_turn.clone(), opponent_turn.clone()); // Getting outcome for the round

        // Calculating round score and adding to the total score (Part one)
        total_score_1 += i32::from(your_turn) + i32::from(outcome);


        /* Code for part two */
        let needed_outcome = check_needed_outcome(&your_label);
        let needed_action = choose_next_action(needed_outcome.clone(), opponent_turn.clone());

        // Calculating round score and adding to the total score (Part two)
        total_score_2 += i32::from(needed_action) + i32::from(needed_outcome);
    }

    println!("According to the strategy guide, the total score would be {}.\n", total_score_1);

    println!("\t --- Part Two --- \n");

    println!("Following the Elf's instructions for the second column, the total score would be {}.", total_score_2);
}