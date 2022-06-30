use crossterm::{
    self, cursor,
    style::{self, Stylize},
    terminal, QueueableCommand, Result,
};
use std::io::{stdout, Write};

fn main() -> Result<()> {
    let mut stdout = stdout();

    stdout.queue(terminal::Clear(terminal::ClearType::All));

    for i in 0..30 {
        for j in 0..30 {
            stdout
                .queue(cursor::MoveTo(i, j))?
                .queue(style::PrintStyledContent("#".red()))?;
        }
    }

    stdout.flush()?;

    // crossterm::
    // println!("Hello, world!");

    Ok(())
}
