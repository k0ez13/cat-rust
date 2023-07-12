use std::io::{stdout, Write};
use std::thread;
use std::time::Duration;
use crossterm::{
    cursor,
    execute,
    style::{Color, Print, ResetColor, SetForegroundColor},
    terminal::{enable_raw_mode, Clear, ClearType},
    ExecutableCommand,
};

fn main() -> crossterm::Result<()> {
    enable_raw_mode()?;

    let mut stdout = stdout();
    execute!(stdout, Clear(ClearType::All))?;

    let total_steps = 30;
    for step in 0..=total_steps {
        let progress_bar = "=".repeat(step) + &" ".repeat(total_steps - step);

        stdout.execute(cursor::MoveTo(0, 0))?;
        stdout.execute(SetForegroundColor(Color::Yellow))?;
        stdout.execute(Print(format!("[{}] {}%", progress_bar, step * 100 / total_steps)))?;
        stdout.execute(ResetColor)?;
        stdout.flush()?;

        thread::sleep(Duration::from_millis(100));
    }

    Ok(())
}