use egui::{Vec2, Color32};

use crate::{texturedb, objs::Board};

pub struct CustomBoard {
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

fn validate_input(width: usize, height: usize, mines: usize) -> bool {
  if width < 3 || height < 3 {
      return false;
  }

  if mines > (width * height) - 1 {
      return false;
  }

  true
}

fn num_to_word(num: u8) -> String {
  // do it lowercase
  let output = match num {
      0 => "zero",
      1 => "one",
      2 => "two",
      3 => "three",
      4 => "four",
      5 => "five",
      6 => "six",
      7 => "seven",
      8 => "eight",
      _ => "zero",
  };
  output.to_string()
}

fn get_tint(val: String) -> Color32 {
  // number to color, 0 through 8, make it like a gradient from green to blue
  match val.as_str() {
      "zero" => Color32::from_rgb(0, 255, 0),
      "one" => Color32::from_rgb(0, 186, 45),
      "two" => Color32::from_rgb(0, 127, 90),
      "three" => Color32::from_rgb(0, 68, 135),
      "four" => Color32::from_rgb(0, 9, 180),
      "five" => Color32::from_rgb(45, 0, 135),
      "six" => Color32::from_rgb(90, 0, 90),
      "seven" => Color32::from_rgb(135, 0, 45),
      "eight" => Color32::from_rgb(180, 0, 0),
      // flag and mine
      "flag" => Color32::from_rgb(255, 255, 0),
      "mine" => Color32::from_rgb(255, 0, 0),
      _ => Color32::from_rgb(0, 0, 0),
  }
}

pub struct Minesweeper {
  board: Board,
  is_game_over: bool,
  is_game_won: bool,
  game_started: bool,
  pub custom_board: CustomBoard,
  window_size: Vec2,
  pub texture_db: texturedb::TextureDatabase,
  initial_load: bool,
    pub pure_random: bool,
}

impl Minesweeper {
    pub(crate) fn update_pure_random(&mut self) {
        self.pure_random = !self.pure_random;
    }
}

impl Minesweeper {
  pub fn new_board(&mut self, width: usize, height: usize, mines: usize) {
      self.board = Board::new(width, height, mines, self.pure_random);
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
      let mut texture = texturedb::TextureDatabase::default();
      Self {
          board: Board::new(10, 10, 10, false),
          is_game_over: false,
          is_game_won: false,
          game_started: false,
          custom_board: CustomBoard::default(),
          window_size: Vec2::new(300.0, 300.0),
          texture_db: texture,
          initial_load: false,
          pure_random: false,
      }
  }
}

impl eframe::App for Minesweeper {
  fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
    frame.set_window_size(self.get_window_size());
      if self.game_started == true {
          egui::CentralPanel::default().show(ctx, |ui| {
              if !self.initial_load {
                  self.initial_load = true;
                  self.texture_db.update_all(ui);
              }
              ui.heading("Minesweeper");

              if self.is_game_over {
                  ui.label("Game Over!");
                  ui.label("Score: ".to_string() + &self.board.score.to_string());
                  if ui.button("New Game").clicked() {
                      self.prompt_for_new_game();
                  }
              } else if self.is_game_won {
                  let elapsed_time = self.board.end_time - self.board.start_time;
                  ui.label(format!("You won! Score: {}, Time: {}", self.board.score, elapsed_time.as_secs()));
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
                  if ui.button("New Game").clicked() {
                      self.prompt_for_new_game();
                  }
              }
              // display the board
              for y in 0..self.board.get_height() as usize {
                  ui.horizontal(|ui| {
                      for x in 0..self.board.get_width() as usize {
                          //ui.add(egui::ImageButton::new(self.texture_db.get_texture("base"), Vec2::new(16.0, 16.0)));
                          let tile = self.board.get_tile(x, y);
                          let mut image = self.texture_db.get_texture("base");
                          let mut tint = Color32::WHITE;
                          if tile.is_revealed {
                              if tile.is_mine {
                                  // bomb emoji
                                  image = self.texture_db.get_texture("mine");
                                  tint = get_tint("mine".to_string());
                              } else {
                                  image = self.texture_db.get_texture(num_to_word(tile.adjacent_mines).as_str());
                                  tint = get_tint(num_to_word(tile.adjacent_mines));
                              }
                          } else if tile.is_flagged {
                              // flag emoji
                              image = self.texture_db.get_texture("flag");
                              tint = get_tint("flag".to_string());
                          }

                          let button =
                              ui.add_enabled(!self.is_game_over, 
                                  egui::ImageButton::new(
                                      image, 
                                      Vec2::new(20.0, 20.0))
                                      .tint(tint)
                                  );

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
      }
      else {


          let mut width = self.custom_board.width.to_string();
          let mut height = self.custom_board.height.to_string();
          let mut mines = self.custom_board.mines.to_string();
          
          egui::CentralPanel::default().show(ctx, |ui| {
              if !self.initial_load {
                  self.initial_load = true;
                  self.texture_db.update_all(ui);
              }

              ui.heading("Minesweeper");

              // test button
              //ui.add(egui::ImageButton::new(self.texture_db.get_texture("base"), Vec2::new(16.0, 16.0)));

              ui.label("Welcome to Minesweeper!");
              ui.label("Select a difficulty to begin.");
              ui.checkbox(&mut self.pure_random, "Pure Random");
              ui.horizontal(|ui| {
                  let b1 = ui.add(egui::Button::new("Easy"));
                  let b2 = ui.add(egui::Button::new("Medium"));
                  let b3 = ui.add(egui::Button::new("Hard"));

                  if b1.clicked() {
                      self.update_window_size(Vec2::new(370.0, 400.0));
                      self.new_board(10, 10, 5);
                  }

                  if b2.clicked() {
                      // increase the window size to fit the board
                      self.update_window_size(Vec2::new(555.0, 550.0));
                      self.new_board(15, 15, 30);
                  }

                  if b3.clicked() {
                      // increase the window size to fit the board
                      self.update_window_size(Vec2 { x: 925.0, y: 900.0 });
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
                      let custom_window_size = Vec2::new((self.custom_board.width * 37) as f32, ((self.custom_board.height * 37) + 30) as f32);
                      self.update_window_size(custom_window_size);
                      
                      if validate_input(self.custom_board.width, self.custom_board.height, self.custom_board.mines) {
                          self.new_board(self.custom_board.width, self.custom_board.height, self.custom_board.mines);
                      }
                  }
              });
          });
      }
  }
}