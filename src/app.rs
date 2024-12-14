use crate::collection::Collection;
use crate::schema::Schema;

struct App {
    collection: Collection,
}

impl App {
    pub fn new(args: Vec<String>) -> Self {
        Self {
            collection: collection_from_args(args),
        }
    }

    pub fn run(&self) {
        self.init();
        let col = self.get_collection();
    }

    fn init(&self) {
    }

    fn get_collection(&self) -> &Collection {
        &self.collection
    }
}

fn collection_from_args(args: Vec<String>) -> Collection {
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

}
