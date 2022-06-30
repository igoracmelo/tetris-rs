use crossterm::{
    self, cursor,
    event::{poll, read, Event},
    queue,
    style::{self, Print, StyledContent, Stylize},
    terminal, ExecutableCommand, QueueableCommand, Result,
};
use std::{
    io::{stdout, Stdout, Write},
    thread,
    time::Duration,
};

fn handle_events() -> Result<()> {
    loop {
        if poll(Duration::from_millis(500))? {
            match read()? {
                Event::Key(event) => println!("{:?}", event),
                _ => {}
            }
        }
    }
    // Ok(())
}

enum Tile {
    Empty,
    Wall,
    Line,
    Square,
    S1,
    S2,
    L1,
    L2,
}

struct Game {
    width: u16,
    height: u16,
    offset_x: u16,
    offset_y: u16,
    // tiles: Vec<Tile>,
    screen: Vec<Vec<Tile>>,
    stdout: Stdout,
}

impl Game {
    fn new(width: u16, height: u16, stdout: Stdout) -> Self {
        let size = terminal::size().unwrap();
        let offset_x = size.0 / 2 - width / 2;
        let offset_y = size.1 / 2 - height / 2;

        let mut screen = Vec::new();
        for _ in 0..height {
            // let row = screen[l];
            let mut row = Vec::new();
            for _ in 0..width {
                row.push(Tile::Empty);
            }
            screen.push(row);
        }

        Game {
            width,
            height,
            offset_x,
            offset_y,
            stdout,
            screen,
        }
    }
    // fn ___init_screen(&mut self) {
    //     for _ in 0..self.height {
    //         // let row = screen[l];
    //         let mut row = Vec::new();
    //         for _ in 0..self.width {
    //             row.push(Tile::Empty);
    //         }
    //         self.screen.push(row);
    //     }
    // }

    fn draw(&mut self) -> Result<()> {
        self.stdout
            .execute(terminal::Clear(terminal::ClearType::All))?;
        for l in 0..self.height {
            for c in 0..self.width {
                let tile = &self.screen[l as usize][c as usize];
                let char = get_tile_char(&tile);

                queue!(
                    self.stdout,
                    cursor::MoveTo(c + self.offset_x, l + self.offset_y),
                    style::PrintStyledContent(char)
                )?;
                // if c == 0 || c == width - 1 || l == height - 1 {
                //     // stdout.queue()?.queue()?;
                // }
            }
        }
        // queue!(
        //     stdout,
        //     cursor::MoveTo(offset_x + 5, offset_y),
        //     style::PrintStyledContent("#".red()),
        //     cursor::MoveTo(offset_x + 5, offset_y + 1),
        //     style::PrintStyledContent("#".red()),
        //     cursor::MoveTo(offset_x + 6, offset_y + 1),
        //     style::PrintStyledContent("#".red()),
        //     cursor::MoveTo(offset_x + 6, offset_y + 2),
        //     style::PrintStyledContent("#".red()),
        // )?;
        self.stdout
            .queue(Print("\n".repeat(self.offset_y.into())))?;
        self.stdout.flush()?;
        Ok(())
    }

    fn add_block(&mut self, block: &[[u8; 4]; 4], l: u16, c: u16) {
        for i in 0..4 {
            for j in 0..4 {
                if block[i][j] == 1 {
                    // game.screen[l as usize][c as usize + j as usize] = Tile::Wall;
                    self.screen[i + l as usize][j + c as usize] = Tile::Line;
                }
            }
        }
    }
}

//  Empty => " ",
//  Wall => "█",
//  Line => "│",
//  Square => "┼",
//  S1 => "┌",
//  S2 => "┐",
//  L1 => "└",
//  L2 => "┘",

fn get_tile_char(tile: &Tile) -> StyledContent<char> {
    match tile {
        Tile::Empty => ' '.black(),
        Tile::Wall => '█'.dark_grey(),
        Tile::Line => '█'.red(),
        Tile::Square => '┼'.yellow(),
        Tile::S1 => '┌'.green(),
        Tile::S2 => '┐'.magenta(),
        Tile::L1 => '└'.cyan(),
        Tile::L2 => '┘'.white(),
    }
}

fn main() -> Result<()> {
    terminal::enable_raw_mode()?;
    let stdout = stdout();
    let mut game = Game::new(16, 18, stdout);
    //     let mut game = Game {
    //     width: 16,
    //     height: 18,
    //     screen: Vec::new(),
    //     offset_x: 0,
    //     offset_y: 0,
    //     stdout,
    // };

    let blocks = [
        // [],
        [[0, 0, 1, 0], [0, 0, 1, 0], [0, 0, 1, 0], [0, 0, 1, 0]],
        [[0, 1, 1, 0], [0, 1, 1, 0], [0, 0, 0, 0], [0, 0, 0, 0]],
    ];

    for l in 0..game.height {
        game.screen[l as usize][0] = Tile::Wall.into();
        game.screen[l as usize][game.width as usize - 1] = Tile::Wall.into();
    }

    for c in 0..game.width {
        game.screen[game.height as usize - 1][c as usize] = Tile::Wall.into();
    }

    game.add_block(&blocks[1], 3, 5);
    game.add_block(&blocks[0], 7, 0);

    // handle_events()?;

    thread::spawn(handle_events);

    loop {
        game.draw()?;
        thread::sleep(Duration::from_millis(300));
    }
}

// fn draw(game: &Game, stdout: &mut Stdout) -> Result<()> {
//     stdout.execute(terminal::Clear(terminal::ClearType::All))?;
//     for l in 0..game.height {
//         for c in 0..game.width {
//             let tile = &game.screen[l as usize][c as usize];
//             let char = get_tile_char(&tile);

//             queue!(
//                 stdout,
//                 cursor::MoveTo(c + game.offset_x, l + game.offset_y),
//                 style::PrintStyledContent(char.dark_grey())
//             )?;
//             // if c == 0 || c == width - 1 || l == height - 1 {
//             //     // stdout.queue()?.queue()?;
//             // }
//         }
//     }
//     // queue!(
//     //     stdout,
//     //     cursor::MoveTo(offset_x + 5, offset_y),
//     //     style::PrintStyledContent("#".red()),
//     //     cursor::MoveTo(offset_x + 5, offset_y + 1),
//     //     style::PrintStyledContent("#".red()),
//     //     cursor::MoveTo(offset_x + 6, offset_y + 1),
//     //     style::PrintStyledContent("#".red()),
//     //     cursor::MoveTo(offset_x + 6, offset_y + 2),
//     //     style::PrintStyledContent("#".red()),
//     // )?;
//     stdout.queue(Print("\n".repeat(game.offset_y.into())))?;
//     stdout.flush()?;
//     Ok(())
// }
