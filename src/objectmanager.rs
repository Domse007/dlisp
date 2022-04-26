use crate::{
    functions::cons,
    lispobject::{LispObject, LispType},
};

#[derive(Default, Debug)]
pub struct Manager {
    frames: Vec<Frame>,
}

impl Manager {
    pub fn new_frame(&mut self) {
        self.frames.push(Frame::default());
    }

    pub fn pop_frame(&mut self) {
        self.frames.pop();
    }

    pub fn get_val(&mut self, name: LispObject) -> Option<LispObject> {
        for frame in self.frames.iter().rev() {
            if let Some(val) = frame.get_val(name.clone()) {
                return Some(val);
            }
        }
        None
    }

    pub fn set_val(&mut self, key: LispObject, value: LispObject) -> Option<()> {
        // Loop through all frames and check if the current frame contains the
        // variable. If it finds the variable: return.
        for frame in self.frames.iter_mut().rev() {
            if frame.set_val(key.clone(), value.clone()).is_some() {
                return Some(());
            }
        }

        // If we did't already return, there was no variable with that name and
        // it gets inserted into the current frame.
        if let Some(frame) = self.frames.last_mut() {
            return frame.set_val_force(key, value);
        }

        None
    }
}

#[derive(Default, Debug)]
struct Frame {
    scoped_objects: Vec<LispObject>,
}

impl Frame {
    pub fn push(&mut self, obj: LispObject) {
        self.scoped_objects.push(obj);
    }

    pub fn get_val(&self, name: LispObject) -> Option<LispObject> {
        let var_name = match name.get_type() {
            LispType::Symbol(s) => s,
            _ => return None,
        };

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
        let var_name = match name.get_type() {
            LispType::Symbol(s) => s,
            _ => return None,
        };

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

        //self.scoped_objects.push(cons(&[name, new]).unwrap());

        None
    }

    pub fn set_val_force(&mut self, name: LispObject, new: LispObject) -> Option<()> {
        self.scoped_objects.push(cons(&[name, new]).unwrap());
        Some(())
    }
}

#[cfg(test)]
mod tests {
    use std::panic;

    use super::*;

    fn test_frame_default() -> Frame {
        let mut frame = Frame::default();
        frame.push(cons(&[LispObject::symbol("test"), LispObject::number(22.)]).unwrap());
        frame.push(cons(&[LispObject::symbol("other-var"), LispObject::bool(true)]).unwrap());
        frame.push(cons(&[LispObject::symbol("another-one"), LispObject::number(23.)]).unwrap());
        frame
    }

    #[test]
    fn test_frame_get_val() {
        let frame = test_frame_default();
        if let LispType::Bool(var) = frame
            .get_val(LispObject::symbol("other-var"))
            .unwrap()
            .get_type()
        {
            assert!(var);
        } else {
            panic!("Value could not be found.");
        }

        if let LispType::Number(n) = frame
            .get_val(LispObject::symbol("another-one"))
            .unwrap()
            .get_type()
        {
            assert_eq!(n, 23.);
        } else {
            panic!("Value could not be found.");
        }
    }

    #[test]
    fn test_frame_set_val_existing() {
        let mut frame = test_frame_default();
        frame.set_val(LispObject::symbol("other-var"), LispObject::bool(true));

        if let LispType::Bool(b) = frame
            .get_val(LispObject::symbol("other-var"))
            .unwrap()
            .get_type()
        {
            assert!(b);
        } else {
            panic!("Value could not be found.");
        }
    }

    #[test]
    fn test_frame_multiple_frames() {
        let mut manager = Manager::default();
        manager.new_frame();
        manager.set_val(LispObject::symbol("test"), LispObject::nil());
        manager.set_val(LispObject::symbol("other"), LispObject::nil());
        manager.new_frame();
        manager.set_val(LispObject::symbol("second"), LispObject::number(1.));
        assert_eq!(
            manager.get_val(LispObject::symbol("other")).unwrap(),
            LispObject::nil()
        );
    }

    #[test]
    fn test_frame_multiple_frames_out_of_scope() {
        let mut manager = Manager::default();
        manager.new_frame();
        manager.set_val(LispObject::symbol("this"), LispObject::number(1.));
        manager.new_frame();
        manager.set_val(LispObject::symbol("test"), LispObject::number(2.));
        assert_eq!(
            manager.get_val(LispObject::symbol("test")),
            Some(LispObject::number(2.))
        );
        manager.pop_frame();
        assert_eq!(manager.get_val(LispObject::symbol("test")), None);
    }
}
