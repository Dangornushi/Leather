use crossterm::{
    event::{DisableMouseCapture, EnableMouseCapture},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{backend::CrosstermBackend, Terminal};
use std::{error::Error, io};

use leather::run_app;
use leather::App;

fn get_terminal() -> Option<Terminal<CrosstermBackend<io::Stdout>>> {
    execute!(io::stdout(), EnterAlternateScreen, EnableMouseCapture).ok()?;
    return Some(Terminal::new(CrosstermBackend::new(io::stdout())).ok()?);
}

fn main() -> Result<(), Box<dyn Error>> {
    // setup terminal
    enable_raw_mode()?;

    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // create app and run it
    let app = App::default(get_terminal().unwrap());
    let res = run_app(&mut get_terminal().unwrap(), app);

    // restore terminal
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;
    if let Err(err) = res {
        println!("{:?}", err)
    }

    Ok(())
}
