use std::time::Duration;

use crossterm::{
    event::{self, Event, KeyCode, KeyEvent}, 
    terminal
};


struct CleanUp;

impl Drop for CleanUp {
    fn drop(&mut self) {
        terminal::disable_raw_mode().expect("Could not turn off raw mode");
    }
}

fn main() -> crossterm::Result<()> {
    let _clean_up = CleanUp;
    terminal::enable_raw_mode()?;
    loop {
        if event::poll(Duration::from_millis(500))? {
            if let Event::Key(event) = event::read()? {
                match event {
                    KeyEvent {
                        code: KeyCode::Char('q'), 
                        modifiers: event::KeyModifiers::CONTROL,
                    } => break,
                    _ => {

                    }
                }
                println!("{:?}\r", event);
            }
        } else {
            println!("No input yet\r");
        }
    }
    Ok(())
}
