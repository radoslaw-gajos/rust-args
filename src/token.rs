use crate::schema::argument::ArgumentType;

#[derive(Debug, PartialEq)]
pub enum Token {
    AppName,
    Argument(ArgumentType),
    StrValue(String),
    IntValue(i64),
}

mod tokens;
mod parser;
