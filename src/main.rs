use std::io;
use std::thread;
use std::time::Duration;
use crossterm::{
    cursor,
    execute,
    style::{Color, Print, ResetColor, SetForegroundColor},
    terminal::{enable_raw_mode, Clear, ClearType},
    ExecutableCommand,
};
use std::io::Write;

fn main() -> io::Result<()> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, Clear(ClearType::All))?;

    let frames = vec![
        r#"
/\_/\
           ( o.o )
                     >   ^ <"#,
        r#"
/\_/\
           ( o.o )
                      >  ^ <"#,
        r#"
/\_/\
           ( o.o )
                       > ^ <"#,
        r#"
/\_/\
           ( o.o )
                      >  ^ <"#,
    ];

    animate(&mut stdout, &frames, Duration::from_millis(250))
}

fn animate<W: Write>(stdout: &mut W, frames: &[&str], delay: Duration) -> io::Result<()> {
    loop {
        for frame in frames {
            stdout.execute(cursor::MoveTo(0, 0))?;
            stdout.execute(ResetColor)?;
            stdout.execute(Clear(ClearType::All))?;
            stdout.execute(SetForegroundColor(Color::Yellow))?;
            stdout.execute(Print(frame))?;
            stdout.flush()?;

            thread::sleep(delay);
        }
    }
}
