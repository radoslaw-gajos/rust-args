use crate::schema::argument::ArgumentType;

#[derive(Debug, PartialEq)]
pub enum Token {
    AppName,
    Argument(ArgumentType),
    StrValue(String),
    IntValue(i64),
}

pub mod tokens;
pub mod parser;
