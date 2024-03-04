pub mod cli;
pub use std::io;
use crate::cli::ui;
use std::{thread, time};
mod download;
use std::process::Command;

fn main() {
    ui();
}

fn cute() {
    println!("Downloading Roblox Installer..");
    let _jej2 = time::Duration::from_millis(240);
        print!("Warning: Roblox Installer Already Found! Running...\n");
        let jej = time::Duration::from_millis(800);
        thread::sleep(jej);
        Command::new("wine").arg("/home/breaktrace/livelylive/RobloxPlayerInstaller.exe").output().expect("Failed to execute command");
}
