use crate::token::Token;

#[derive(Default)]
pub struct Tokens {
    items: Vec<Token>,
}

impl Tokens {
    pub fn new() -> Self {
        Self {
            items: Vec::new(),
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
}
