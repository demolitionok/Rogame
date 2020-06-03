use rand::{rngs::ThreadRng, Rng};
use sfml::{graphics::*, system::*, window::*};
use super::CenterText;

type OnClickCallback<T> = Box<dyn Fn(&mut T)>;

struct GameButton<'a, T> {
    button_shape: RectangleShape<'a>,
    text: String,
    font: SfBox<Font>,
    on_click: OnClickCallback<T>,
}

impl<'s, T> Drawable for GameButton<'s, T> {
    fn draw<'a: 'shader, 'texture, 'shader, 'shader_texture>(
        &'a self,
        target: &mut dyn RenderTarget,
        states: RenderStates<'texture, 'shader, 'shader_texture>,
    ) {
        self.button_shape.draw(target, states);
        self.button_text().draw(target, states);
    }
}

impl<'a, T> GameButton<'a, T> {
    fn button_text(&self) -> Text {
        let mut t = Text::new(&self.text, &self.font, 14);
        t.center_text(&self.button_shape);
        t.set_fill_color(Color::RED);
        t
    }
    fn new(
        position: impl Into<Vector2f>,
        size: impl Into<Vector2f>,
        text: String,
        on_click: OnClickCallback<T>,
    ) -> Self {
        let mut button_shape = RectangleShape::with_size(size.into());
        button_shape.set_position(position.into());
        let font = Font::from_file("C:\\Windows\\Fonts\\Arial.ttf").unwrap();
        GameButton {
            button_shape,
            text,
            font,
            on_click,
        }
    }
}