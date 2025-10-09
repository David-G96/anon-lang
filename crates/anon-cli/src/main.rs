mod cli;

use colored::Colorize;

fn main() {
    println!("Hello, world!");
    println!(
        "{}{}",
        "Note: ".red(),
        "this crate is still under development!"
    );
}
