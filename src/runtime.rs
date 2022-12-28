use crate::{Location, tokenize::Ident, ast::{Args, Value, ExpressionContent, Expression}};
use std::{result, collections::HashMap, rc::Rc};
use native_functions::*;
use runtime_errors::*;

pub mod native_functions;
pub mod runtime_errors;

pub type Result<T> = result::Result<T, Box<dyn RuntimeError>>;





pub trait Function {
  fn run(&self, runtime: &Runtime, args: &Args) -> Option<Result<Value>>;
}

pub struct Runtime {
  functions: HashMap<String, Rc<dyn Function>>
}

impl Runtime {
  pub fn new() -> Self {
    Self {
      functions: HashMap::from([
        (String::from("print"), Rc::new(Print) as Rc<dyn Function>),
      ])
    }
  }

  pub fn resolve_function(&self, ident: &Ident) -> Result<Rc<dyn Function>> {
    match self.functions.get(ident.content()) {
      Some(f) => Ok(Rc::clone(f)),
      None => Err(Box::new(ResolutionError::new(ident.clone())))
    }
  }

  pub fn eval_expr(&self, expr: &Expression) -> Result<Value> {
    use ExpressionContent::*;

    match expr.content() {
      ExecuteFn((f, raw_args)) 
        => Ok(self
            .resolve_function(f)?
            .run(self, &raw_args.iter()
              .map(|arg| self.eval_expr(arg))
              .collect::<Result<Args>>()?)
            .unwrap_or_else(|| Err(Box::new(NonReturingFunctionError::new(*expr.location()))))?),
      Parenthases(content) => self.eval_expr(content),
      Literal(content) => Ok(Value::Simple(*content)),
      BinaryExpr(expr, args) => {
        let args = args.iter().map(|arg| self.eval_expr(arg)).collect::<Result<Args>>()?;
        expr.eval(&args[0], &args[1])
      }
    }
  }
}