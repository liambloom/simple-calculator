use std::{rc::Rc, collections::HashMap, ops::Range};

use num_rational::Rational64;

use crate::{tokenize::{Token, TokenStream, Punct, TokenContent, Ident}, Location, LocatableContent, runtime};

pub type CodeObject = LocatableContent<CodeObjectContent>;

pub enum CodeObjectContent {
    // Value(Value),
    ExecuteFn(FnInfo),
    // Parenthases(Code),
}

pub type Expression = LocatableContent<ExpressionContent>;

// impl Expression {
//     fn eval(&self) -> runtime::Result<Value> {
//         use ExpressionContent::*;
        
        
//     }
// }

pub enum ExpressionContent {
    ExecuteFn(FnInfo),
    Parenthases(Box<Expression>),
    Literal(SimpleValue),
    BinaryExpr(BinaryExpr, Box<[Expression; 2]>)
}

#[derive(Debug, Copy, Clone)]
pub enum BinaryExpr {
    Add,
    Subtract,
    Multiply,
    Divide,
}

impl BinaryExpr {
    pub fn eval(&self, left: &Value, right: &Value) -> runtime::Result<Value> {
        use BinaryExpr::*;

        #[allow(irrefutable_let_patterns)]
        if let (Value::Simple(SimpleValue::Number(left)), Value::Simple(SimpleValue::Number(right))) = (left, right) {
            Ok(Value::number(match self {
                Add => left + right,
                Subtract => left - right,
                Multiply => left * right,
                Divide => left / right,
            }))
        }
        else {
            todo!();
        }
    }

    pub fn symbol(&self) -> &'static str {
        use BinaryExpr::*;

        match self {
            Add => "+",
            Subtract => "-",
            Multiply => "*",
            Divide => "/",
        }
    }
}

type FnInfo = (Ident, Vec<Expression>);

pub type Code = Vec<CodeObject>;
pub type Args = Vec<Value>;

#[derive(Debug, Copy, Clone)]
pub enum SimpleValue {
    Number(Rational64),
}

#[derive(Debug, Clone)]
pub enum Value {
    Simple(SimpleValue),
}

impl Value {
    pub fn number(value: Rational64) -> Value {
        Value::Simple(SimpleValue::Number(value))
    }
}


fn parse(stream: TokenStream) -> Code {
    let mut parenthases = vec![(Code::new(), Location::default())];

    for (i, token) in stream.iter().enumerate() {
        
        
        
    }

    todo!()
}

// const ORDER: HashMap<Punct, u8> = HashMap::from([

// ]);
