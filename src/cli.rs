use std::io::stdin;
use std::io::stdout;
use std::io::Write;
use colored::Colorize;
use crate::cute;
use crate::download::MIRRORS;

pub fn ui() {
    print!("There are several mirrors you can pick. \n[1] RBXCDN, [2] CFLY-CDN, [3] AWS, [4] Closest.\nPlease type the number here: ");
    stdout().flush().unwrap();
    let mut choice = String::new();
    stdin().read_line(&mut choice).expect("bruh");
    let choice: i8 = choice.trim().parse().expect("baka");
    
    match choice {
        1 => println!("Selected Mirrors: {} [√], {} [X], {} [X]", MIRRORS[0].green(), MIRRORS[1].red(), MIRRORS[2].red()),
        2 => println!("Selected Mirrors: {} [X], {} [√], {} [X]", MIRRORS[0].red(), MIRRORS[1].green(), MIRRORS[2].red()),
        3 => println!("Selected Mirrors: {} [X], {} [X], {} [√]", MIRRORS[0].red(), MIRRORS[1].red(), MIRRORS[2].green()),
        4 => println!("Picked recommended one."),
        _naidesu => println!("Invalid choice")
    } 

    if choice == 2 {
        cute();
    } else {
        return
    }
}