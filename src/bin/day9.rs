use std::collections::HashSet;

struct Head {
    position: (i32, i32),
}

#[derive(Clone)] // Necessary to create a vec! using Tail::new()
struct Tail {
    position: (i32, i32),
    visited_positions: HashSet<(i32, i32)>,
}

impl Head {
    fn new() -> Head { // Creates new instance of Head
        Head {
            position: (0, 0),
        }
    }

    // Moves head based on direction
    fn move_head(&mut self, direction: &str) {
        let (x, y) = match direction {
            "U" => (0, 1),
            "D" => (0, -1),
            "L" => (-1, 0),
            "R" => (1, 0),
            _ => (0, 0),
        };
        self.position = (self.position.0 + x, self.position.1 + y); 
    }
}

impl Tail {

    fn new() -> Tail { // Creates new instance of Tail
        Tail {
            position: (0, 0),
            visited_positions: HashSet::new(), // HashSet that stores each new position visited by the tail
        }
    }

    // Checking if moving the tail if necessary
    fn check_if_tail_moves(&mut self, head_position: (i32, i32)) {
        
        let x = head_position.0 - self.position.0;
        let y = head_position.1 - self.position.1;

        if head_position.1 == self.position.1 { // Y coordinate hasn't changed; X will move
            self.move_x(x);
        }
        else if head_position.0 == self.position.0 { // X coordinate hasn't changed; Y will move
            self.move_y(y);
        }
        else { // X and Y coordinates aren't the same; that means the head moved diagonally
            self.move_diagonally(x, y);
        }
        self.visited_positions.insert(self.position); // Inserting new tail position on HashSet if tail has moved to a new position
    }

    // Moves tail on X axis
    fn move_x(&mut self, x: i32) {
        if x.abs() == 2 {
            self.check_x_value(x);
        }
    }

    // Moves tail on Y axis
    fn move_y(&mut self, y: i32) {
        if y.abs() == 2 {
            self.check_y_value(y);
        }
    }

    // Moves the tail on both X and Y axis if the head has moved on either axis
    fn move_diagonally(&mut self, x: i32, y: i32) {
        if x.abs() == 2 || y.abs() == 2 {
            self.check_x_value(x);
            self.check_y_value(y);
        }
    }

    // If x is positive, increment tail position on X axis. Otherwise, decrement it.
    fn check_x_value(&mut self, x: i32) {
        if x > 0 { self.position.0 += 1; } // Tail moves right
        else { self.position.0 -= 1; } // Tail moves left
    }

    // If y is positive, increment tail position on Y axis. Otherwise, decrement it.
    fn check_y_value(&mut self, y: i32) {
        if y > 0 { self.position.1 += 1; } // Tail moves up
        else { self.position.1 -= 1; } // Tail moves down
    }
}

fn main() {
    println!(" --- Day 9: Rope Bridge --- \n");
    println!("\t --- Part One --- \n");

    let movements = include_str!("../../input/day9.txt");

    let mut head = Head::new();
    let mut tail = Tail::new();

    for movement in movements.lines() {
        let (direction, value) = movement.split_once(' ').expect("Direction formatted incorrectly.");
        let times_to_move: i32 = value.parse().expect("Direction should have a numeric value.");

        for _ in 0..times_to_move {
            head.move_head(direction);
            tail.check_if_tail_moves(head.position);
        }
    }
    println!("The tail of the rope visits {} position(s) at least once.\n", tail.visited_positions.len());

    println!("\t --- Part Two --- \n");

    let mut head = Head::new();
    let mut tails = vec![Tail::new(); 9]; // Creating vector of Tails for all the last 9 knots
    
    for movement in movements.lines() {
        let (direction, value) = movement.split_once(' ').expect("Direction formatted incorrectly.");
        let times_to_move: i32 = value.parse().expect("Direction should have a numeric value.");

        for _ in 0..times_to_move {

            // Moving the head and the next knot
            head.move_head(direction);
            tails[0].check_if_tail_moves(head.position);

            // For all the other knots, they will move based on their previous knot
            for tail_idx in 1..9 {
                let position = tails[tail_idx - 1].position; // Getting previous knot position
                tails[tail_idx].check_if_tail_moves(position);
            }
        }
    }
    println!("On a larger rope, the tail of the rope visits {} position(s) at least once.", tails[tails.len() - 1].visited_positions.len());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_two_knots() {
        let mut head = Head::new();
        let mut tail = Tail::new();

        for movement in INPUT_TWO_KNOTS.lines() {
            let (direction, value) = movement.trim().split_once(' ').expect("Direction formatted incorrectly.");
            let times_to_move: i32 = value.parse().expect("Direction should have a numeric value.");

            for _ in 0..times_to_move {
                head.move_head(direction);
                tail.check_if_tail_moves(head.position);
            }
        }
        assert_eq!(tail.visited_positions.len(), 13);
    }

    #[test]
    fn test_ten_knots() {
        let mut head = Head::new();
        let mut tails = vec![Tail::new(); 9]; // Creating vector of Tails for all the last 9 knots
        
        for movement in INPUT_TEN_KNOTS.lines() {
            let (direction, value) = movement.trim().split_once(' ').expect("Direction formatted incorrectly.");
            let times_to_move: i32 = value.parse().expect("Direction should have a numeric value.");
    
            for _ in 0..times_to_move {
    
                // Moving the head and the next knot
                head.move_head(direction);
                tails[0].check_if_tail_moves(head.position);
    
                // For all the other knots, they will move based on their previous knot
                for tail_idx in 1..9 {
                    let position = tails[tail_idx - 1].position; // Getting previous knot position
                    tails[tail_idx].check_if_tail_moves(position);
                }
            }
        }
        assert_eq!(tails[tails.len() - 1].visited_positions.len(), 36);
    }

    const INPUT_TWO_KNOTS: &str =
    "R 4
    U 4
    L 3
    D 1
    R 4
    D 1
    L 5
    R 2";

    const INPUT_TEN_KNOTS: &str =
    "R 5
    U 8
    L 8
    D 3
    R 17
    D 10
    L 25
    U 20";
}