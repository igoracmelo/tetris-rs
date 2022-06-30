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
    // S1,
    // S2,
    // L1,
    // L2,
}

#[derive(Clone)]
struct Block {
    x: u16,
    y: u16,
    height: u16,
    fixed: bool,
    tile: Tile,
}

impl Block {
    fn new(x: u16, y: u16, tile: Tile) -> Block {
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
    current_block: Block,
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
            current_block: Block {
                x: 0,
                y: 0,
                height: 0,
                tile: Tile::Empty,
                fixed: false,
            },
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
        self.current_block = block;
        // self.current_block = Some(block);
        self._add_block_to_screen();
        // let block = self.shapmax!();es[tile.clone() as usize];
    }

    fn _add_block_to_screen(&mut self) {
        let block = &self.current_block;
        let shape = self.shapes[block.tile.clone() as usize];
        for i in 0..4 {
            for j in 0..4 {
                if shape[i][j] == 1 {
                    self.screen[i + block.y as usize][j + block.x as usize] = block.tile.clone();
                }
            }
        }
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
        let block = &mut self.current_block;
        let bottom_y = (block.y + block.height) as usize;
        if !block.fixed && bottom_y < self.screen.len() - 1 {
            block.x = block.x.wrapping_add(x as u16);
            block.y = block.y.wrapping_add(y as u16);
        } else {
            block.fixed = true;
        }
    }

    fn _clear_block(&mut self) {
        let block = &self.current_block;
        for l in 0..4 {
            for c in 0..4 {
                let shape = &self.shapes[block.tile.clone() as usize];
                if shape[l as usize][c as usize] == 1 {
                    self.screen[l + block.y as usize][c + block.x as usize] = Tile::Empty;
                }
            }
        }
    }

    fn move_block(&mut self, x: i16, y: i16) {
        self._clear_block();
        self._move_block(x, y);
        self._add_block_to_screen();
    }

    fn update(&mut self) {
        self.move_block(0, 1);
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

fn get_tile_char(tile: &Tile) -> StyledContent<char> {
    match tile {
        Tile::Empty => ' '.black(),
        Tile::Wall => '█'.dark_grey(),
        Tile::Line => '█'.red(),
        Tile::Square => '█'.yellow(),
        // Tile::S1 => '┌'.green(),
        // Tile::S2 => '┐'.magenta(),
        // Tile::L1 => '└'.cyan(),
        // Tile::L2 => '┘'.white(),
    }
}

fn main() -> Result<()> {
    terminal::enable_raw_mode()?;
    let stdout = stdout();
    let mut game = Game::new(16, 18, stdout);

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

    loop {
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
