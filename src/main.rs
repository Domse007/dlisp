#![allow(dead_code)]
use std::env;

use ast::ast;
use config::Config;
use evaluator::eval;
use lispobject::LispObject;
use objectmanager::Manager;

mod ast;
mod config;
mod error;
mod evaluator;
mod functions;
mod lispobject;
mod objectmanager;

fn main() {
    let args = env::args().skip(1).collect::<Vec<String>>();
    let (cmds, lisp) = sort_input(&args);
    let config = Config::new(cmds);
    // convert the strings to lispobjects
    let lisp = LispObject::new_list(&lisp, config.eval_input);
    // Instantiate the objectmanager
    let mut manager = Manager::default();
    // Convert the args to an instruction, that the interpreter can understand
    let instr = LispObject::list(&[
        LispObject::symbol("set"),
        LispObject::symbol("argv").move_quoted(),
        lisp.move_quoted(),
    ]);

    let res = eval(instr, &mut manager);
    if res.is_err() {
        eprintln!("[ERROR] {}", res.unwrap_err());
        std::process::exit(-1);
    }

    if let Some(code) = config.get_file_string() {
        let ast = ast(code.as_str()).unwrap();
        for block in ast {
            eval(block, &mut manager).unwrap();
        }
    }
}

fn sort_input(args: &[String]) -> (Vec<String>, Vec<String>) {
    let mut cmds = Vec::new();
    let mut lisp = Vec::new();
    let mut args = args.iter();
    while let Some(arg) = args.next() {
        if arg == "-f" {
            cmds.push(arg.to_string());
            cmds.push(args.next().unwrap().to_string());
        } else if arg.starts_with('-') {
            cmds.push(arg.to_string());
        } else {
            lisp.push(arg.to_string());
        }
    }
    (cmds, lisp)
}

#[test]
fn test_sort_input() {
    let test = ["-i", "-f", "hello", "(test", "this)", "12"]
        .iter()
        .map(|e| e.to_string())
        .collect::<Vec<String>>();
    let exp = ["-i", "-f", "hello"]
        .iter()
        .map(|e| e.to_string())
        .collect::<Vec<String>>();
    let other = ["(test", "this)", "12"]
        .iter()
        .map(|e| e.to_string())
        .collect::<Vec<String>>();
    let (s1, s2) = sort_input(&test);
    assert_eq!(s1, exp);
    assert_eq!(s2, other);
}
