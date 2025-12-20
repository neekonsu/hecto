use std::io::{stdout, Write};
use crossterm::{execute, queue};
use crossterm::cursor::{Hide, Show, MoveTo}; 
use crossterm::terminal::{Clear, ClearType, enable_raw_mode, disable_raw_mode, size};
use crossterm::style::Print;

pub struct Terminal {
    x: u16,
    y: u16
}

impl Terminal {
    // Enter raw mode, clear screen, move cursor 0,0
    pub fn initialize() -> Result<(), std::io::Error> {
        enable_raw_mode()?;
        Self::clear_screen()?;
        Self::cursor_home()?;
        Ok(())
    }
    // Clear screen
    pub fn clear_screen() -> Result<(), std::io::Error> {
        execute!(stdout(), Clear(ClearType::All))?;
        Ok(())
    }
    // Draw tildes, consider renaming
    pub fn draw_rows() -> Result<(), std::io::Error> {
        let rows = size()?.1;
        queue!(stdout(), Clear(ClearType::All), Hide)?;
        for r in 0..rows {
            queue!(stdout(), MoveTo(0,r), Print("~".to_string()))?;
        }
        queue!(stdout(), Show)?;
        stdout().flush()?;
        Ok(())
    }
    // Move cursor 0,0
    pub fn cursor_home() -> Result<(), std::io::Error> {
        execute!(stdout(), MoveTo(0,0))?;
        Ok(())
    }
    // Exit Raw Mode
    pub fn terminate() -> Result<(), std::io::Error> {
        disable_raw_mode()?;
        Ok(())
    }
}
