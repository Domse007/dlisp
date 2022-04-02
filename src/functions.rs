use crate::lispobject::{LispObject, LispType};

pub fn call_builtin(fn_name: &str, args: &[LispObject]) -> Result<LispObject, &'static str> {
    match fn_name {
        "cons" => cons(args),
        "list" => list(args),
        "add" | "+" => add(args),
        _ => Err("No built in function with that name."),
    }
}

pub fn cons(args: &[LispObject]) -> Result<LispObject, &'static str> {
    let (first, second) = match (args.get(0), args.get(1)) {
        (Some(f), Some(s)) => (f.clone(), s.clone()),
        _ => return Err("Not enough arguments."),
    };

    Ok(LispObject::cons(first, second))
}

pub fn list(args: &[LispObject]) -> Result<LispObject, &'static str> {
    Ok(LispObject::new_with(LispType::List(args.into()), true))
}

pub fn add(args: &[LispObject]) -> Result<LispObject, &'static str> {
    let (first, second) = match (args.get(0), args.get(1)) {
        (Some(f), Some(s)) => (f.clone(), s.clone()),
        _ => return Err("Not enough arguments,"),
    };

    if let (LispType::Number(n1), LispType::Number(n2)) = (first.get_type(), second.get_type()) {
        Ok(LispObject::new_with(LispType::Number(n1 + n2), false))
    } else {
        Err("Arguments are of false types.")
    }
}

pub fn print(args: &[LispObject]) -> Result<LispObject, &'static str> {
    let msg = match args.get(0) {
        Some(n) => n,
        None => return Err("Not enough arguments."),
    };

    match msg.get_type() {
        LispType::Number(n) => print!("{}", n),
        LispType::Symbol(s) => print!("{}", s),
        LispType::List(l) => print!("{:?}", l),
        LispType::Cons(c) => print!("{:?}", c),
        LispType::Bool(b) => print!("{}", if b { "t" } else { "nil" }),
    }

    Ok(LispObject::nil())
}
