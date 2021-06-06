use std::fmt;

#[derive(Clone, Debug)]
pub struct ExpressionError(String);

#[derive(Clone, Debug)]
pub enum ExpressionType {
    Atom(AtomType),
    List(Vec<Expression>),
}

impl fmt::Display for ExpressionType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self)
    }
}

// impl<T> ExpressionType<T> {
//     pub fn get_atom_value(&self) -> Result<String, ExpressionError> {
//         match self {
//             ExpressionType::Atom(s) => Ok(s.to_string()),
//             _ => Err(ExpressionError("Not an atom".to_string())),
//         }
//     }

//     pub fn get_list_value(&self) -> Result<&Vec<Expression>, ExpressionError> {
//         match self {
//             ExpressionType::List(l) => Ok(l),
//             _ => Err(ExpressionError("Not a list".to_string())),
//         }
//     }
// }

#[derive(Clone, Debug)]
pub enum AtomType {
    Symbol(String),
    String(String),
    Number(String),
}

impl fmt::Display for AtomType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self)
    }
}

#[derive(Clone, Debug)]
pub struct Expression(ExpressionType);

impl Expression {
    pub fn new(et: ExpressionType) -> Self {
        Expression(et)
    }

    pub fn new_list() -> Self {
        Expression(ExpressionType::List(Vec::new()))
    }

    pub fn new_symbol(value: String) -> Self {
        Expression(ExpressionType::Atom(AtomType::Symbol(value)))
    }

    pub fn new_string(value: String) -> Self {
        Expression(ExpressionType::Atom(AtomType::String(value)))
    }

    pub fn new_number(value: String) -> Self {
        Expression(ExpressionType::Atom(AtomType::Number(value)))
    }

    pub fn push(&mut self, exp: Expression) -> Result<(), String> {
        match self.0.to_owned() {
            ExpressionType::Atom(_) => Err("Tried to append to an atom".to_string()),
            ExpressionType::List(mut n) => {
                n.push(exp);
                self.0 = ExpressionType::List(n);
                return Ok(());
            }
        }
    }

    pub fn len(&self) -> usize {
        match &self.0 {
            ExpressionType::Atom(_) => 0,
            ExpressionType::List(n) => n.len(),
        }
    }

    pub fn get_value(&self) -> &ExpressionType {
        return &self.0;
    }
}

impl fmt::Display for Expression {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}
