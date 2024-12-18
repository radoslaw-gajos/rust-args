use crate::collection::Collection;
use crate::schema::Schema;

pub struct App {
    collection: Collection,
}

impl App {
    pub fn new(args: Vec<String>) -> Self {
        Self {
            collection: collection_from_args(args.iter().map(|x| x.as_str()).collect()),
        }
    }

    pub fn run(&self) {
        let col = self.get_collection();

        let string = col.get_str("s").unwrap_or("");
        let int = col.get_int("i").unwrap_or(0);
        let b = col.get_bool("b");

        println!("String: {string}");
        println!("Int: {int}");
        println!("Bool: {b}");
    }

    fn get_collection(&self) -> &Collection {
        &self.collection
    }
}

fn collection_from_args(args: Vec<&str>) -> Collection {
    Collection::from_args(args, get_schema())
}

fn get_schema() -> Schema {
    Schema::from(vec![
        ("s".to_string(), "string".to_string()),
        ("b".to_string(), "bool".to_string()),
        ("i".to_string(), "int".to_string()),
    ])
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_return_false_by_default() {
        // given
        let args = vec!["app_name".to_string()];

        // when
        let app = App::new(args);

        // then
        assert!(!app.collection.get_bool("b"))
    }

    #[test]
    fn should_return_true_when_flag_set() {
        // given
        let args = vec!["app_name".to_string(), "-b".to_string()];

        // when
        let app = App::new(args);

        // then
        assert!(app.collection.get_bool("b"))
    }

    #[test]
    fn should_return_none_when_string_not_set() {
        // given
        let args = vec!["app_name".to_string()];

        // when
        let app = App::new(args);

        // then
        assert_eq!(app.collection.get_str("s"), None)
    }

    #[test]
    fn should_return_string() {
        // given
        let args = vec!["app_name".to_string(), "-s".to_string(), "foo".to_string()];

        // when
        let app = App::new(args);

        // then
        assert_eq!(app.collection.get_str("s"), Some("foo"))
    }

    #[test]
    fn should_return_none_when_int_not_set() {
        // given
        let args = vec!["app_name".to_string()];

        // when
        let app = App::new(args);

        // then
        assert_eq!(app.collection.get_int("i"), None)
    }

    #[test]
    fn should_return_int() {
        // given
        let args = vec!["app_name".to_string(), "-i".to_string(), "-42".to_string()];

        // when
        let app = App::new(args);

        // then
        assert_eq!(app.collection.get_int("i"), Some(-42))
    }
}
