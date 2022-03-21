use crate::lispobject::LispObject;

enum FirstStage {
    Object(LispObject),
    LParen,
    RParen,
}

pub fn ast(code: &str) -> Result<LispObject, &'static str> {
    let tokens = code
        .replace("(", " ( ")
        .replace(")", " ) ")
        .split(" ")
        .map(|e| e.to_string())
        .collect::<Vec<String>>();
    let mut representation = vec![];

    for token in tokens {
        if token == "(" {
            representation.push(FirstStage::LParen);
        } else if token == ")" {
            representation.push(FirstStage::RParen);
        } else {
            representation.push(FirstStage::Object(LispObject::new(&token)));
        }
    }

    Err("")
}

enum SecondStage {}
