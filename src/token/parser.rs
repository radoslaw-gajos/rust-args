use crate::schema::{self, Schema};
use crate::token::tokens::Tokens;
use crate::token::Token;
use crate::schema::argument::ArgumentType;
use dyn_clone::{clone_trait_object, DynClone};

#[derive(Default)]
pub struct TokenParser {
    args: Vec<String>,
    index: usize,
    schema: Schema,
    strategy: Box<dyn ParserStrategy>,
    tokens: Tokens,
}

trait ParserStrategy: DynClone {
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

impl TokenParser {
    fn new() -> Self {
        let mut parser = Self::default();
        parser
    }

    fn args(mut self, args: Vec<&str>) -> Self {
        Self {
            args: args.into_iter().map(String::from).collect(),
            ..self
        }
    }

    fn schema(mut self, schema: Schema) -> Self {
        Self {
            schema,
            ..self
        }
    }

    fn collect(mut self) -> Tokens {
        let mut parser = self;
        while !parser.is_done() {
            parser = parser.parse_current();
            parser.next();
        }
        parser.tokens
    }

    fn is_done(&self) -> bool {
        self.index >= self.args.len()
    }

    fn set_strategy(&mut self, strategy: Box<dyn ParserStrategy>) {
        self.strategy = strategy;
    }

    fn parse_current(mut self) -> Self {
        let strategy = self.strategy.clone();
        strategy.parse(self)
    }

    fn next(&mut self) {
        self.index += 1;
    }
    
    fn current_arg(&self) -> &str {
        &self.args.get(self.index).expect("Index should be valid")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_return_empty_token_collection_when_no_args() {
        // given
        let mut parser = TokenParser::new();

        // when
        let tokens = parser.collect();
        
        // then
        assert_eq!(tokens.size(), 0);
    }

    #[test]
    fn should_interpret_first_arg_as_appname() {
        // given
        let mut parser = TokenParser::new()
            .args(vec!["app_name"]);

        // when
        let tokens = parser.collect();
        
        // then
        assert_eq!(tokens.size(), 1);
    }

    #[test]
    fn should_get_boolean_argument() {
        // given
        let schema = Schema::from(vec![
            ("b".to_string(), "bool".to_string()),
        ]);
        let mut parser = TokenParser::new()
            .args(vec!["app_name", "-b"])
            .schema(schema);

        // when
        let tokens = parser.collect();

        // then
        assert_eq!(tokens.size(), 2);
        assert_eq!(tokens.get(1), &Token::Argument(ArgumentType::Bool));
    }

    #[test]
    fn should_get_string_argument() {
        // given
        let schema = Schema::from(vec![
            ("s".to_string(), "string".to_string()),
        ]);
        let mut parser = TokenParser::new()
            .args(vec!["app_name", "-s", "string"])
            .schema(schema);

        // when
        let tokens = parser.collect();

        // then
        assert_eq!(tokens.size(), 3);
        assert_eq!(tokens.get(1), &Token::Argument(ArgumentType::Str));
        assert_eq!(tokens.get(2), &Token::StrValue("string".to_string()));
    }

    #[test]
    fn should_get_int_argument() {
        // given
        let schema = Schema::from(vec![
            ("i".to_string(), "int".to_string()),
        ]);
        let mut parser = TokenParser::new()
            .args(vec!["app_name", "-i", "42"])
            .schema(schema);

        // when
        let tokens = parser.collect();

        // then
        assert_eq!(tokens.size(), 3);
        assert_eq!(tokens.get(1), &Token::Argument(ArgumentType::Int));
        assert_eq!(tokens.get(2), &Token::IntValue(42));
    }

    #[test]
    fn should_parse_multiple_arguments() {
        // given
        let schema = Schema::from(vec![
            ("i".to_string(), "int".to_string()),
            ("s".to_string(), "string".to_string()),
            ("b".to_string(), "bool".to_string()),
        ]);
        let mut parser = TokenParser::new()
            .args(vec!["app_name", "-i", "42", "-b", "-s", "string"])
            .schema(schema);

        // when
        let tokens = parser.collect();

        // then
        assert_eq!(tokens.size(), 6);
        assert_eq!(tokens.get(0), &Token::AppName);
        assert_eq!(tokens.get(1), &Token::Argument(ArgumentType::Int));
        assert_eq!(tokens.get(2), &Token::IntValue(42));
        assert_eq!(tokens.get(3), &Token::Argument(ArgumentType::Bool));
        assert_eq!(tokens.get(4), &Token::Argument(ArgumentType::Str));
        assert_eq!(tokens.get(5), &Token::StrValue("string".to_string()));
    }
}

