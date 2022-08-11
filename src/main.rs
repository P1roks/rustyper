mod app;
mod readfile;
mod uis;

use crate::app::App;
use crate::uis::render_ui;
use std::{error::Error, io};

use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use tui::{
    backend::{Backend, CrosstermBackend},
    Terminal,
};

fn main() -> Result<(), Box<dyn Error>> {
    //setup Terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let res = run_app(&mut terminal);

    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    if let Err(err) = res {
        print!("{:?}", err);
    }

    Ok(())
}

fn run_app<B: Backend>(term: &mut Terminal<B>) -> Result<(), Box<dyn Error>> {
    let mut msg = App::new(crate::readfile::get_words(20, "words/polish")?);

    let len = msg.text.chars().count() - 1;

    loop {
        term.draw(|f| render_ui(f, &msg))?;
        if !msg.is_complete() {
            if let Event::Key(key) = event::read()? {
                let chr = msg.text.chars().nth(msg.index).unwrap_or_default();
                match key.code {
                    KeyCode::Char(c) => {
                        if c == chr {
                            msg.misspells[msg.index] = true;
                        } else {
                            msg.mistakes += 1;
                        }
                        msg.typed += 1;
                        msg.index += 1;
                    }
                    KeyCode::Backspace => {
                        msg.index -= if msg.index > 0 { 1 } else { 0 };
                        msg.misspells[msg.index] = false;
                    }
                    KeyCode::Esc => return Ok(()),
                    _ => {}
                }
            }

            //String ends with space
            if msg.index == len {
                msg.stop_timer();
                msg.completed();
            }
        } else {
            if let Event::Key(key) = event::read()? {
                if let KeyCode::Esc = key.code {
                    return Ok(());
                }
            }
        }
    }
}
