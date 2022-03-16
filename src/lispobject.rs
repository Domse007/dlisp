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
