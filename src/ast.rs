use std::{rc::Rc, collections::HashMap};

use num_rational::{Ratio, Rational64};

use crate::tokenize::primary::{PreToken, PreTokenStream, Punct};

enum CodeObject {
    Value(Value),
    ExecuteFn(Rc<Function>, Vec<CodeObject>),
    Parenthases(Code),
}

type Code = Vec<CodeObject>;

enum Value {
    Number(Rational64),
}

struct Function {

}

fn parse(stream: PreTokenStream) -> Code {
    todo!()
}

// const ORDER: HashMap<Punct, u8> = HashMap::from([

// ]);
