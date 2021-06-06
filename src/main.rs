mod evaluator;
mod parser;
mod tokenizer;

pub mod expression;
pub mod types;

use clap::{App, Arg, SubCommand};

use expression::{Expression, ExpressionType};

use evaluator::Evaluator;
use parser::Parser;
use tokenizer::Tokenizer;

fn main() {
    let input = r#"
print("hello")
def x = 0
"#
    .to_string();

    let mut tokenizer = Tokenizer::new();
    let mut tokenizer_value = tokenizer.tokenize(input).unwrap_or_else(|x| {
        println!("{}", x);
        return Vec::new();
    });

    println!("{:?}", tokenizer_value);

    let mut parser = Parser::new();
    let parser_value = parser.parse(&mut tokenizer_value).unwrap_or_else(|x| {
        println!("{}", x);
        return Expression::new_list();
    });

    // TODO debug
    match parser_value.get_value() {
        ExpressionType::List(n) => {
            println!("printing expression list");
            for i in n {
                println!("{:?}", i);
            }
            println!("done");
        }
        _ => println!("bad value"),
    }

    let mut evaluator = Evaluator::new();
    let eval_value = evaluator.evaluate(&parser_value);
}
