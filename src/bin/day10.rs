pub const DISPLAY_WIDTH: usize = 40;
pub const DISPLAY_HEIGHT: usize = 6;

enum Instruction {
    Addx(i32),
    Noop,
    Undefined,
}

struct Cpu {
    x: i32, // Register
    cycle: i32,
    current_instruction: Instruction,
    total_signal_strength: i32,
    display: [char; DISPLAY_WIDTH * DISPLAY_HEIGHT],

    /* Counter that stores how many interesting cycles have been evaluated already
     * Interesting cycles have the structure [20 + (current cycle * counter)] */
    interesting_cycles_counter: i32,
}

impl Cpu {

    // Creates a new CPU instance
    fn new() -> Cpu {
        Cpu {
            x: 1,
            cycle: 0,
            current_instruction: Instruction::Undefined,
            total_signal_strength: 0,
            interesting_cycles_counter: 0,
            display: ['.'; DISPLAY_WIDTH * DISPLAY_HEIGHT],
        }
    }

    // Parsing instruction from string to enum Instruction
    fn parse_instruction(&mut self, input: &str) {
        if input.starts_with("noop") {
            self.current_instruction = Instruction::Noop;
        } else {
            let (_, value) = input
                .split_once(' ')
                .expect("addx instruction should be written as 'addx [value]'");

            let value: i32 = value.parse().expect("addx instruction requires a numeric value");
            self.current_instruction = Instruction::Addx(value);
        }
    }

    /* Increments the cycle.
     * Before that, the draw() method is called because the CRT draws a single pixel during each cycle */
    fn increment_cycle(&mut self) {
        self.draw_display();
        self.cycle += 1;
    }

    // Runs one CPU cycle
    fn tick(&mut self) {
        match self.current_instruction {
            Instruction::Addx(value) => { // Increments the cycle by 2 and adds a value to the register X
                self.increment_cycle();
                self.determine_signal_strength(); // Calculating the signal strength between each cycle increment
                self.increment_cycle();
                self.x += value;
            },
            Instruction::Noop => {
                self.increment_cycle() // noop instruction only increments the cycle counter
            },
            Instruction::Undefined => (),
        }
        self.determine_signal_strength(); // Cycle has incremented; calculating signal strength
    }

    // Determines signal strength if the current cycle is an interesting one, and adds the signal to the total value
    fn determine_signal_strength(&mut self) {
        let interesting_cycles = 20 + 40 * self.interesting_cycles_counter;
        match self.cycle + 1 == interesting_cycles {
            true => {
                self.total_signal_strength += self.x * (self.cycle + 1);
                self.interesting_cycles_counter += 1; // Interesting cycle has been evaluated; incrementing counter
            },
            false => (), // The cycle is not interesting; doesn't do anything
        }
    }

    // Draws a single pixel based on the cycle number
    fn draw_display(&mut self) {
        
        // Getting CRT range where the sprite is located
        let sprite_idx_min = self.x - 1;
        let sprite_idx_max = self.x + 1;

        // Gets current position from 1D to 2D based on current cycle
        let display_position = self.cycle % (DISPLAY_WIDTH as i32);

        // Checks if position is in the sprite range
        if (sprite_idx_min..sprite_idx_max + 1).contains(&display_position) {
            self.display[self.cycle as usize] = '#';
        }
    }

    /* Prints the display on screen
     *
     * 'display' is a 1D array. So, to see it as a 2D array, its positions are given by
     * 'width_idx + (height_idx * DISPLAY_WIDTH)' */
    fn show_display(&mut self) {
        for height_idx in 0..DISPLAY_HEIGHT {
            for width_idx in 0..DISPLAY_WIDTH {
                print!("{}", self.display[width_idx + (height_idx * DISPLAY_WIDTH)]);   
            }
            println!();
        }
    }
}

fn main() {
    println!(" --- Day 10: Cathode-Ray Tube --- \n");
    println!("\t --- Part One --- \n");

    let instructions = include_str!("../../input/day10.txt");
    let mut cpu: Cpu = Cpu::new();

    for instruction in instructions.lines() {
        cpu.parse_instruction(instruction);
        cpu.tick();
    }
    println!("The sum of the interesting signal strengths is {}.\n", cpu.total_signal_strength);

    println!("\t --- Part Two --- \n");

    cpu.show_display();
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_signal_strength() {
        let mut cpu: Cpu = Cpu::new();
        
        for instruction in TEST_INPUT.lines() {
            cpu.parse_instruction(instruction.trim());
            cpu.tick();
        }
        assert_eq!(cpu.total_signal_strength, 13140); 
    }

    #[test]
    fn test_crt() {
        let mut cpu: Cpu = Cpu::new();

        let expected_crt =
        "##..##..##..##..##..##..##..##..##..##..
        ###...###...###...###...###...###...###.
        ####....####....####....####....####....
        #####.....#####.....#####.....#####.....
        ######......######......######......####
        #######.......#######.......#######.....";
        let expected_crt = expected_crt.replace("\n        ", "");
        
        for instruction in TEST_INPUT.lines() {
            cpu.parse_instruction(instruction.trim());
            cpu.tick();
        }
        assert_eq!(cpu.display.iter().collect::<String>().trim(), expected_crt); 
    }

    const TEST_INPUT: &str =
        "addx 15
        addx -11
        addx 6
        addx -3
        addx 5
        addx -1
        addx -8
        addx 13
        addx 4
        noop
        addx -1
        addx 5
        addx -1
        addx 5
        addx -1
        addx 5
        addx -1
        addx 5
        addx -1
        addx -35
        addx 1
        addx 24
        addx -19
        addx 1
        addx 16
        addx -11
        noop
        noop
        addx 21
        addx -15
        noop
        noop
        addx -3
        addx 9
        addx 1
        addx -3
        addx 8
        addx 1
        addx 5
        noop
        noop
        noop
        noop
        noop
        addx -36
        noop
        addx 1
        addx 7
        noop
        noop
        noop
        addx 2
        addx 6
        noop
        noop
        noop
        noop
        noop
        addx 1
        noop
        noop
        addx 7
        addx 1
        noop
        addx -13
        addx 13
        addx 7
        noop
        addx 1
        addx -33
        noop
        noop
        noop
        addx 2
        noop
        noop
        noop
        addx 8
        noop
        addx -1
        addx 2
        addx 1
        noop
        addx 17
        addx -9
        addx 1
        addx 1
        addx -3
        addx 11
        noop
        noop
        addx 1
        noop
        addx 1
        noop
        noop
        addx -13
        addx -19
        addx 1
        addx 3
        addx 26
        addx -30
        addx 12
        addx -1
        addx 3
        addx 1
        noop
        noop
        noop
        addx -9
        addx 18
        addx 1
        addx 2
        noop
        noop
        addx 9
        noop
        noop
        noop
        addx -1
        addx 2
        addx -37
        addx 1
        addx 3
        noop
        addx 15
        addx -21
        addx 22
        addx -6
        addx 1
        noop
        addx 2
        addx 1
        noop
        addx -10
        noop
        noop
        addx 20
        addx 1
        addx 2
        addx 2
        addx -6
        addx -11
        noop
        noop
        noop";
}