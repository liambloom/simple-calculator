use std::{collections::HashMap, ops::Range, rc::Rc};
use lazy_static::lazy_static;

use num_rational::Rational64;

use crate::{LocatableContent, Location, runtime, tokenize::{Punct, Token, TokenContent, TokenStream}};
use crate::error::{CompilationError, CompilationErrorKind};
use crate::tokenize::{DelimiterType, Ident};

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

    pub fn priority(&self) -> u8 {
        use BinaryExpr::*;

        match self {
            Add | Subtract => 1,
            Multiply | Divide => 2,
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


fn parse(stream: TokenStream) -> Result<Code, CompilationError> {
    let mut code = Code::new();
    let mut iter = stream.into_iter();

    while let Some(token) = iter.next()  {
        if let TokenContent::Ident(ident) = token.content() {
            let maybe_args = iter.next().unwrap();
            if let Token { content: TokenContent::Block(DelimiterType::Parenthases, args), location: arg_location } = maybe_args {
                let mut parsed_args = Vec::new();
                let mut last_comma = None;
                for e in args.iter().enumerate() {
                    if e.1.content() == &TokenContent::Punct(Punct::Comma) {
                        let arg_start = last_comma.map(|n| n + 1).unwrap_or(0);
                        if e.0 == arg_start {
                            return Err(CompilationError::new(e.1.location, CompilationErrorKind::TwoCommas));
                        }
                        else {
                            parsed_args.push(eval_expr(&args[arg_start..e.0]));
                            last_comma = Some(e.0);
                        }
                    }
                }

                code.push(CodeObject::new(CodeObjectContent::ExecuteFn((ident.clone(), parsed_args)), (*token.location()..=arg_location).into()));
            }
            else {
                return Err(CompilationError::new(maybe_args.location, CompilationErrorKind::SyntaxError {
                    expected: vec![String::from("(")], found: maybe_args
                }));
            }
        }
    }

    Ok(code)
}

fn eval_expr(stream: &[Token]) -> Expression {
    lazy_static! {
        static ref OPERATOR_ORDER: [Vec<Punct>; 2] = [
            vec![Punct::Asterisk, Punct::Slash],
            vec![Punct::Plus, Punct::Dash],
        ];

        static ref SUPPORTED_OPERATORS: Vec<Punct> = {
            let mut v = Vec::new();
            for i in OPERATOR_ORDER.iter() {
                for j in i {
                    v.push(*j);
                }
            }
            v
        };
    }

    let operators = [Vec::new(), Vec::new()];

    for token in stream.iter() {

    }



    todo!();
}

// const ORDER: HashMap<Punct, u8> = HashMap::from([

// ]);
