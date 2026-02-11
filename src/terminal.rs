use anyhow::Result;
use crossterm::cursor::Show;
use crossterm::event::{DisableMouseCapture, EnableMouseCapture};
use crossterm::execute;
use crossterm::terminal::{
    disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen,
};
use std::io::{self, Stdout};

pub struct TerminalGuard {
    stdout: Stdout,
    mouse_enabled: bool,
}

impl TerminalGuard {
    pub fn enter(mouse_enabled: bool) -> Result<Self> {
        enable_raw_mode()?;
        let mut stdout = io::stdout();
        if let Err(err) = execute!(stdout, EnterAlternateScreen) {
            // If we fail after enabling raw mode, make a best effort to restore the terminal.
            let _ = disable_raw_mode();
            return Err(err.into());
        }
        if mouse_enabled {
            if let Err(err) = execute!(stdout, EnableMouseCapture) {
                let _ = execute!(stdout, LeaveAlternateScreen);
                let _ = disable_raw_mode();
                return Err(err.into());
            }
        }
        Ok(Self {
            stdout,
            mouse_enabled,
        })
    }

    pub fn stdout(&mut self) -> &mut Stdout {
        &mut self.stdout
    }
}

impl Drop for TerminalGuard {
    fn drop(&mut self) {
        // Best-effort restore for happy path and error paths. Ordering is intentional:
        // 1) show cursor (ratatui may hide it during draws)
        // 2) disable mouse capture (when enabled)
        // 3) leave alternate screen
        // 4) disable raw mode
        if self.mouse_enabled {
            let _ = execute!(self.stdout, Show, DisableMouseCapture, LeaveAlternateScreen);
        } else {
            let _ = execute!(self.stdout, Show, LeaveAlternateScreen);
        }
        let _ = disable_raw_mode();
    }
}
