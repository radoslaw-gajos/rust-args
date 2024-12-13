struct App {
}

impl App {
    pub fn new(args: Vec<String>) -> Self {
        Self {
        }
    }

    pub fn run(&self) {
        self.init();
        let col = self.get_collection();
    }

    fn init(&self) {
    }

    fn get_collection(&self) -> Collection {
    }
}

#[cfg(test)]
mod tests {
    use super::*;
}
