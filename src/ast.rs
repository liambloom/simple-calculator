use std::{rc::Rc, collections::HashMap, ops::Range};

use num_rational::{Ratio, Rational64};

use crate::{tokenize::{Token, TokenStream, Punct, TokenContent}, Location, LocatableContent};

pub type CodeObject = LocatableContent<CodeObjectContent>;

pub enum CodeObjectContent {
    Value(Value),
    ExecuteFn(Rc<dyn Function>, Vec<CodeObjectContent>),
    Parenthases(Code),
}

pub type Code = Vec<CodeObject>;
pub type Args = Vec<Value>;

pub enum Value {
    Number(Rational64),
}

pub trait Function {
    fn run(&self, args: &Args) -> Value;
}



fn parse(stream: TokenStream) -> Code {
    let mut parenthases = vec![(Code::new(), Location::default())];

    for (i, token) in stream.iter().enumerate() {
        match token.content() {
            TokenContent::Punct(Punct::OpenParenth) => {
                // parentheses_start = token.location().begin();
                parenthases.push((Code::new(), *token.location()));
            }
            TokenContent::Punct(Punct::CloseParenth) => {
                if parenthases.len() < 2 {
                    panic!();
                }
                let code = parenthases.pop().unwrap();
                parenthases.last_mut().unwrap().0.push(CodeObject::new(CodeObjectContent::Parenthases(code.0), code.1));
            }

            _ => todo!()
        }
    }

    todo!()
}

trait TokenProcesssor {
    /// Processes a particular token. The result is a code
    fn process(&self, token: &Token, index: &usize, stream: &TokenStream) -> CodeObject;
}

struct NumbericBinaryOperationProcessor {

}

// const ORDER: HashMap<Punct, u8> = HashMap::from([

// ]);
