use rand::Rng;
use std::time::Instant;

pub struct Tile {
    pub is_mine: bool,
    pub is_revealed: bool,
    pub is_flagged: bool,
    pub adjacent_mines: u8,
}

pub struct Board {
    tiles: Vec<Vec<Tile>>,
    width: u8,
    height: u8,
    mines: u8,
    flags: u8,
    pub score: u8,
    pub pure_random: bool,

    pub start_time: Instant,
    pub end_time: Instant,
}

impl Board {
    pub fn new(width: usize, height: usize, mines: usize, pure_random: bool) -> Board {
        let mut tiles = Vec::new();
        for _ in 0..height {
            let mut row = Vec::new();
            for _ in 0..width {
                row.push(Tile {
                    is_mine: false,
                    is_revealed: false,
                    is_flagged: false,
                    adjacent_mines: 0,
                });
            }
            tiles.push(row);
        }
        let mut board = Board {
            tiles,
            width: width as u8,
            height: height as u8,
            mines: mines as u8,
            flags: 0,
            score: 0,
            start_time: Instant::now(),
            end_time: Instant::now(),
            pure_random,
        };

        if !pure_random { board.advanced_place_mines(); } else { board.place_mines(); }
        board.calculate_adjacent_mines();

        board
    }

    fn advanced_place_mines(&mut self) {
        let mut rng = rand::thread_rng();
        let mut mines_placed = 0;
        while mines_placed < self.mines {
            // pick a random tile
            let x = rng.gen_range(0..self.width);
            let y = rng.gen_range(0..self.height);

            // if the tile is already a mine, skip it
            if self.tiles[y as usize][x as usize].is_mine {
                continue;
            }

            let mut mines_in_surrounding_tiles: u8 = 0;
            let mut tiles_in_surrounding_tiles: u8 = 0;
            for y_offset in -1..=1 {
                for x_offset in -1..=1 {
                    // skip the current tile
                    if x_offset == 0 && y_offset == 0 {
                        continue;
                    }

                    // skip tiles that are out of bounds
                    if x as i8 + x_offset < 0 || x as i8 + x_offset >= self.width as i8 || y as i8 + y_offset < 0 || y as i8 + y_offset >= self.height as i8 {
                        continue;
                    }

                    // increment the number of mines and tiles in the surrounding 8 tiles
                    tiles_in_surrounding_tiles += 1;
                    if self.tiles[(y as i8 + y_offset) as usize][(x as i8 + x_offset) as usize].is_mine {
                        mines_in_surrounding_tiles += 1;
                    }
                }
            }
            let likelihood = (mines_in_surrounding_tiles as f32 / tiles_in_surrounding_tiles as f32) * 100.0;

            if rng.gen_range(0..100) > likelihood as u8 {
                println!("skipping tile (likelihood is {})", likelihood);
                continue;
            }

            // place a mine in this tile
            println!("Placing mine at {}, {} (likelihood: {})", x, y, likelihood);
            self.tiles[y as usize][x as usize].is_mine = true;
            mines_placed += 1;
        }
    }

    fn place_mines(&mut self) {
        let mut mines_placed = 0;
        while mines_placed < self.mines {
            let x = rand::random::<usize>() % self.width as usize;
            let y = rand::random::<usize>() % self.height as usize;

            if !self.tiles[y][x].is_mine {
                self.tiles[y][x].is_mine = true;
                mines_placed += 1;
            }
        }
    }

    fn calculate_adjacent_mines(&mut self) {
        for y in 0..self.height {
            for x in 0..self.width {
                let mut adjacent_mines = 0;
                for y_offset in -1..=1 {
                    for x_offset in -1..=1 {
                        if x_offset == 0 && y_offset == 0 {
                            continue;
                        }

                        let x = x as i8 + x_offset;
                        let y = y as i8 + y_offset;

                        if x < 0 || x >= self.width as i8 || y < 0 || y >= self.height as i8 {
                            continue;
                        }

                        if self.tiles[y as usize][x as usize].is_mine {
                            adjacent_mines += 1;
                        }
                    }
                }
                self.tiles[y as usize][x as usize].adjacent_mines = adjacent_mines;
            }
        }
    }


    pub fn select_tile(&mut self, x: usize, y: usize) -> bool {
        let tile = &mut self.tiles[y][x];
        if tile.is_flagged {
            return false;
        }

        if tile.is_revealed {
            return false;
        }

        tile.is_revealed = true;
        if tile.is_mine {
            return true;
        }

        if tile.adjacent_mines == 0 {
            for y_offset in -1..=1 {
                for x_offset in -1..=1 {
                    if x_offset == 0 && y_offset == 0 {
                        continue;
                    }

                    let x = x as i8 + x_offset;
                    let y = y as i8 + y_offset;

                    if x < 0 || x >= self.width as i8 || y < 0 || y >= self.height as i8 {
                        continue;
                    }

                    if self.select_tile(x as usize, y as usize) {
                        //println!("Mine found at {}, {}", x, y);
                    } else {
                        //println!("No mine found at {}, {}", x, y);
                        score += 1;
                    }
                }
            }
        }

        // increase the score by 1
        self.score += 1;
        false
    }

    pub fn flag_tile(&mut self, x: usize, y: usize) {
        if self.flags == self.mines {
            return;
        }

        let tile = &mut self.tiles[y][x];
        if tile.is_revealed {
            println!("Tile is already revealed");
            return;
        }

        tile.is_flagged = !tile.is_flagged;
        if tile.is_flagged {
            self.flags += 1;
        } else {
            self.flags -= 1;
        }
    }

    pub fn is_win(&mut self) -> bool {
        self.on_game_end();
        for y in 0..self.height {
            for x in 0..self.width {
                let tile = &self.tiles[y as usize][x as usize];
                if !tile.is_mine && !tile.is_revealed {
                    return false;
                }
            }
        }
        true
    }

    pub fn on_game_end(&mut self) {
        if (self.end_time - self.start_time).as_secs() == 0 {
            self.end_time = Instant::now();
        }
    }

    pub fn on_lost(&mut self) {
        self.on_game_end();
        for y in 0..self.height {
            for x in 0..self.width {
                let tile = &mut self.tiles[y as usize][x as usize];
                if tile.is_mine {
                    tile.is_revealed = true;
                    if tile.is_flagged {
                        self.score += 10;
                    }
                }
            }
        }

        self.print_board();
    }

    pub fn get_mines(&self) -> u8 {
        self.mines
    }

    pub fn get_flags(&self) -> u8 {
        self.flags
    }

    pub fn get_height(&self) -> u8 {
        self.height
    }

    pub fn get_width(&self) -> u8 {
        self.width
    }

    pub fn get_tile(&mut self, x: usize, y: usize) -> &Tile {
        &self.tiles[y][x]
    }
}
