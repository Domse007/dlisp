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
    let mut ret_val: Result<LispObject, &'static str> = Err("No value applied.");
    // Check if the to be evaled object is a list.
    if let LispType::List(list) = eval_obj.get_type() {
        let mut list = list.iter();
        if let Some(obj) = list.next() {
            // Check if the first element is a symbol, otherwise error.
            if let LispType::Symbol(sym) = obj.get_type() {
                // Recursively eval its parameters.
                // Also replace the unquoted symbols with values. => Variables
                let mut args = vec![];
                for arg in list {
                    let evaluated = eval(arg.clone(), obj_manager)?;
                    if !evaluated.is_quoted() {
                        if let LispType::Symbol(_) = evaluated.get_type() {
                            args.push(obj_manager.get_val(evaluated).unwrap());
                        } else {
                            args.push(evaluated);
                        }
                    } else {
                        args.push(evaluated);
                    }
                }
                let mut args = args.iter();
                // Check if it's a special form. Otherwise, eval normally.
                match sym.as_str() {
                    "setq" => {
                        obj_manager
                            .set_val(args.next().unwrap().clone(), args.next().unwrap().clone());
                        ret_val = Ok(LispObject::nil());
                    }
                    _ => ret_val = call_builtin(&sym, &args.cloned().collect::<Vec<LispObject>>()),
                }
            } else {
                ret_val = Err("Invalid Syntax. Non quoted list must start with a symbol.")
            }
        } else {
            ret_val = Ok(LispObject::nil());
        }
    } else {
        ret_val = Ok(eval_obj)
    }

    obj_manager.pop_frame();
    ret_val
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
}
