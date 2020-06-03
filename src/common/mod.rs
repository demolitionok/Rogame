pub mod button;
pub mod settings;

use sfml::graphics::{Transformable, Text};

pub trait CenterText {
    fn center_text(&mut self, other: &impl Transformable);
}

impl<'a> CenterText for Text<'a> {
    fn center_text(&mut self, other: &impl Transformable) {
        self.set_position(other.position());
    }
}