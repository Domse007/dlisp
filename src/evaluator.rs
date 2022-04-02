use std::slice::Iter;

use crate::{
    functions::call_builtin,
    lispobject::{LispObject, LispType},
    objectmanager::Manager,
};

pub fn eval(eval_obj: LispObject, obj_manager: &mut Manager) -> Result<LispObject, &'static str> {
    // If it is quoted, return early. There is nothing to be done.
    if eval_obj.is_quoted() {
        return Ok(eval_obj);
    }

    // Get a new frame for scoping.
    obj_manager.new_frame();

    // Check if the to be evaled object is a list.
    let ret_val: Result<LispObject, &'static str> =
        if let LispType::List(list) = eval_obj.get_type() {
            dbg!(&list);
            eval_list(eval_obj, list, obj_manager)
        } else {
            Ok(eval_obj)
        };

    obj_manager.pop_frame();
    ret_val
}

/// Wrapper function to evaluate lists.
fn eval_list(
    eval_obj: LispObject,
    list: Vec<LispObject>,
    obj_manager: &mut Manager,
) -> Result<LispObject, &'static str> {
    let mut ret_val = Err("Could not process list.");
    let mut list = list.iter();
    if let Some(obj) = list.next() {
        // Check if the first element is a symbol, otherwise error.
        if let LispType::Symbol(sym) = obj.get_type() {
            let args = replace_var_with_val(list, obj_manager)?;
            // Check if it's a special form. Otherwise, eval normally.
            ret_val = check_run_special_form(sym, args, obj_manager);
        } else if let LispType::List(int_list) = obj.get_type() {
            for elem in int_list {
                ret_val = eval(elem, obj_manager);
            }
        } else if let LispType::Symbol(_) = obj.get_type() {
            return match obj_manager.get_val(obj.clone()) {
                Some(val) => Ok(val),
                None => Err("Value could not be found."),
            };
        } else {
            //ret_val = Err("Invalid Syntax. Non quoted list must start with a symbol.")
            ret_val = Ok(eval_obj);
        }
    } else {
        ret_val = Ok(LispObject::nil());
    }
    ret_val
}

/// Check if the symbol is a special form and evaluate it.
fn check_run_special_form(
    symbol: String,
    args: Vec<LispObject>,
    obj_manager: &mut Manager,
) -> Result<LispObject, &'static str> {
    let mut args = args.iter();
    match symbol.as_str() {
        "setq" => {
            obj_manager.set_val(args.next().unwrap().clone(), args.next().unwrap().clone());
            Ok(LispObject::nil())
        }
        _ => call_builtin(&symbol, &args.cloned().collect::<Vec<LispObject>>()),
    }
}

fn replace_var_with_val(
    list: Iter<LispObject>,
    obj_manager: &mut Manager,
) -> Result<Vec<LispObject>, &'static str> {
    // Recursively eval its parameters.
    // Also replace the unquoted symbols with values. => Variables
    let mut args = vec![];
    for arg in list {
        let evaluated = eval(arg.clone(), obj_manager)?;
        if !evaluated.is_quoted() {
            if let LispType::Symbol(_) = evaluated.get_type() {
                // eval the returned lispobject
                args.push(eval(obj_manager.get_val(evaluated).unwrap(), obj_manager)?);
            } else {
                args.push(evaluated);
            }
        } else {
            args.push(evaluated);
        }
    }
    Ok(args)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_eval_simple_built_in() {
        let eval_obj = LispObject::list(&[
            LispObject::new("add"),
            LispObject::new("22"),
            LispObject::new("33"),
        ]);
        let mut obj_manager = Manager::default();
        let res = eval(eval_obj, &mut obj_manager).unwrap();

        assert_eq!(LispObject::new_with(LispType::Number(55.), false), res);
    }

    #[test]
    fn test_eval_recursive_built_in() {
        let eval_obj = LispObject::list(&[
            LispObject::new("add"),
            LispObject::list(&[
                LispObject::new("add"),
                LispObject::new("11"),
                LispObject::new("22"),
            ]),
            LispObject::new("33"),
        ]);
        let mut obj_manager = Manager::default();
        let res = eval(eval_obj, &mut obj_manager).unwrap();

        assert_eq!(LispObject::new_with(LispType::Number(66.), false), res);
    }

    #[test]
    fn test_eval_multiple_passes_built_in() {
        let eval_obj = LispObject::list(&[
            LispObject::new("add"),
            LispObject::new("22"),
            LispObject::new("33"),
        ]);
        let mut obj_manager = Manager::default();
        let res = eval(eval_obj.clone(), &mut obj_manager).unwrap();
        assert_eq!(LispObject::new_with(LispType::Number(55.), false), res);
        let res = eval(eval_obj, &mut obj_manager).unwrap();
        assert_eq!(LispObject::new_with(LispType::Number(55.), false), res);
    }

    #[test]
    fn test_eval_setq() {
        let test = LispObject::list(&[
            LispObject::list(&[
                LispObject::symbol("setq"),
                LispObject::symbol("test"),
                LispObject::bool(true),
            ]),
            LispObject::symbol("test"),
        ]);
        let mut obj_manager = Manager::default();
        let res = eval(test, &mut obj_manager).unwrap();
        assert_eq!(res, LispObject::bool(true));
    }

    #[test]
    fn test_eval_var_lookup() {
        let eval_obj = LispObject::list(&[
            LispObject::list(&[
                LispObject::symbol("setq"),
                LispObject::symbol("test"),
                LispObject::list(&[
                    LispObject::symbol("+"),
                    LispObject::number(22.),
                    LispObject::number(23.),
                ]),
            ]),
            LispObject::list(&[
                LispObject::symbol("add"),
                LispObject::symbol("test"),
                LispObject::number(2.),
            ]),
            LispObject::symbol("test"),
        ]);
        let mut obj_manager = Manager::default();
        let res = eval(eval_obj, &mut obj_manager).unwrap();
        assert_eq!(res, LispObject::number(47.));
    }

    #[test]
    fn test_recursive_no_var() {
        let eval_obj = LispObject::list(&[
            LispObject::symbol("+"),
            LispObject::list(&[
                LispObject::symbol("+"),
                LispObject::number(2.),
                LispObject::number(3.),
            ]),
            LispObject::number(4.),
        ]);
        let mut obj_manager = Manager::default();
        let res = eval(eval_obj, &mut obj_manager).unwrap();
        assert_eq!(res, LispObject::number(9.));
    }
}
