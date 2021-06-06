use crate::expression::Expression;
use crate::tokenizer::END_LINE;
use std::fmt;

#[derive(Debug, Clone)]
pub struct ParserError(String);

impl fmt::Display for ParserError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

pub struct Parser {
    result: Expression,
    depth: i32,
}

impl Parser {
    pub fn new() -> Self {
        Parser {
            result: Expression::new_list(),
            depth: 0,
        }
    }

    pub fn parse(&mut self, tokens: &mut Vec<String>) -> Result<Expression, ParserError> {
        if tokens.len() == 0 {
            return Err(ParserError("Unexpected EOF".to_string()));
        }

        let mut is_param = false;
        let mut param_counter = 0;

        let mut current_expression = Expression::new_list();
        let mut param_expression = Expression::new_list();

        tokens.reverse();

        loop {
            let t = tokens.pop();
            println!("\nprocessing {}", t.clone().unwrap_or_default());
            match t {
                Some(w) => match &*w {
                    "(" => {
                        is_param = true;
                        param_counter += 1;
                    }
                    ")" => {
                        param_counter -= 1;
                        if param_counter == 0 {
                            is_param = false;
                            match current_expression.push(param_expression.clone()) {
                                Ok(()) => param_expression = Expression::new_list(),
                                Err(n) => println!("{}", n),
                            }
                        }
                    }
                    "then" => {
                        self.depth += 1;
                        match self.parse(tokens) {
                            Ok(n) => match current_expression.push(n.clone()) {
                                Ok(()) => {}
                                Err(n) => {
                                    println!("{}", n);
                                    break;
                                }
                            },
                            Err(n) => {
                                println!("{}", n);
                                break;
                            }
                        }
                    }
                    "end" => {
                        self.depth -= 1;
                        match self.result.push(current_expression.clone()) {
                            Ok(()) => current_expression = Expression::new_list(),
                            Err(n) => println!("{}", n),
                        }
                    }
                    END_LINE => {
                        if param_counter == 0 && current_expression.len() > 0 {
                            match self.result.push(current_expression.clone()) {
                                Ok(()) => current_expression = Expression::new_list(),
                                Err(n) => println!("{}", n),
                            }
                        } else {
                            println!("ignoring end line")
                        }
                    }
                    _ => {
                        if is_param {
                            println!("param {}", w);
                            match param_expression.push(Expression::new_string(w)) {
                                Ok(()) => {}
                                Err(n) => println!("{}", n),
                            }
                        } else {
                            println!("symbol {}", w);
                            // String
                            if w.chars().nth(0).unwrap() == '"' {
                                match current_expression.push(Expression::new_string(w)) {
                                    Ok(()) => {}
                                    Err(n) => println!("{}", n),
                                }
                            }
                            // Number
                            else if let Ok(_) = w.parse::<f32>() {
                                match current_expression.push(Expression::new_number(w)) {
                                    Ok(()) => {}
                                    Err(n) => println!("{}", n),
                                }
                            }
                            // Symbol
                            else {
                                match current_expression.push(Expression::new_symbol(w)) {
                                    Ok(()) => {}
                                    Err(n) => println!("{}", n),
                                }
                            }
                        }
                    }
                },
                None => break, // Finished parsing
            }
        }

        if param_expression.len() > 0 {
            match current_expression.push(param_expression.clone()) {
                Ok(()) => {}
                Err(n) => println!("{}", n),
            }
        }

        if current_expression.len() > 0 {
            match self.result.push(current_expression) {
                Ok(()) => {}
                Err(n) => println!("{}", n),
            }
        }
        return Ok(self.result.to_owned());
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn name() {
        unimplemented!();
    }
}
