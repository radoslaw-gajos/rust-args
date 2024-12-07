use crate::token::Token;
use crate::schema::Schema;

#[derive(Default)]
pub struct Tokens {
    items: Vec<Token>,
    schema: Option<Schema>,
    index: usize,
}

impl Tokens {
    pub fn new(schema: Schema) -> Self {
        Self {
            schema: Some(schema),
            items: Vec::new(),
            index: 0,
        }
    }

    pub fn add(&mut self, token: Token) {
        self.items.push(token);
    }

    pub fn size(&self) -> usize {
        self.items.len()
    }

    pub fn get(&self, index: usize) -> &Token {
        self.items.get(index).expect("Valid index expected")
    }

    pub fn schema(&self) -> Option<&Schema> {
        self.schema.as_ref()
    }

    pub fn schema_set(&mut self, schema: Schema) {
        self.schema = Some(schema);
    }

    pub fn current(&self) -> Option<&Token> {
        self.items.get(self.index)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_get_none_if_no_more_tokens() {
        // given
        let tokens = Tokens::default();

        // when
        let current = tokens.current();

        // then
        assert_eq!(current, None);
    }

    #[test]
    fn should_return_first_token() {
        // given
        let mut tokens = Tokens::default();
        tokens.add(Token::AppName);

        // when
        let current = tokens.current();

        // then
        assert_eq!(current, Some(Token::AppName).as_ref());
    }
}
