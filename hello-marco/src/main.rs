//a command line tool to play marco polo game//

use std::io::{self, Write};

fn main() {
    println!("🎮  Marco Polo Game!");
    println!("Type 'marco' to play or 'exit' to quit.");

    loop {
        print!("> "); // Prompt
        io::stdout().flush().unwrap(); // Force display

        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read input");

        match input.trim().to_lowercase().as_str() {
            "marco" => println!("Polo!"),
            "exit" => {
                println!("Goodbye! 👋");
                break;
            }
            _ => println!("❌  Say 'marco' or 'exit'."),
        }
    }
}
