#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

use eframe::egui;
use eframe::run_native;
use egui::Vec2;
use objs::Board;

mod objs;

struct CustomBoard {
    pub width: usize,
    pub height: usize,
    pub mines: usize,
}

impl Default for CustomBoard {
    fn default() -> Self {
        Self {
            width: 10,
            height: 10,
            mines: 10,
        }
    }
}

struct Minesweeper {
    board: Board,
    is_game_over: bool,
    is_game_won: bool,
    game_started: bool,
    pub custom_board: CustomBoard,
    window_size: Vec2,
}

impl Minesweeper {
    pub fn new_board(&mut self, width: usize, height: usize, mines: usize) {
        self.board = Board::new(width, height, mines);
        self.is_game_over = false;
        self.is_game_won = false;
        self.game_started = true;
    }

    pub fn prompt_for_new_game(&mut self) {
        self.game_started = false;
        self.window_size = Vec2::new(300.0, 300.0);
    }

    pub fn update_custom_board(&mut self, width: usize, height: usize, mines: usize) {
        self.custom_board.width = width;
        self.custom_board.height = height;
        self.custom_board.mines = mines;
    }

    pub fn update_window_size(&mut self, size: Vec2) {
        self.window_size = size;
    }

    pub fn get_window_size(&self) -> Vec2 {
        self.window_size
    }
}

impl Default for Minesweeper {
    fn default() -> Self {
        Self {
            board: Board::new(10, 10, 10),
            is_game_over: false,
            is_game_won: false,
            game_started: false,
            custom_board: CustomBoard::default(),
            window_size: Vec2::new(300.0, 300.0),
        }
    }
}

impl eframe::App for Minesweeper {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        frame.set_window_size(self.get_window_size());
        if self.game_started == true {
            egui::CentralPanel::default().show(ctx, |ui| {
                ui.heading("Minesweeper");

                if self.is_game_over {
                    ui.label("Game Over!");
                    if ui.button("New Game").clicked() {
                        self.prompt_for_new_game();
                    }
                } else if self.is_game_won {
                    ui.label("You won!");
                    if ui.button("New Game").clicked() {
                        self.prompt_for_new_game();
                    }
                } else {
                    let info = format!(
                        "Total mines: {} Flags used: {} Mines remaining: {}",
                        self.board.get_mines(),
                        self.board.get_flags(),
                        self.board.get_mines() - self.board.get_flags()
                    );

                    ui.label(info);
                }
                // display the board
                for y in 0..self.board.get_height() as usize {
                    ui.horizontal(|ui| {
                        for x in 0..self.board.get_width() as usize {
                            let tile = self.board.get_tile(x, y);
                            let label: String;
                            if tile.is_revealed {
                                if tile.is_mine {
                                    // bomb emoji
                                    label = "💣".to_string();
                                } else {
                                    label = format!(" {} ", tile.adjacent_mines);
                                }
                            } else if tile.is_flagged {
                                // flag emoji
                                label = "🚩".to_string();
                            } else {
                                // blank emoji
                                label = "⬜".to_string();
                            }

                            let button =
                                ui.add_enabled(!self.is_game_over, egui::Button::new(label));

                            if button.clicked() {
                                self.is_game_over = self.board.select_tile(x, y);
                                self.is_game_won = self.board.is_win();

                                if self.is_game_over {
                                    self.board.on_lost();
                                }
                            }

                            if button.secondary_clicked() {
                                self.board.flag_tile(x, y);
                            }
                        }
                    });
                }
            });
        } else {
            let mut width = self.custom_board.width.to_string();
            let mut height = self.custom_board.height.to_string();
            let mut mines = self.custom_board.mines.to_string();
            
            egui::CentralPanel::default().show(ctx, |ui| {
                ui.heading("Minesweeper");
                ui.label("Welcome to Minesweeper!");
                ui.label("Select a difficulty to begin.");
                ui.horizontal(|ui| {
                    let b1 = ui.add(egui::Button::new("Easy"));
                    let b2 = ui.add(egui::Button::new("Medium"));
                    let b3 = ui.add(egui::Button::new("Hard"));

                    if b1.clicked() {
                        self.new_board(10, 10, 5);
                    }

                    if b2.clicked() {
                        // increase the window size to fit the board
                        self.update_window_size(Vec2::new(450.0, 450.0));
                        self.new_board(15, 15, 30);
                    }

                    if b3.clicked() {
                        // increase the window size to fit the board
                        self.update_window_size(Vec2 { x: 750.0, y: 750.0 });
                        self.new_board(25, 25, 50);
                    }
                });
                ui.label("Or play custom");
                ui.label("Enter the width, height, and number of mines.");
                ui.horizontal(|ui| {
                    let w = ui.add_sized([20.0, 20.0], egui::TextEdit::singleline(&mut width).hint_text("Width"));
                    let h = ui.add_sized([20.0, 20.0], egui::TextEdit::singleline(&mut height).hint_text("Height"));
                    let m = ui.add_sized([20.0, 20.0], egui::TextEdit::singleline(&mut mines).hint_text("Mines"));

                    if w.changed() || h.changed() || m.changed() {
                        let w = width.parse::<usize>().unwrap_or(10);
                        let h = height.parse::<usize>().unwrap_or(10);
                        let m = mines.parse::<usize>().unwrap_or(10);

                        self.update_custom_board(w, h, m);
                    }

                    if ui.button("Start Custom Game").clicked() {
                        let custom_window_size = Vec2::new((self.custom_board.width * 30) as f32, (self.custom_board.height * 30) as f32);
                        self.update_window_size(custom_window_size);
                        // TODO: validate input
                        self.new_board(
                            self.custom_board.width,
                            self.custom_board.height,
                            self.custom_board.mines,
                        );
                    }
                });
            });
        }
    }
}

fn main() {
    let mut options = eframe::NativeOptions::default();

    options.initial_window_size = Some(Vec2::new(300.0, 300.0));
    options.resizable = false;

    run_native(
        "Minesweeper",
        options,
        Box::new(|_ctx| Box::new(Minesweeper::default())),
    );
}
