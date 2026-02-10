use anyhow::Result;
use crossterm::cursor::Show;
use crossterm::execute;
use crossterm::terminal::{
    disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen,
};
use std::io::{self, Stdout};

pub struct TerminalGuard {
    stdout: Stdout,
}

impl TerminalGuard {
    pub fn enter() -> Result<Self> {
        enable_raw_mode()?;
        let mut stdout = io::stdout();
        if let Err(err) = execute!(stdout, EnterAlternateScreen) {
            // If we fail after enabling raw mode, make a best effort to restore the terminal.
            let _ = disable_raw_mode();
            return Err(err.into());
        }
        Ok(Self { stdout })
    }

    pub fn stdout(&mut self) -> &mut Stdout {
        &mut self.stdout
    }
}

impl Drop for TerminalGuard {
    fn drop(&mut self) {
        // Best-effort restore for happy path and error paths. Ordering is intentional:
        // 1) show cursor (ratatui may hide it during draws)
        // 2) leave alternate screen
        // 3) disable raw mode
        let _ = execute!(self.stdout, Show, LeaveAlternateScreen);
        let _ = disable_raw_mode();
    }
}
