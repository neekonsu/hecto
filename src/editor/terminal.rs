use std::io::{stdout, Write, Error};
use crossterm::queue;
use crossterm::cursor::{Hide, Show, MoveTo}; 
use crossterm::terminal::{Clear, ClearType, enable_raw_mode, disable_raw_mode, size};
use crossterm::style::Print;

#[derive(Copy, Clone)]
pub struct Position {
    pub x: u16,
    pub y: u16,
}


#[derive(Copy, Clone)]
pub struct Size {
    pub width: u16,
    pub height: u16,
}


pub struct Terminal;

impl Terminal {
    // Enter raw mode, clear screen, move cursor 0,0
    pub fn initialize() -> Result<(), Error> {
        enable_raw_mode()?;
        Self::clear_screen()?;
        Self::move_cursor(Position{x:0,y:0})?;
        Self::execute()?;
        Ok(())
    }
    pub fn show() -> Result<(), Error> {
        queue!(stdout(), Show)?;
        Ok(())
    }
    pub fn hide() -> Result<(), Error> {
        queue!(stdout(), Hide)?;
        Ok(())
    }
    pub fn clear_line() -> Result<(), Error> {
        queue!(stdout(), Clear(ClearType::CurrentLine))?;
        Ok(())
    }
    pub fn clear_screen() -> Result<(), Error> {
        queue!(stdout(), Clear(ClearType::All))?;
        Ok(())
    }
    pub fn size() -> Result<Size, Error> {
        let (w, h) = size()?;
        Ok(Size{width:w, height:h})
    }
    pub fn move_cursor(pos: Position) -> Result<(), Error> {
        queue!(stdout(), MoveTo(pos.x, pos.y))?;
        Ok(())
    }
    pub fn print(string: &str) -> Result<(), Error> {
        queue!(stdout(), Print(string))?;
        Ok(())
    }
    pub fn execute() -> Result<(), Error> {
        stdout().flush()?;
        Ok(())
    }
    // Exit Raw Mode
    pub fn terminate() -> Result<(), Error> {
        Self::execute()?;
        disable_raw_mode()?;
        Ok(())
    }
}
