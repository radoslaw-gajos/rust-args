struct App {
}

impl App {
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
