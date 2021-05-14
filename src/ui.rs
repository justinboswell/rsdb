
use tui:: {
    backend::Backend,
    Frame,
    widgets::{
        Block,
        Borders,
    }
};

use crate::{
    app::App,
};

pub fn draw<B: Backend>(frame: &mut Frame<B>, _app: &App) {
    let size = frame.size();
    let block = Block::default()
        .title("RSDB")
        .borders(Borders::ALL);
    frame.render_widget(block, size);
}
