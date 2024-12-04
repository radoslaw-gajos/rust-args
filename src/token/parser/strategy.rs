use crate::token::parser::TokenParser;
use crate::token::Token;
use crate::schema::argument::ArgumentType;
use dyn_clone::{clone_trait_object, DynClone};

pub trait ParserStrategy: DynClone {
    fn parse(&self, parser: TokenParser) -> TokenParser;
}

clone_trait_object!(ParserStrategy);

#[derive(Clone)]
struct InitParser;

#[derive(Clone)]
struct ArgumentParser;

#[derive(Clone)]
struct StrParser;

#[derive(Clone)]
struct IntParser;

impl ParserStrategy for InitParser {
    fn parse(&self, mut parser: TokenParser) -> TokenParser {
        parser.tokens.add(Token::AppName);
        parser.set_strategy(Box::new(ArgumentParser));
        parser
    }
}

impl ParserStrategy for ArgumentParser {
    fn parse(&self, mut parser: TokenParser) -> TokenParser {
        let arg = parser.current_arg();
        let arg_type = parser.schema.from_str(arg).expect("Expects valid argument");

        let strategy: Box<dyn ParserStrategy> = match arg_type {
            ArgumentType::Bool => Box::new(ArgumentParser),
            ArgumentType::Int => Box::new(IntParser),
            ArgumentType::Str => Box::new(StrParser),
        };

        parser.set_strategy(strategy);
        parser.tokens.add(Token::Argument(arg_type));

        parser
    }
}

impl ParserStrategy for StrParser {
    fn parse(&self, mut parser: TokenParser) -> TokenParser {
        let string_value = parser.current_arg().to_string();
        let token = Token::StrValue(string_value);

        parser.tokens.add(token);

        parser.set_strategy(Box::new(ArgumentParser));
        parser
    }
}

impl ParserStrategy for IntParser {
    fn parse(&self, mut parser: TokenParser) -> TokenParser {
        let int_value = parser.current_arg().parse().expect("Valid number expected");
        let token = Token::IntValue(int_value);

        parser.tokens.add(token);

        parser.set_strategy(Box::new(ArgumentParser));
        parser
    }
}

impl Default for Box<dyn ParserStrategy> {
    fn default() -> Self {
        Box::new(InitParser)
    }
}

