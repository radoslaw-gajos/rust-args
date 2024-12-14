use crate::collection::Collection;

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

    fn get_collection(&self) -> Collection {
        self.collection
    }
}

fn collection_from_args(args: Vec<String>) -> Collection {
    Collection::from_args(args, get_schema())
}

fn get_schema() -> Schema {
    todo!();
}

#[cfg(test)]
mod tests {
    use super::*;
}
