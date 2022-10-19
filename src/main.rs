#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

use eframe::egui;
use eframe::run_native;
use egui::FontDefinitions;
use objs::Board;
use objs::Tile;

mod objs;

struct Minesweeper {
    board: Board,
    is_game_over: bool,
    is_game_won: bool,
}

impl Default for Minesweeper {
    fn default() -> Self {
        Self {
            board: Board::new(10, 10, 6),
            is_game_over: false,
            is_game_won: false,
        }
    }
}

impl eframe::App for Minesweeper {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
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

                        let button = ui.add_enabled(!self.is_game_over, egui::Button::new(label));

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
    }
}

fn main() {
    let mut minesweeper = Minesweeper {
        board: Board::new(10, 10, 10),
        is_game_over: false,
        is_game_won: false,
    };

    let options = eframe::NativeOptions::default();

    run_native(
        "Minesweeper", 
        options, 
        Box::new(|ctx| Box::new(Minesweeper::default())));
}