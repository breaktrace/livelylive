use crate::download::MIRRORS;
use std::io::stdin;
use std::io::stdout;
use std::io::Write;
use colored::Colorize;

pub fn ui() {
    print!("There are several mirrors you can pick. \n[1] RBXCDN, [2] CFLY-CDN, [3] AWS, [4] Recommended.\nPlease type the number here: ");
    stdout().flush().unwrap();
    let mut choice = String::new();
    stdin().read_line(&mut choice).expect("bruh");
    let choice: i8 = choice.trim().parse().expect("baka");
    
    match choice {
        1 => println!("Selected Mirrors: {}, {} [X], {} [X]", MIRRORS[0].green(), MIRRORS[1].red(), MIRRORS[2].red()),
        2 => println!("Sisted Mirrors: {} [X], {}, {} [X]", MIRRORS[0].red(), MIRRORS[1].green(), MIRRORS[2].red()),
        3 => println!("Sisted Mirrors: {} [X], {} [X], {}", MIRRORS[0].red(), MIRRORS[1].red(), MIRRORS[2].green()),
        4 => println!("Picked recommended one."),
        _naidesu => println!("Invalid choice")
    }
}