mod cli;

use colored::Colorize;

fn main() {
    println!("Hello, world!");
    println!("{}this crate is still under development!", "Note: ".red());

    foo(&String::from("hello world"));
}

fn foo(val: &String) {
    println!("{}", val)
}
