use super::{Function, Runtime, Result};
use crate::ast::{Args, Value};

pub struct Print;

impl Function for Print {
    fn run(&self, runtime: &Runtime, args: &Args) -> Option<Result<Value>> {
        println!("{:?}", args[0]);
        None
    }
}