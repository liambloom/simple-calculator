use crate::ast::{Function, Value, Args};
use crate::tokenize::Punct;

trait MathOperation: Function {
  fn symbol() -> Punct;
}

struct Add;

impl Function for Add {
  fn run(&self, args: &Args) -> Value {
    if args.len() != 2 {
      todo!();
    }

    let (Value::Number(left), Value::Number(right)) = (&args[0], &args[1]);
    Value::Number(left + right)
  }
}