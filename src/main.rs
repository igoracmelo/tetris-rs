use crossterm::{
    self, cursor,
    event::{poll, read, Event, KeyCode, KeyEvent},
    queue,
    style::{self, Print, StyledContent, Stylize},
    terminal, ExecutableCommand, QueueableCommand, Result,
};
use std::{
    io::{stdout, Stdout, Write},
    process::exit,
    thread,
    time::Duration,
};

fn gracefully_exit() {
    terminal::disable_raw_mode().unwrap();
    exit(0);
}

fn clamp(min: u16, value: u16, max: u16) -> u16 {
    let mut v = value;
    if v < min {
        v = min;
    } else if v > max {
        v = max;
    }
    v
}

#[derive(Clone)]
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

// enum BlockShape {
//     Share([[0, 0, 1, 0], [0, 0, 1, 0], [0, 0, 1, 0], [0, 0, 1, 0]]),
//         2 => [[0, 1, 1, 0], [0, 1, 1, 0], [0, 0, 0, 0], [0, 0, 0, 0]],
// }

#[derive(Clone)]
struct Block {
    x: u16,
    y: u16,
    height: u16,
    // symbol: StyledContent<char>,
    // shape: u16,
    fixed: bool,
    tile: Tile,
    // shape: &[[u8; 4]; 4],
}

impl Block {
    fn new(x: u16, y: u16, tile: Tile) -> Block {
        // for i in 0..4 {
        //     let has_piece = false;
        //     for j in 0..4 {
        //         if
        //     }
        // }
        Block {
            x,
            y,
            height: 0,
            tile,
            fixed: false,
        }
    }
}

struct Game {
    width: u16,
    height: u16,
    offset_x: u16,
    offset_y: u16,
    // tiles: Vec<Tile>,
    screen: Vec<Vec<Tile>>,
    stdout: Stdout,
    current_block: Option<Block>,
    shapes: Vec<[[u8; 4]; 4]>,
    // blocks: Vec<Block>,
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
            current_block: None,
            shapes: Vec::new(),
            // blocks: Vec::new(),
        }
    }

    fn draw(&mut self) -> Result<()> {
        self.stdout
            .execute(terminal::Clear(terminal::ClearType::All))?;

        // printing the screen
        for l in 0..self.height {
            for c in 0..self.width {
                let tile = &self.screen[l as usize][c as usize];
                let char = get_tile_char(&tile);

                queue!(
                    self.stdout,
                    cursor::MoveTo(c + self.offset_x, l + self.offset_y),
                    style::PrintStyledContent(char)
                )?;
            }
        }
        self.stdout
            .queue(Print("\n".repeat(self.offset_y.into())))?;
        self.stdout.flush()?;
        Ok(())
    }

    // fn add_block(&mut self, tile: Tile, l: u16, c: u16) {
    //     let block = self.shapes[tile.clone() as usize];
    //     for i in 0..4 {
    //         for j in 0..4 {
    //             if block[i][j] == 1 {
    //                 // game.screen[l as usize][c as usize + j as usize] = Tile::Wall;
    //                 self.screen[i + l as usize][j + c as usize] = tile.clone();
    //             }
    //         }
    //     }

    //     self.current_block = Some(Block {
    //         x: c,
    //         y: l,
    //         tile,
    //         // symbol: get_tile_char(&tile),
    //     });
    // }

    fn add_block(&mut self, mut block: Block) {
        // let block_copy = block.clone();
        let mut height = 0;
        let shape = self.shapes[block.tile.clone() as usize];
        for i in 0..4 {
            for j in 0..4 {
                if shape[i][j] == 1 {
                    height += 1;
                    break;
                }
            }
        }

        block.height = height;
        self.current_block = Some(block);
        self._add_block_to_screen();
        // let block = self.shapmax!();es[tile.clone() as usize];
    }

    fn _add_block_to_screen(&mut self) {
        // self.current_block = Some(block);
        if let Some(block) = &self.current_block {
            // let block_copy = block.clone();

            let shape = self.shapes[block.tile.clone() as usize];
            for i in 0..4 {
                for j in 0..4 {
                    if shape[i][j] == 1 {
                        // game.screen[l as usize][c as usize + j as usize] = Tile::Wall;
                        self.screen[i + block.y as usize][j + block.x as usize] =
                            block.tile.clone();
                    }
                }
            }
        }
        // let block = self.shapes[tile.clone() as usize];
    }

    // fn start_handling_events(self) -> Result<()> {
    //     loop {
    //         if poll(Duration::from_millis(500))? {
    //             match read()? {
    //                 Event::Key(event) => on_keypress(&event),
    //                 _ => {}
    //             }
    //         }
    //     }
    //     // Ok(())
    // }

    fn _move_block(&mut self, x: i16, y: i16) {
        // self.clear_block();
        // self.add_block(block.clone());
        if let Some(block) = &mut self.current_block {
            let bottom_y = (block.y + block.height) as usize;
            if !block.fixed && bottom_y < self.screen.len() - 1 {
                // println!("{}", block.y);
                // println!("{}", self.screen.len());
                // block.x += x;
                // block.y += y;
                block.x = block.x.wrapping_add(x as u16);
                block.y = block.y.wrapping_add(y as u16);
            } else {
                block.fixed = true;
                // self.current_block = None;
            }
        }
    }

    fn _clear_block(&mut self) {
        if let Some(block) = &mut self.current_block {
            for l in 0..4 {
                for c in 0..4 {
                    let shape = &self.shapes[block.tile.clone() as usize];
                    if shape[l as usize][c as usize] == 1 {
                        self.screen[l + block.y as usize][c + block.x as usize] = Tile::Empty;
                    }
                }
            }
            // block.y += 1;
        }

        // if let Some(_) = self.current_block {
        //     todo!()
        // } else {
        //     todo!()
        // }
    }

    fn move_block(&mut self, x: i16, y: i16) {
        self._clear_block();
        self._move_block(x, y);
        self._add_block_to_screen();
    }

    fn update(&mut self) {
        self.move_block(0, 1);
        // self.current_block.y += 1;
    }

    fn on_keypress(&mut self, event: &KeyEvent) {
        if event.code == KeyCode::Char('q') {
            gracefully_exit();
        }

        if event.code == KeyCode::Char('a') {
            self.move_block(-1, 0);
        }

        if event.code == KeyCode::Char('d') {
            self.move_block(1, 0);
        }

        if event.code == KeyCode::Char('s') {
            self.move_block(0, 1);
        }

        // println!("{:?}", event.code);
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
        Tile::Square => '█'.yellow(),
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

    // let blocks = [
    //     // [],
    //     ,
    //     ,
    // ];

    game.shapes
        .push([[0, 0, 0, 0], [0, 0, 0, 0], [0, 0, 0, 0], [0, 0, 0, 0]]);

    game.shapes
        .push([[0, 0, 0, 0], [0, 0, 0, 0], [0, 0, 0, 0], [0, 0, 0, 0]]);

    game.shapes
        .push([[0, 0, 1, 0], [0, 0, 1, 0], [0, 0, 1, 0], [0, 0, 1, 0]]);

    game.shapes
        .push([[0, 1, 1, 0], [0, 1, 1, 0], [0, 0, 0, 0], [0, 0, 0, 0]]);

    for l in 0..game.height {
        game.screen[l as usize][0] = Tile::Wall.into();
        game.screen[l as usize][game.width as usize - 1] = Tile::Wall.into();
    }

    for c in 0..game.width {
        game.screen[game.height as usize - 1][c as usize] = Tile::Wall.into();
    }

    game.add_block(Block::new(5, 3, Tile::Line));
    game.add_block(Block::new(0, 7, Tile::Square));
    //     Block {
    //     x: 0,
    //     y: 7,
    //     height: 2,
    //     tile: Tile::Square,
    // });

    // handle_events()?;

    // thread::spawn(move || game.start_handling_events());

    // let mut l = 1;
    loop {
        // match read()? {
        //     Event::Key(event) => on_keypress(&event),
        //     _ => {}
        // }

        // game.add_block(&blocks[1], Tile::Line, l, 5);
        // l += 1;

        if poll(Duration::from_millis(30))? {
            if let Event::Key(event) = read()? {
                game.on_keypress(&event);
            }
        } else {
            game.update();
            game.draw()?;
            thread::sleep(Duration::from_millis(300));
        }
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
