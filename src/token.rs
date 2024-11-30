use crate::schema::argument::ArgumentType;
use crate::schema::{self, Schema};
use dyn_clone::{clone_trait_object, DynClone};

#[derive(Debug, PartialEq)]
pub enum Token {
    Argument(ArgumentType),
    StrValue(String),
    IntValue(i64),
    BoolValue(bool),
}

#[derive(Default)]
pub struct Tokens {
    items: Vec<Token>,
}

impl Tokens {
    fn new() -> Self {
        Self {
            items: Vec::new(),
        }
    }

    fn add(&mut self, token: Token) {
        self.items.push(token);
    }

    fn size(&self) -> usize {
        self.items.len()
    }
}

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
struct ArgumentParser;

#[derive(Clone)]
struct StrParser;

#[derive(Clone)]
struct IntParser;

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

        parser
    }
}

impl ParserStrategy for StrParser {
    fn parse(&self, mut parser: TokenParser) -> TokenParser {
        parser
    }
}

impl ParserStrategy for IntParser {
    fn parse(&self, mut parser: TokenParser) -> TokenParser {
        parser
    }
}

impl Default for Box<dyn ParserStrategy> {
    fn default() -> Self {
        Box::new(ArgumentParser)
    }
}

impl TokenParser {
    fn new() -> Self {
        let mut parser = Self::default();
        parser.index = 1; // ignore app name
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
    fn should_ignore_first_argument() {
        // given
        let mut parser = TokenParser::new()
            .args(vec!["app_name"]);

        // when
        let tokens = parser.collect();
        
        // then
        assert_eq!(tokens.size(), 0);
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
        assert_eq!(tokens.size(), 1);
    }
}
