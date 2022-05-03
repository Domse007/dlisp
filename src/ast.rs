use crate::lispobject::{LispObject, LispType};

#[derive(Debug, Clone, PartialEq)]
struct WorkingLispObject {
    finished: bool,
    objects: Vec<LispObject>,
}

impl WorkingLispObject {
    fn new() -> Self {
        Self {
            finished: false,
            objects: vec![],
        }
    }

    fn merge(&mut self, obj: Self) -> Result<(), Self> {
        if !self.finished {
            self.objects.push(obj.into());
            return Ok(());
        }
        Err(obj)
    }

    fn set_done(&mut self) {
        self.finished = true;
    }

    fn push(&mut self, obj: LispObject) {
        self.objects.push(obj);
    }
}

impl From<WorkingLispObject> for LispObject {
    fn from(this: WorkingLispObject) -> Self {
        if this.objects.is_empty() {
            LispObject::nil()
        } else {
            LispObject::new_with(LispType::List(this.objects), false)
        }
    }
}

pub fn ast(code: &str) -> Result<Vec<LispObject>, &'static str> {
    let tokens = code
        .replace('(', " ( ")
        .replace(')', " ) ")
        .split(' ')
        .map(|e| e.to_string())
        .filter(|x| !x.is_empty())
        .filter(|x| !x.contains('\n'))
        .collect::<Vec<String>>();

    let mut stack: Vec<WorkingLispObject> = vec![];

    for token in tokens {
        if token == "(" {
            stack.push(WorkingLispObject::new());
        } else if token == ")" {
            let mut elem = stack.pop().unwrap();
            elem.set_done();
            match stack.iter_mut().last() {
                Some(merger) => {
                    let merge_status = merger.merge(elem);
                    if merge_status.is_err() {
                        stack.push(merge_status.err().unwrap())
                    }
                }
                None => stack.push(elem),
            }
        } else {
            stack
                .iter_mut()
                .last()
                .unwrap()
                .push(LispObject::new(token.as_str()));
        }
    }

    Ok(stack
        .iter()
        .cloned()
        .map(|e| e.into())
        .collect::<Vec<LispObject>>())
}

#[cfg(test)]
mod tests {
    use crate::evaluator::eval;
    use crate::lispobject::{LispObject, LispType};
    use crate::objectmanager::Manager;

    use super::ast;

    #[test]
    fn test_ast_gen() {
        let test = "(test (+ 1 1) 2)";
        let exp = LispObject::new_with(
            LispType::List(vec![
                LispObject::symbol("test"),
                LispObject::new_with(
                    LispType::List(vec![
                        LispObject::symbol("+"),
                        LispObject::number(1.),
                        LispObject::number(1.),
                    ]),
                    false,
                ),
                LispObject::number(2.),
            ]),
            false,
        );
        assert_eq!(ast(test).unwrap()[0], exp);
    }

    #[test]
    fn test_ast_gen_multiple() {
        let test = "(+ (+ 2 3) 4)";
        let exp = LispObject::new_with(
            LispType::List(vec![
                LispObject::symbol("+"),
                LispObject::new_with(
                    LispType::List(vec![
                        LispObject::symbol("+"),
                        LispObject::number(2.),
                        LispObject::number(3.),
                    ]),
                    false,
                ),
                LispObject::number(4.),
            ]),
            false,
        );
        let res = ast(test).unwrap();
        assert_eq!(res[0], exp);
        assert_eq!(
            eval(res[0].clone(), &mut Manager::default()).unwrap(),
            LispObject::number(9.)
        )
    }
}
