use std::{
    collections::HashMap,
    env::{self, ArgsOs, args},
};

/// cli struct to receive env args
/// format: command [option] [args]
/// usage: anon [command] [options]
pub struct Cli {
    pub map: HashMap<String, Command>,
}

impl Cli {
    pub fn new() -> Self {
        let mut map = HashMap::with_capacity(10);
        map.insert(String::from("help"), Command::help());
        Self { map }
    }

    pub fn run(&self, args: Vec<String>) {
        match args.first().map(|x| x.as_str()) {
            Some("help") | None => {
                self.map.get("help").unwrap().process(&vec![]);
            }
            Some(s) => {
                eprint!("Error: Unknown command: {}", s);
            }
        }
    }
}

pub struct Command {
    pub name: String,
    pub option: Vec<String>,
    pub call_back: Box<dyn Fn(&Vec<String>) -> ()>,
}

impl Command {
    pub fn process(&self, args: &Vec<String>) {
        (self.call_back)(&args)
    }

    pub fn help() -> Self {
        Self {
            name: String::from("help"),
            option: vec![],
            call_back: Box::new(|_: &Vec<String>| {
                print!(include_str!("../help_assets/main_help.txt"))
            }),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_cli_help() {
        let cli = Cli::new();
        cli.run(vec!["help".into()]);
    }
}
