use crate::functions::call_builtin;
use crate::lispobject::{LispObject, LispType};
use crate::objectmanager::Manager;

pub fn eval(obj: LispObject, manager: &mut Manager) -> Result<LispObject, &'static str> {
    if obj.is_quoted() {
        return Ok(obj);
    }

    match obj.get_type() {
        LispType::List(l) => eval_list(l, manager),
        LispType::Symbol(_) if !obj.is_quoted() => Ok(match manager.get_val(obj.clone()) {
            Some(var) => var,
            None => obj,
        }),
        _ => Ok(obj),
    }
}

fn eval_list(list: Vec<LispObject>, manager: &mut Manager) -> Result<LispObject, &'static str> {
    manager.new_frame();
    let mut parameters = vec![];
    for element in list.clone() {
        parameters.push(eval(element, manager)?);
    }

    match parameters[0].get_type() {
        LispType::Symbol(s) => match s.as_str() {
            "set" => {
                manager.set_val(parameters[1].clone(), parameters[2].clone());
                Ok(LispObject::nil())
            }
            "quote" => {
                let mut param = parameters[1].clone();
                param.set_quoted();
                Ok(param)
            }
            _ => {
                manager.pop_frame();
                call_builtin(&s, &parameters[1..])
            }
        },
        LispType::List(_) => eval(parameters[0].clone(), manager),
        _ => Err("First element must be a symbol."),
    }
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
        let test = vec![
            LispObject::list(&[
                LispObject::symbol("set"),
                LispObject::symbol("test").move_quoted(),
                LispObject::bool(true),
            ]),
            LispObject::symbol("test"),
        ];
        let mut obj_manager = Manager::default();
        let _first = eval(test[0].clone(), &mut obj_manager).unwrap();
        let res = eval(test[1].clone(), &mut obj_manager).unwrap();
        assert_eq!(res, LispObject::bool(true));
    }

    #[test]
    fn test_eval_var_lookup() {
        let eval_obj = vec![
            LispObject::list(&[
                LispObject::symbol("set"),
                LispObject::symbol("test").move_quoted(),
                LispObject::list(&[
                    LispObject::symbol("+"),
                    LispObject::number(22.),
                    LispObject::number(23.),
                ]),
            ]),
            LispObject::list(&[
                LispObject::symbol("set"),
                LispObject::symbol("test").move_quoted(),
                LispObject::list(&[
                    LispObject::symbol("add"),
                    LispObject::symbol("test"),
                    LispObject::number(2.),
                ]),
            ]),
            LispObject::symbol("test"),
        ];
        let mut obj_manager = Manager::default();
        let _res = eval(eval_obj[0].clone(), &mut obj_manager).unwrap();
        let _res = eval(eval_obj[1].clone(), &mut obj_manager).unwrap();
        let res = eval(eval_obj[2].clone(), &mut obj_manager).unwrap();
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
