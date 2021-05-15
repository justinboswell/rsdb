use std::io;
use termion::raw::IntoRawMode;
use termion::event::Key;
use termion::input::TermRead;
use tui::Terminal;
use tui::backend::TermionBackend;
use tui::widgets::{Widget, Block, Borders};
use tui::layout::{Layout, Constraint, Direction};

fn main() -> Result<(), io::Error> {
    let stdout = io::stdout().into_raw_mode()?;
    let backend = TermionBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    let mut quit = false;
    terminal.clear()?;
    loop {
        terminal.draw(|frame| {
            let size = frame.size();
            let block = Block::default()
                .title("RSDB")
                .borders(Borders::ALL);
            frame.render_widget(block, size);
        })?;
        let stdin = io::stdin();
        for k in stdin.keys() {
            if let Ok(key) = k {
                if key == Key::Char('q') {
                    quit = true;
                    break;
                }
            }
        }
        if quit {
            break;
        }
    }
    
    Ok(())
}
