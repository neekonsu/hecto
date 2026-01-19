mod terminal;
use terminal::{Terminal, Size, Position};
use std::io::Error;
use crossterm::event::{read, Event, Event::Key, KeyCode::Char, KeyEvent, KeyModifiers};

pub struct Editor {
    should_quit: bool,
    cursor_pos: Position,
}

impl Editor {
    pub const fn default() -> Self {
        Self { 
            should_quit: false,
            cursor_pos: Position{x:0,y:0},
        }
    }
    pub fn run(&mut self) {
        Terminal::initialize().unwrap();
        let result = self.repl();
        Terminal::terminate().unwrap();
        result.unwrap();
    }
    fn repl(&mut self) -> Result<(), Error> {
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
    fn refresh_screen(&mut self) -> Result<(), Error> { 
        Terminal::hide()?;
        if self.should_quit {
            Terminal::clear_screen()?;
            Terminal::print("Goodbye.\r\n")?;
        } else {
            Self::draw_rows()?;
            Terminal::move_cursor(Position{x:0, y:0})?;
        }
        Terminal::show()?;
        Terminal::execute()?;
        Ok(())
    }
    fn evaluate_event(&mut self, event: &Event) -> Result<(), Error> {
        if let Key(KeyEvent {code, modifiers, ..}) = event {
            match code {
                Char('q') if *modifiers == KeyModifiers::CONTROL => {
                    self.should_quit = true;
                    Ok(())
                }
                Char('j' | 'k') => {
                    let diff_pos_y: u16 = 2*(code.as_char() - 'j')-1;
                    self.cursor_pos.y += diff_pos_y;
                    Terminal::move_cursor(self.cursor_pos)
                }
                Char('h' | 'l') => {
                    let diff_pos_x: u16 = (code.as_char() - 'i')/2;
                    self.cursor_pos.x += diff_pos_x;
                    Terminal::move_cursor(self.cursor_pos)
                }
                _ => ()
            }
        }
    }
    fn draw_rows() -> Result<(), Error> {
        let  Size{height, ..} = Terminal::size()?;
        Terminal::clear_screen()?;
        for r in 0..height {
            Terminal::print("~")?;
            Terminal::move_cursor(Position{x:0, y:r})?;
        }
        Ok(())
    }
}

