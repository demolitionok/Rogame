mod game;
mod common;
use common::settings::{WithSettings, Settings};

fn main() {
    let mut s = Settings::from_file(String::from("C:\\Users\\georg\\Desktop\\Pipka\\rogualike\\config\\config.json"));
    game::menu::Menu::run(&mut s);
    println!("{:?}", s);
}
