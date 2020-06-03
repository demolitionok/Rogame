mod game;
mod common;
use common::settings::{WithSettings, Settings};

fn main() {
    let mut s = Settings::new();
    game::menu::Menu::run(&mut s);
    println!("Hello, world!");
}
