#[derive(PartialEq, PartialOrd, Debug, Clone)]
pub struct LispObject {
    ltype: LispType,
    quoted: bool,
}

impl LispObject {
    pub fn new(mut token: &str) -> Self {
        let quoted = if token.starts_with('\'') {
            token = &token[1..];
            true
        } else {
            false
        };
        Self {
            ltype: LispType::new(token),
            quoted,
        }
    }

    pub fn new_list(tokens: &[String], quoted: bool) -> LispObject {
        let res = tokens
            .iter()
            .map(|e| Self::new(e))
            .collect::<Vec<LispObject>>();
        Self {
            ltype: LispType::List(res),
            quoted,
        }
    }

    pub fn is_quoted(&self) -> bool {
        self.quoted
    }

    pub fn get_type(&self) -> LispType {
        self.ltype.clone()
    }

    pub fn set_quoted(&mut self) {
        self.quoted = true;
    }

    pub fn new_with(ltype: LispType, quoted: bool) -> Self {
        Self { ltype, quoted }
    }

    pub fn nil() -> Self {
        Self {
            ltype: LispType::Bool(false),
            quoted: false,
        }
    }

    pub fn symbol<T: ToString>(name: T) -> Self {
        Self {
            ltype: LispType::Symbol(name.to_string()),
            quoted: false,
        }
    }

    pub fn number(num: f64) -> Self {
        Self {
            ltype: LispType::Number(num),
            quoted: false,
        }
    }

    pub fn list(list: &[LispObject]) -> Self {
        Self {
            ltype: LispType::List(list.into()),
            quoted: false,
        }
    }

    pub fn cons(key: LispObject, val: LispObject) -> Self {
        Self {
            ltype: LispType::new_cons((key, val)),
            quoted: false,
        }
    }

    pub fn bool(b: bool) -> Self {
        Self {
            ltype: LispType::Bool(b),
            quoted: false,
        }
    }
}

impl ToString for LispObject {
    fn to_string(&self) -> String {
        if self.quoted {
            format!("'{}", self.ltype.to_string())
        } else {
            self.ltype.to_string()
        }
    }
}

#[derive(PartialEq, PartialOrd, Debug, Clone)]
pub enum LispType {
    Number(f64),
    Symbol(String),
    List(Vec<LispObject>),
    Cons(Box<(LispObject, LispObject)>),
    Bool(bool),
}

impl LispType {
    pub fn new(token: &str) -> Self {
        if let Ok(num) = token.parse::<f64>() {
            Self::Number(num)
        } else if token == "t" {
            Self::Bool(true)
        } else if token == "nil" {
            Self::Bool(false)
        } else {
            Self::Symbol(token.to_string())
        }
    }

    pub fn new_cons(pair: (LispObject, LispObject)) -> Self {
        Self::Cons(Box::new(pair))
    }
}

impl ToString for LispType {
    fn to_string(&self) -> String {
        match self {
            Self::Number(n) => n.to_string(),
            Self::Symbol(s) => s.to_string(),
            Self::List(l) => {
                let mut s = "(".to_string();
                for o in l {
                    s.push_str(o.to_string().as_str());
                    s.push(' ');
                }
                s.pop();
                s.push(')');
                s
            }
            Self::Cons(c) => format!("({} {})", c.0.to_string(), c.1.to_string()),
            Self::Bool(b) => {
                if *b {
                    "t".to_string()
                } else {
                    "nil".to_string()
                }
            }
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lispobject() {
        let tests = [
            ("'test", LispType::Symbol("test".to_string()), true),
            ("'12", LispType::Number(12.), true),
            ("33", LispType::Number(33.), false),
            ("test", LispType::Symbol("test".to_string()), false),
        ];

        for (test, exp, q) in tests {
            let res = LispObject::new(test);
            assert_eq!(res.get_type(), exp);
            assert_eq!(res.is_quoted(), q);
        }
    }

    #[test]
    fn test_lisptype() {
        let tests = [
            ("33.3", LispType::Number(33.3)),
            ("5", LispType::Number(5.)),
            ("lisp", LispType::Symbol("lisp".to_string())),
        ];
        for (test, res) in tests {
            assert_eq!(LispType::new(test), res);
        }
    }

    #[test]
    fn test_lisptype_to_string() {
        let tests = [
            (LispObject::bool(true), "t"),
            (
                LispObject::cons(LispObject::nil(), LispObject::symbol("test")),
                "(nil test)",
            ),
            (
                LispObject::list(&[LispObject::nil(), LispObject::number(2.1)]),
                "(nil 2.1)",
            ),
        ];
        for (test, exp) in tests {
            assert_eq!(test.to_string(), exp.to_string());
        }
    }
}
