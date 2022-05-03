use std::{fs, process};

const HELP_MSG: &'static str = "dlisp [FLAGS] [LISP]
    -f FILE     Eval specified file.
    -v          Print version.
    -h          Show this help message.";

pub struct Config {
    pub eval_input: bool,
    pub file: Option<String>,
}

impl Config {
    pub fn new(args: Vec<String>) -> Self {
        let eval_input = false;
        let mut file = None;
        let mut args = args.iter();
        while let Some(arg) = args.next() {
            match arg.as_str() {
                "-v" | "--version" => {
                    println!(
                        "dlisp version {}\nThis project is licensed under GPLv3.",
                        env!("CARGO_PKG_VERSION")
                    );
                    process::exit(0);
                }
                "-h" | "--help" => {
                    println!("{}", HELP_MSG);
                    process::exit(0);
                }
                "-f" => file = Some(args.next().unwrap().to_string()),
                _ => {}
            }
        }
        Self { eval_input, file }
    }

    pub fn get_file_string(&self) -> Option<String> {
        if self.file.is_some() {
            return Some(fs::read_to_string(self.file.as_ref().unwrap()).unwrap());
        }
        None
    }
}
