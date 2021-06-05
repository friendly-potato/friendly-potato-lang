use std::fmt;

pub const END_LINE: &str = "__end_line__";

#[derive(Debug, Clone)]
pub struct TokenizeError(String);

#[derive(PartialEq)]
enum TokenType {
    None,
    Expression,
    Space,
    Symbol,
    Quotation,
    Bracket(BracketType),
    EscapeCharacter,
    Ignore,
}

#[derive(PartialEq)]
enum BracketType {
    Paren,
    Square,
    Curly,
}

impl fmt::Display for TokenizeError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

pub struct Tokenizer {
    result: Vec<String>,
    token_builder: String,
    current_token_type: TokenType,
}

impl Tokenizer {
    pub fn new() -> Self {
        Tokenizer {
            result: Vec::new(),
            token_builder: String::new(),
            current_token_type: TokenType::None,
        }
    }
    pub fn tokenize(&mut self, value: String) -> Result<Vec<String>, TokenizeError> {
        let mut paren_counter = 0;
        let mut square_bracket_counter = 0;
        let mut curly_bracket_counter = 0;

        let mut is_escape_character = false;

        for (_, c) in value.chars().enumerate() {
            if c == '"' {
                if is_escape_character {
                    // Double quote literal
                    self.add_to_token_builder(c);
                    is_escape_character = false;
                } else if self.current_token_type == TokenType::Quotation {
                    // Close double quote
                    self.add_to_token_builder(c);
                    self.current_token_type = TokenType::None;
                    self.build_token();
                } else {
                    // Open double quote
                    self.build_token();
                    self.add_to_token_builder(c);
                    self.current_token_type = TokenType::Quotation;
                }
            } else if self.current_token_type == TokenType::Quotation {
                if c == '\\' {
                    is_escape_character = true;
                } else {
                    self.add_to_token_builder(c);
                }
            } else {
                match c {
                    '(' => {
                        paren_counter += 1;
                        self.build_bracket(BracketType::Paren, true, c);
                    }
                    ')' => {
                        paren_counter -= 1;
                        self.build_bracket(BracketType::Paren, false, c);
                    }
                    '[' => {
                        square_bracket_counter += 1;
                        self.build_bracket(BracketType::Square, true, c);
                    }
                    ']' => {
                        square_bracket_counter -= 1;
                        self.build_bracket(BracketType::Square, false, c);
                    }
                    '{' => {
                        curly_bracket_counter += 1;
                        self.build_bracket(BracketType::Curly, true, c);
                    }
                    '}' => {
                        curly_bracket_counter -= 1;
                        self.build_bracket(BracketType::Curly, false, c);
                    }
                    ' ' | '\t' | ',' => {
                        self.build_token();
                        self.current_token_type = TokenType::Space;
                    }
                    '\r' | '\n' => {
                        self.build_token();
                        self.token_builder.push_str(END_LINE);
                        self.build_token();
                    }
                    ';' => {
                        self.current_token_type = TokenType::Ignore;
                    }
                    _ => {
                        if self.current_token_type != TokenType::Symbol {
                            self.build_token();
                        }
                        self.current_token_type = TokenType::Symbol;
                        self.add_to_token_builder(c);
                    }
                }
            }
        }

        // Clean up token builder if we didn't end with a newline
        self.build_token();
        // self.token_builder.push_str(END_LINE);
        // self.build_token();

        if paren_counter != 0 {
            return Err(TokenizeError("Mismatched parens".to_string()));
        }
        if square_bracket_counter != 0 {
            return Err(TokenizeError("Mismatched square brackets".to_string()));
        }
        if curly_bracket_counter != 0 {
            return Err(TokenizeError("Mismatched curly brackets".to_string()));
        }

        return Ok(self.result.to_owned());
    }

    fn add_to_token_builder(&mut self, c: char) {
        self.token_builder.push_str(&c.to_string());
    }

    fn build_token(&mut self) {
        if self.token_builder.len() > 0 {
            self.result.push(self.token_builder.clone());
            self.token_builder.clear();
        }
    }

    fn build_bracket(&mut self, bracket_type: BracketType, is_open: bool, c: char) {
        self.build_token();
        if is_open {
            self.current_token_type = TokenType::Bracket(bracket_type);
        } else {
            self.current_token_type = TokenType::None;
        }
        self.add_to_token_builder(c);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_print_hello() {
        let input = r#"print("hello")"#.to_string();

        let mut tokenizer = Tokenizer::new();

        assert_eq!(
            tokenizer.tokenize(input).unwrap(),
            vec!["print", "(", "\"hello\"", ")"]
        )
    }
}
