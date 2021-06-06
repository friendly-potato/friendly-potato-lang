use crate::expression::{AtomType, Expression, ExpressionType};
use crate::types::{PotatoPrimitive, PotatoType};
use std::any::Any;
use std::collections::HashMap;

pub enum Env {
    Env(HashMap<String, Box<dyn PotatoPrimitive>>),
    Nil,
}

pub struct Environment {
    inner: Env,
    outer: Env,
}

impl Environment {
    pub fn new(inner: Env, outer: Env) -> Self {
        Environment {
            inner: inner,
            outer: outer,
        }
    }
    pub fn create_base_environment() -> Self {
        Environment {
            inner: Env::Env(HashMap::new()),
            outer: Env::Nil,
        }
    }
}

pub struct CallStack {
    ast_traceback: Vec<u128>,
    exp_index: u128,
}

impl Default for CallStack {
    fn default() -> Self {
        CallStack {
            ast_traceback: Vec::new(),
            exp_index: 0,
        }
    }
}

#[derive(Debug, Clone)]
pub struct EvaluatorError(String);

pub struct Evaluator {
    call_stack: CallStack,
}

impl Evaluator {
    pub fn new() -> Self {
        Evaluator {
            call_stack: CallStack::default(),
        }
    }

    pub fn evaluate(
        &mut self,
        exp: &Expression,
    ) -> Result<Box<dyn PotatoPrimitive>, EvaluatorError> {
        match exp.get_value() {
            ExpressionType::Atom(n) => match n {
                AtomType::String(v) => {}
                AtomType::Number(v) => {}
                AtomType::Symbol(v) => {}
            },
            ExpressionType::List(n) => {}
        }
        return Ok(Box::new(PotatoType::Int(0)));
    }
}
