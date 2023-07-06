use std::io;

use anyhow::Result;
use crossterm::{
    event::{self, Event, KeyCode},
    terminal::{disable_raw_mode, enable_raw_mode},
};
use menu::MenuView;
use ratatui::{backend::CrosstermBackend, Terminal};

mod menu;

fn main() -> Result<()> {
    enable_raw_mode()?;

    let backend = CrosstermBackend::new(io::stdout());
    let mut terminal = Terminal::new(backend)?;

    terminal.clear()?;

    let items = "Lorem ipsum dolor sit amet, consectetur adipiscing elit. Vestibulum pretium ullamcorper lectus nec varius. Vestibulum ante ipsum primis in faucibus orci luctus et ultrices posuere cubilia curae; Nunc et mi quis ligula imperdiet vehicula ac quis metus. Etiam sodales condimentum turpis, at scelerisque neque semper et. Sed nec diam elementum, volutpat odio eget, feugiat est. Duis non mollis velit. Praesent eu risus mauris. Aliquam vestibulum enim at nulla dictum viverra non sit amet mi. Quisque laoreet diam justo, a cursus dolor maximus et. Donec tristique fringilla elit, id viverra arcu tempus tempus. Mauris rhoncus congue diam et ultricies. ".split(' ').map(String::from).collect::<Vec<_>>();

    let mut menu = MenuView::new(items);

    loop {
        terminal.draw(|f| menu.render(f, f.size()))?;

        if let Event::Key(e) = event::read()? {
            match e.code {
                KeyCode::Char('q') => {
                    break;
                }
                _ => menu.on_event(e),
            }
        }
    }

    disable_raw_mode()?;
    terminal.clear()?;

    Ok(())
}
