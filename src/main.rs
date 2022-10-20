#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

use eframe::egui;
use eframe::run_native;
use egui::Color32;
use egui::Vec2;
use objs::Board;
use minesweeper::Minesweeper;
use minesweeper::CustomBoard;

mod objs;
mod texturedb;
mod minesweeper;

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
