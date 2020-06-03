pub struct Settings {}

impl Settings {
    pub fn new() -> Self {
        Settings {}
    }
}

pub trait WithSettings {
    fn run(settings : &mut Settings);
}