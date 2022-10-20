#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

use eframe::egui;
use eframe::run_native;
use objs::Board;

mod objs;

struct Minesweeper {
    board: Board,
    is_game_over: bool,
    is_game_won: bool,
    game_started: bool,
}

impl Minesweeper {
    pub fn new_board(&mut self, width: usize, height: usize, mines: usize) {
        self.board = Board::new(width, height, mines);
        self.is_game_over = false;
        self.is_game_won = false;
        self.game_started = true;
    }
}

impl Default for Minesweeper {
    fn default() -> Self {
        Self {
            board: Board::new(10, 10, 10),
            is_game_over: false,
            is_game_won: false,
            game_started: false,
        }
    }
}

impl eframe::App for Minesweeper {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        if (self.game_started == true) {
            egui::CentralPanel::default().show(ctx, |ui| {
                ui.heading("Minesweeper");

                if self.is_game_over {
                    ui.label("Game Over!");
                    self.board.on_lost();
                } else if self.is_game_won {
                    ui.label("You won!");
                }

                // display the board
                for y in 0..self.board.get_height() as usize {
                    ui.horizontal(|ui| {
                        for x in 0..self.board.get_width() as usize {
                            let tile = self.board.get_tile(x, y);
                            let mut label = String::new();
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
                            }

                            if button.secondary_clicked() {
                                self.board.flag_tile(x, y);
                            }
                        }
                    });
                }
            });
        } else {
            egui::CentralPanel::default().show(ctx, |ui| {
                ui.heading("Minesweeper");
                ui.label("Welcome to Minesweeper!");
                ui.label("Select a difficulty to begin.");
                ui.horizontal(|ui| {
                    let b1 = ui.add_enabled(!self.is_game_over, egui::Button::new("Easy"));
                    let b2 = ui.add_enabled(!self.is_game_over, egui::Button::new("Medium"));
                    let b3 = ui.add_enabled(!self.is_game_over, egui::Button::new("Hard"));
                    
                    if b1.clicked() {
                        self.new_board(10, 10, 10);
                    }

                    if b2.clicked() {
                        self.new_board(15, 15, 30);
                    }

                    if b3.clicked() {
                        self.new_board(25, 25, 50);
                    }
                });
            });
        }
    }
}

fn main() {
    let options = eframe::NativeOptions::default();

    run_native(
        "Minesweeper",
        options,
        Box::new(|ctx| Box::new(Minesweeper::default())),
    );
}
