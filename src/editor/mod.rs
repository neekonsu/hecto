mod terminal;
use std::io::{self, Write};
use terminal::Terminal;
use crossterm::event::{read, Event, Event::Key, KeyCode::Char, KeyEvent, KeyModifiers};

pub struct Editor {
    should_quit: bool,
    stdout: std::io::Stdout,
}

impl Editor {
    pub const fn default() -> Self {
        Self { 
            should_quit: false,
            stdout: io::stdout(),
        }
    }
    pub fn run(&mut self) {
        Terminal::initialize(&self.stdout).unwrap();
        let result = self.repl();
        Terminal::terminate(&self.stdout).unwrap();
        result.unwrap();
    }
    fn repl(&mut self) -> Result<(), std::io::Error> {
        loop {
            self.refresh_screen()?;
            if self.should_quit {
                break;
            }
            let event = read()?;
            self.evaluate_event(&event);
        }
        Ok(())
    }
    fn refresh_screen(&self) -> Result<(), std::io::Error> { 
        if self.should_quit {
            Terminal::clear_screen(&self.stdout)?;
            print!("Goodbye.\r\n");
        } else {
            Terminal::draw_rows(&self.stdout)?;
            Terminal::cursor_home(&self.stdout)?;
        }
        Ok(())
    }
    fn evaluate_event(&mut self, event: &Event) {
        if let Key(KeyEvent {code, modifiers, ..}) = event {
            match code {
                Char('q') if *modifiers == KeyModifiers::CONTROL => {
                    self.should_quit = true;
                }
                _ => (),
            }
        }
    }
}

