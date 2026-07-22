use std::io;
use rand::Rng;

fn main() {
    'outer: loop {
        println!("Choose a die:");

        println!("d2/Coin, d4, d6, d8, d10, d12, d20, d100");
        println!("Type 'exit' to quit.");

        let mut die_choice = String::new();
        io::stdin()
            .read_line(&mut die_choice)
            .expect("Failed to read line");

        let trimmed_choice = die_choice.trim().to_lowercase();

        // Check for 'exit' command
        if trimmed_choice == "exit" {
            println!("Bye!");
            break 'outer;
        }

        // Parsing the die type from various inputs
        let max_sides: u32 = match trimmed_choice.trim_start_matches('d').parse() {
            Ok(num) if num > 0 => num,
            _ => {
                println!("Invalid choice, please enter something like 'd6' or '20'");
                // Re-prompt selection of die type
                continue;
            }
        };

        println!("Rolling a d{max_sides}");

        // Inner rolling scope
        loop {
            let mut input = String::new();
            println!("Press enter to roll!");
            
            io::stdin()
                .read_line(&mut input)
                .expect("Failed to read line");

            let command = input.trim().to_lowercase();

            // Direct/Menu exit command

            if command == "exit" {
                println!("Bye!");
                break 'outer; // Directly exits the program
            } else if command == "change" {
                break; // Only the inner loop, like pressing 'menu'
            }

            // Gambling time!
            let die_roll = rand::thread_rng().gen_range(1..=max_sides);
            println!("You rolled: {die_roll}");
        }
    }
}