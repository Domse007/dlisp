pub struct Config {
    pub eval_input: bool,
    pub ignore_lisp_input: bool,
    pub file: Option<String>,
}

impl Config {
    pub fn new(args: Vec<String>) -> Self {
        let mut eval_input = false;
        let mut ignore_lisp_input = false;
        let mut file = None;
        let mut args = args.iter();
        while let Some(arg) = args.next() {
            match arg.as_str() {
                "-i" => ignore_lisp_input = true,
                "-f" => file = Some(args.next().unwrap().to_string()),
                "-e" => eval_input = true,
                _ => {}
            }
        }
        Self {
            eval_input,
            ignore_lisp_input,
            file,
        }
    }
}
