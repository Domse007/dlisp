use crate::{
    functions::cons,
    lispobject::{LispObject, LispType},
};

pub struct Manager {
    frames: Vec<Frame>,
}

#[derive(Default)]
struct Frame {
    scoped_objects: Vec<LispObject>,
}

impl Frame {
    pub fn push(&mut self, obj: LispObject) {
        self.scoped_objects.push(obj);
    }

    pub fn get_val(&self, name: LispObject) -> Option<LispObject> {
        let mut var_name: String = String::new();

        match name.get_type() {
            LispType::Symbol(s) => var_name = s,
            _ => return None,
        }

        for obj in self.scoped_objects.iter() {
            if let LispType::Cons(pair) = obj.get_type() {
                if let LispType::Symbol(s) = pair.0.get_type() {
                    if s == var_name {
                        return Some(pair.1);
                    }
                }
            }
        }

        None
    }

    pub fn set_val(&mut self, name: LispObject, new: LispObject) -> Option<()> {
        let mut var_name = String::new();

        match name.get_type() {
            LispType::Symbol(s) => var_name = s,
            _ => return None,
        }

        for obj in self.scoped_objects.iter_mut() {
            if let LispType::Cons(pair) = obj.get_type() {
                if let LispType::Symbol(s) = pair.0.get_type() {
                    if s == var_name {
                        *obj = cons(&[pair.0, new]).unwrap();
                        return Some(());
                    }
                }
            }
        }

        None
    }
}
