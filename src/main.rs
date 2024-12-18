#![warn(clippy::all)]

use args::app::App;
use std::env;

fn main() {
    App::new(get_args()).run();
}

fn get_args() -> Vec<String> {
    env::args().collect()
}
