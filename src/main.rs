mod full_calculator;
use std::io;

fn main() {
    let mut choice: i32 = -1;
    while choice != 0 {
        println!("{} Please choose an option. Enter 0 to end the program", choice);
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read input");
        choice = input.trim().parse::<i32>().unwrap_or(-1);

        match choice {
            0 => {
                println!("Exiting the program.");
            },
            1 => {
                println!("You chose the addition function");
                full_calculator::addition();
            },
            2 => {
                println!("You chose the subtraction function");
                full_calculator::subtraction();
            },
            3 => {
                println!("You chose the multiplication function");
                full_calculator::multiplication();
            },
            4 => {
                println!("You chose the division function");
                full_calculator::division();
            },
            _ => {
                println!("Invalid Choice");
            }
        }
    }
}
