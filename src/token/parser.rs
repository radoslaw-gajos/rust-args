use crate::schema::Schema;
use crate::token::tokens::Tokens;
use crate::token::parser::strategy::ParserStrategy;

#[derive(Default)]
pub struct TokenParser {
    args: Vec<String>,
    index: usize,
    schema: Option<Schema>,
    strategy: Box<dyn ParserStrategy>,
    tokens: Tokens,
}

mod strategy;

impl TokenParser {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn args(self, args: Vec<&str>) -> Self {
        Self {
            args: args.into_iter().map(String::from).collect(),
            ..self
        }
    }

    pub fn schema(self, schema: Schema) -> Self {
        Self {
            schema: Some(schema),
            ..self
        }
    }

    pub fn collect(self) -> Tokens {
        let mut parser = self;
        while !parser.is_done() {
            parser = parser.parse_current();
            parser.next();
        }
        if parser.schema.is_some() {
            parser.tokens.schema_set(parser.schema.unwrap());
        }

        parser.tokens
    }

    pub fn set_strategy(&mut self, strategy: Box<dyn ParserStrategy>) {
        self.strategy = strategy;
    }

    fn is_done(&self) -> bool {
        self.index >= self.args.len()
    }

    fn parse_current(self) -> Self {
        let strategy = self.strategy.clone();
        strategy.parse(self)
    }

    fn next(&mut self) {
        self.index += 1;
    }
    
    fn current_arg(&self) -> &str {
        self.args.get(self.index).expect("Index should be valid")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::token::Token;
    use crate::schema::argument::ArgumentType;

    #[test]
    fn should_return_empty_token_collection_when_no_args() {
        // given
        let parser = TokenParser::new();

        // when
        let tokens = parser.collect();
        
        // then
        assert_eq!(tokens.size(), 0);
    }

    #[test]
    fn should_interpret_first_arg_as_appname() {
        // given
        let parser = TokenParser::new()
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
        let parser = TokenParser::new()
            .args(vec!["app_name", "-b"])
            .schema(schema);

        // when
        let tokens = parser.collect();

        // then
        assert_eq!(tokens.size(), 2);
        assert_eq!(tokens.get(1), &Token::Argument(ArgumentType::Bool, "b".to_string()));
    }

    #[test]
    fn should_get_string_argument() {
        // given
        let schema = Schema::from(vec![
            ("s".to_string(), "string".to_string()),
        ]);
        let parser = TokenParser::new()
            .args(vec!["app_name", "-s", "string"])
            .schema(schema);

        // when
        let tokens = parser.collect();

        // then
        assert_eq!(tokens.size(), 3);
        assert_eq!(tokens.get(1), &Token::Argument(ArgumentType::Str, "s".to_string()));
        assert_eq!(tokens.get(2), &Token::StrValue("string".to_string()));
    }

    #[test]
    fn should_get_int_argument() {
        // given
        let schema = Schema::from(vec![
            ("i".to_string(), "int".to_string()),
        ]);
        let parser = TokenParser::new()
            .args(vec!["app_name", "-i", "42"])
            .schema(schema);

        // when
        let tokens = parser.collect();

        // then
        assert_eq!(tokens.size(), 3);
        assert_eq!(tokens.get(1), &Token::Argument(ArgumentType::Int, "i".to_string()));
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
        let parser = TokenParser::new()
            .args(vec!["app_name", "-i", "42", "-b", "-s", "string"])
            .schema(schema);

        // when
        let tokens = parser.collect();

        // then
        assert_eq!(tokens.size(), 6);
        assert_eq!(tokens.get(0), &Token::AppName);
        assert_eq!(tokens.get(1), &Token::Argument(ArgumentType::Int, "i".to_string()));
        assert_eq!(tokens.get(2), &Token::IntValue(42));
        assert_eq!(tokens.get(3), &Token::Argument(ArgumentType::Bool, "b".to_string()));
        assert_eq!(tokens.get(4), &Token::Argument(ArgumentType::Str, "s".to_string()));
        assert_eq!(tokens.get(5), &Token::StrValue("string".to_string()));
    }
}

