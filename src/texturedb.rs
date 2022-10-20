use egui::{Image, TextBuffer, TextureHandle, Ui};

pub struct TextureDatabase {
    pub texture: Option<egui::TextureHandle>,
    pub base: Option<egui::TextureHandle>,
    pub flag: Option<egui::TextureHandle>,
    pub mine: Option<egui::TextureHandle>,
    pub zero: Option<egui::TextureHandle>,
    pub one: Option<egui::TextureHandle>,
    pub two: Option<egui::TextureHandle>,
    pub three: Option<egui::TextureHandle>,
    pub four: Option<egui::TextureHandle>,
    pub five: Option<egui::TextureHandle>,
    pub six: Option<egui::TextureHandle>,
    pub seven: Option<egui::TextureHandle>,
    pub eight: Option<egui::TextureHandle>,
}

pub struct ImageDatabase {
    pub base: egui::ColorImage,
    pub flag: egui::ColorImage,
    pub mine: egui::ColorImage,
    pub zero: egui::ColorImage,
    pub one: egui::ColorImage,
    pub two: egui::ColorImage,
    pub three: egui::ColorImage,
    pub four: egui::ColorImage,
    pub five: egui::ColorImage,
    pub six: egui::ColorImage,
    pub seven: egui::ColorImage,
    pub eight: egui::ColorImage,
}

impl TextureDatabase {
    pub fn update_all(&mut self, ui: &mut egui::Ui) {
        self.update_with_ui(ui, "base");
        self.update_with_ui(ui, "flag");
        self.update_with_ui(ui, "mine");
        self.update_with_ui(ui, "zero");
        self.update_with_ui(ui, "one");
        self.update_with_ui(ui, "two");
        self.update_with_ui(ui, "three");
        self.update_with_ui(ui, "four");
        self.update_with_ui(ui, "five");
        self.update_with_ui(ui, "six");
        self.update_with_ui(ui, "seven");
        self.update_with_ui(ui, "eight");
    }
    pub(crate) fn update_with_ui(&mut self, ui: &mut egui::Ui, image: &str) -> &egui::TextureHandle {
        let image_db = ImageDatabase::default();
        let texture = match image {
            "base" => {
                let image_file = image_db.base;
                self.base.get_or_insert_with(|| {
                    ui.ctx().load_texture(
                        image,
                        image_file,
                        egui::TextureFilter::Linear
                    )
                })
            },
            "flag" => {
                let image_file = image_db.flag;
                self.flag.get_or_insert_with(|| {
                    ui.ctx().load_texture(
                        image.as_str(),
                        image_file,
                        egui::TextureFilter::Linear
                    )
                })
            },
            "mine" => {
                let image_file = image_db.mine;
                self.mine.get_or_insert_with(|| {
                    ui.ctx().load_texture(
                        image.as_str(),
                        image_file,
                        egui::TextureFilter::Linear
                    )
                })
            },
            "zero" => {
                let image_file = image_db.zero;
                self.zero.get_or_insert_with(|| {
                    ui.ctx().load_texture(
                        image.as_str(),
                        image_file,
                        egui::TextureFilter::Linear
                    )
                })
            },
            "one" => {
                let image_file = image_db.one;
                self.one.get_or_insert_with(|| {
                    ui.ctx().load_texture(
                        image.as_str(),
                        image_file,
                        egui::TextureFilter::Linear
                    )
                })
            },
            "two" => {
                let image_file = image_db.two;
                self.two.get_or_insert_with(|| {
                    ui.ctx().load_texture(
                        image.as_str(),
                        image_file,
                        egui::TextureFilter::Linear
                    )
                })
            },
            "three" => {
                let image_file = image_db.three;
                self.three.get_or_insert_with(|| {
                    ui.ctx().load_texture(
                        image.as_str(),
                        image_file,
                        egui::TextureFilter::Linear
                    )
                })
            },
            "four" => {
                let image_file = image_db.four;
                self.four.get_or_insert_with(|| {
                    ui.ctx().load_texture(
                        image.as_str(),
                        image_file,
                        egui::TextureFilter::Linear
                    )
                })
            },
            "five" => {
                let image_file = image_db.five;
                self.five.get_or_insert_with(|| {
                    ui.ctx().load_texture(
                        image.as_str(),
                        image_file,
                        egui::TextureFilter::Linear
                    )
                })
            },
            "six" => {
                let image_file = image_db.six;
                self.six.get_or_insert_with(|| {
                    ui.ctx().load_texture(
                        image.as_str(),
                        image_file,
                        egui::TextureFilter::Linear
                    )
                })
            },
            "seven" => {
                let image_file = image_db.seven;
                self.seven.get_or_insert_with(|| {
                    ui.ctx().load_texture(
                        image.as_str(),
                        image_file,
                        egui::TextureFilter::Linear
                    )
                })
            },
            "eight" => {
                let image_file = image_db.eight;
                self.eight.get_or_insert_with(|| {
                    ui.ctx().load_texture(
                        image.as_str(),
                        image_file,
                        egui::TextureFilter::Linear
                    )
                })
            },
            _ => {
                let image_file = image_db.base;
                self.base.get_or_insert_with(|| {
                    ui.ctx().load_texture(
                        image.as_str(),
                        image_file,
                        egui::TextureFilter::Linear
                    )
                })
            },
        };
        texture
    }

    pub fn get_texture(&mut self, name: &str) -> &egui::TextureHandle {
        match name {
            "base" => self.base.as_ref().unwrap(),
            "flag" => self.flag.as_ref().unwrap(),
            "mine" => self.mine.as_ref().unwrap(),
            "zero" => self.zero.as_ref().unwrap(),
            "one" => self.one.as_ref().unwrap(),
            "two" => self.two.as_ref().unwrap(),
            "three" => self.three.as_ref().unwrap(),
            "four" => self.four.as_ref().unwrap(),
            "five" => self.five.as_ref().unwrap(),
            "six" => self.six.as_ref().unwrap(),
            "seven" => self.seven.as_ref().unwrap(),
            "eight" => self.eight.as_ref().unwrap(),
            _ => self.base.as_ref().unwrap(),
        }
    }
}

impl Default for TextureDatabase {
    fn default() -> Self {
        Self {
            texture: None,
            base: None,
            flag: None,
            mine: None,
            zero: None,
            one: None,
            two: None,
            three: None,
            four: None,
            five: None,
            six: None,
            seven: None,
            eight: None
        }
    }
}

impl ImageDatabase {
    pub fn load_image_from_path(path: &std::path::Path) -> Result<egui::ColorImage, image::ImageError> {
        let image = image::io::Reader::open(path)?.decode()?;
        let size = [image.width() as _, image.height() as _];
        let image_buffer = image.to_rgba8();
        let pixels = image_buffer.as_flat_samples();
        Ok(egui::ColorImage::from_rgba_unmultiplied(
            size,
            pixels.as_slice(),
        ))
    }
}

impl Default for ImageDatabase {

    fn default() -> Self {
        let base = ImageDatabase::load_image_from_path(std::path::Path::new("assets/base.png")).unwrap();
        let flag = ImageDatabase::load_image_from_path(std::path::Path::new("assets/f.png")).unwrap();
        let mine = ImageDatabase::load_image_from_path(std::path::Path::new("assets/b.png")).unwrap();
        let zero = ImageDatabase::load_image_from_path(std::path::Path::new("assets/0.png")).unwrap();
        let one = ImageDatabase::load_image_from_path(std::path::Path::new("assets/1.png")).unwrap();
        let two = ImageDatabase::load_image_from_path(std::path::Path::new("assets/2.png")).unwrap();
        let three = ImageDatabase::load_image_from_path(std::path::Path::new("assets/3.png")).unwrap();
        let four = ImageDatabase::load_image_from_path(std::path::Path::new("assets/4.png")).unwrap();
        let five = ImageDatabase::load_image_from_path(std::path::Path::new("assets/5.png")).unwrap();
        let six = ImageDatabase::load_image_from_path(std::path::Path::new("assets/6.png")).unwrap();
        let seven = ImageDatabase::load_image_from_path(std::path::Path::new("assets/7.png")).unwrap();
        let eight = ImageDatabase::load_image_from_path(std::path::Path::new("assets/8.png")).unwrap();
        Self {
            base,
            flag,
            mine,
            zero,
            one,
            two,
            three,
            four,
            five,
            six,
            seven,
            eight,
        }
    }
}