use crate::token::parser::TokenParser;
use crate::token::tokens::Tokens;
use crate::schema::Schema;
use std::collections::HashMap;

#[derive(Default)]
struct Collection {
    schema: Schema,
    ints: HashMap<String,i64>,
    strings: HashMap<String,String>,
    bools: HashMap<String,bool>,
}

impl Collection {
    fn from(tokens: Tokens) -> Collection {
        let mut collection = Collection::default();
        collection.schema = (*tokens.schema().expect("Schema expected")).clone();
        collection
    }

    fn get_int(&self, key: &str) -> i64 {
        todo!();
    }

    fn get_str(&self, key: &str) -> String {
        todo!();
    }

    fn get_bool(&self, key: &str) -> bool {
        if self.bools.contains_key(key) {
            return *self.bools.get(key).unwrap();
        }
        if self.schema.get(key.chars().nth(0).expect("Valid string expected")).is_some() {
            return false;
        }
        panic!("Key not found in schema!");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_get_values_from_collection() {
        // given
        let schema = Schema::from(vec![
            ("i".to_string(), "int".to_string()),
            ("s".to_string(), "string".to_string()),
            ("b".to_string(), "bool".to_string()),
        ]);
        let parser = TokenParser::new()
            .args(vec!["app_name", "-i", "42", "-b", "-s", "string"])
            .schema(schema);
        let tokens = parser.collect();

        // when
        let collection = Collection::from(tokens);

        // then
        assert_eq!(collection.get_int("i"), 42);
        assert_eq!(collection.get_str("s"), "string");
        assert!(collection.get_bool("b"));
    }

    #[test]
    fn should_get_false_when_flag_not_set() {
        // given
        let schema = Schema::from(vec![
            ("b".to_string(), "bool".to_string()),
        ]);
        let parser = TokenParser::new()
            .args(vec!["app_name"])
            .schema(schema);
        let tokens = parser.collect();

        // when
        let collection = Collection::from(tokens);

        // then
        assert!(!collection.get_bool("b"));
    }
}
