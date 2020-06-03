use rand::{rngs::ThreadRng, Rng};
use sfml::{graphics::*, system::*, window::*};
use crate::common::settings::{WithSettings, Settings};

const WINDOW_WIDTH: u32 = 800;
const WINDOW_HEIGHT: u32 = 600;

pub struct Menu {
}

impl WithSettings for Menu {
    fn run(set: &mut Settings) {}
}