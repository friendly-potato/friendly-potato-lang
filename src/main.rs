pub mod expression;
mod parser;
mod tokenizer;

use clap::{App, Arg, SubCommand};

use expression::{Expression, ExpressionType};
use parser::Parser;
use tokenizer::Tokenizer;

fn main() {
    let input = r#"print("hello")"#.to_string();

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

    // println!("{:?}", parser_value);
}
