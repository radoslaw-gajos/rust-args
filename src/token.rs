use crate::schema::argument::ArgumentType;

type Name = String;

#[derive(Debug, PartialEq)]
pub enum Token {
    AppName,
    Argument(ArgumentType, Name),
    StrValue(String),
    IntValue(i64),
}

pub mod tokens;
pub mod parser;
