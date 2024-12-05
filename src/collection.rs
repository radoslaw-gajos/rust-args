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
        assert!(collection.get_str("b"));
    }
}
