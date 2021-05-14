
pub struct App {
    quit: bool,
}

impl App {
    pub fn new() -> App {
        App {
            quit: false,
        }
    }

    pub fn request_exit(&mut self) {
        self.quit = true;
    }

    pub fn should_exit(&self) -> bool {
        self.quit
    }
}
