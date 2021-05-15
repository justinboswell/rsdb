use std::io;
use termion::raw::IntoRawMode;
use tui::Terminal;
use tui::backend::TermionBackend;

mod ui;
mod app;
mod input;

use app::App;

fn main() -> Result<(), io::Error> {
    let stdout = io::stdout().into_raw_mode()?;
    let backend = TermionBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    let mut app = App::new();

    terminal.clear()?;
    loop {
        terminal.draw(|frame| ui::draw(frame, &app))?;
        input::read_input(&mut app);
        if app.should_exit() {
            break;
        }
    }
    
    Ok(())
}
