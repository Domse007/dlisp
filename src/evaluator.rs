use crate::functions::call_builtin;
use crate::lispobject::{LispObject, LispType};
use crate::objectmanager::Manager;

pub fn eval(eval_obj: LispObject, obj_manager: &mut Manager) -> Result<LispObject, &'static str> {
    if eval_obj.is_quoted() {
        return Ok(eval_obj);
    }
    obj_manager.new_frame();

    let mut ret_obj = Err("");

    match eval_obj.get_type() {
        LispType::List(list) => {
            if let LispType::Symbol(sym) = list.first().unwrap().get_type() {
                let mut args = vec![];
                for arg in list.iter().skip(1) {
                    let arg = eval(arg.clone(), obj_manager)?;
                    args.push(match obj_manager.get_val(arg.clone()) {
                        Some(val) => val,
                        None => arg.clone(),
                    });
                }
                dbg!(&sym);
                dbg!(&args);
                ret_obj = match sym.as_str() {
                    "progn" => {
                        obj_manager.new_frame();
                        let mut ret = Err("");
                        for arg in args {
                            ret = eval(arg, obj_manager);
                        }
                        obj_manager.pop_frame();
                        ret
                    }
                    "setq" => {
                        obj_manager.set_val(args[0].clone(), args[1].clone());
                        Ok(LispObject::nil())
                    }
                    _ => call_builtin(sym.as_str(), &args),
                };
            }
        }
        _ => ret_obj = Ok(eval_obj),
    }

    obj_manager.pop_frame();
    ret_obj
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
            LispObject::symbol("progn"),
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
            LispObject::symbol("progn"),
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
