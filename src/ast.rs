use std::iter::Peekable;
use std::slice::Iter;

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

struct UnfinishedObject {
    parts: Vec<LispObject>,
}

impl UnfinishedObject {
    pub fn new() -> Self {
        Self { parts: vec![] }
    }

    pub fn push(&mut self, part: LispObject) {
        self.parts.push(part)
    }

    pub fn get(self) -> Vec<LispObject> {
        self.parts
    }
}

fn parse_tree(tree: &mut Peekable<Iter<FirstStage>>) -> Result<LispObject, &'static str> {
    let mut obj = UnfinishedObject::new();

    match tree.peek() {
        Some(FirstStage::LParen) => {
            while let Some(first) = tree.peek() {
                match first {
                    FirstStage::Object(ob) => obj.push(ob.clone()),
                    FirstStage::LParen => obj.push(parse_tree(tree)?),
                    FirstStage::RParen => return Ok(LispObject::list(&obj.get())),
                }
                if let FirstStage::Object(o) = tree.next().unwrap() {
                    obj.push(o.clone())
                }
            }
        }
        Some(FirstStage::RParen) => {
            return Ok(LispObject::list(&obj.get()));
        }
        _ => unreachable!(),
    }

    Err("Could not parse tree.")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_tree() {
        let tree = vec![
            FirstStage::LParen,
            FirstStage::Object(LispObject::nil()),
            FirstStage::LParen,
            FirstStage::Object(LispObject::nil()),
            FirstStage::RParen,
            FirstStage::Object(LispObject::nil()),
            FirstStage::RParen,
        ];
        let exp = LispObject::list(&[
            LispObject::nil(),
            LispObject::list(&[LispObject::nil()]),
            LispObject::nil(),
        ]);

        assert_eq!(parse_tree(&mut tree.iter().peekable()).unwrap(), exp);
    }
}
