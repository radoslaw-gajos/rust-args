use crate::token::parser::TokenParser;
use crate::token::tokens::Tokens;
use crate::schema::Schema;

struct Collection {
    ints: Vec<i64>,
    strings: Vec<String>,
    bools: Vec<bool>,
}

impl Collection {
    fn from(tokens: Tokens) -> Collection {
        todo!();
    }

    fn get_int(&self, key: &str) -> i64 {
        todo!();
    }

    fn get_str(&self, key: &str) -> String {
        todo!();
    }

    fn get_bool(&self, key: &str) -> bool {
        todo!();
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
}
