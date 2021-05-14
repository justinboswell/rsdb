use std::io;
use termion::event::Key;
use termion::input::TermRead;
use crate::{
    app::App,
};

pub fn read_input(app: &mut App) {
    let stdin = io::stdin();
    for k in stdin.keys() {
        if let Ok(key) = k {
            if key == Key::Char('q') {
                app.request_exit();
                break;
            }
        }
    }
}
