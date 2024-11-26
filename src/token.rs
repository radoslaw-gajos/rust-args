use crate::schema::argument::ArgumentType;
use crate::schema::{self, Schema};
use std::rc::Rc;

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
    schema: Schema,
    strategy: Rc<Box<dyn ParserStrategy>>,
    tokens: Tokens,
}

trait ParserStrategy {
    fn parse(&self, parser: TokenParser) -> TokenParser;
}

struct ArgumentParser;

impl ParserStrategy for ArgumentParser {
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
        Self::default()
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
        self.tokens
    }

    fn set_strategy(&mut self, strategy: Box<dyn ParserStrategy>) {
        *self.strategy.borrow_mut() = strategy;
    }

    fn parse_current(mut self) -> Self {
        let strategy = self.strategy.borrow();
        strategy.parse(self)
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
