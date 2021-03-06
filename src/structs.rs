use savefile::prelude;
use crate::readinput;
use std::path::Path;
use std::fs;

#[derive(Debug)]
#[derive(Savefile)]
pub enum AxolotlType {
    Leucistic,
    GoldenAlbino,
    WildType,
    Piebald,
    Mosaic,
    Copper,
    Lavender,
    BlackMelanoid,
    WhiteAlbino,
    SpeckledLeucistic,
    Chimera,
    HeavilyMarkedMelanoid,
    GreenFluorescentProtein,
    Firefly,
    Enigma,
}

#[derive(Savefile)]
#[derive(Debug)]
pub enum AxolotlFoods {
    BigFish,
    LittleFish,
    Worms,
    PetFood,
    Other(String),
}

#[derive(Savefile)]
#[derive(Debug)]
pub struct Axolotl {
    pub color_type: AxolotlType,
    pub name: String,
    pub favorite_food: AxolotlFoods,
    pub owner: String,
    pub lives: String,
}

#[derive(Savefile)]
pub struct AxolotlVec {
    pub axolotls: Vec<Axolotl>,
}

fn check_save_folder() {
    if !(Path::new("./saves").exists()) {
        let dir = fs::create_dir("./saves");
        let _dir = match dir {
            Ok(directory) => directory,
            Err(_) => {
                panic!("Failed to create save folder. Program Exiting");
            },
        };
    }
}

impl AxolotlVec {
    pub fn new() -> Self {
        check_save_folder();
        Self {axolotls: Vec::new()}
        

    }

    pub fn append_axolotl(&mut self, color_type: AxolotlType, name: String, favorite_food: AxolotlFoods, owner: String, lives: String) {
        self.axolotls.push(Axolotl{color_type, name, favorite_food, owner, lives});
    }

    pub fn list_axolotls(&mut self) {
        println!("my axolotls: {:#?}", self.axolotls);
    }

    pub fn as_string(&self) -> String {
        format!("{:?}", self.axolotls).trim().to_string()
    }

    pub fn read_save(&mut self) {
        let save = readinput::ask_for_save();
        let path = String::from(format!("./saves/{}.dat", save));

        let tmp: AxolotlVec = match prelude::load_file(path.as_str(), 0) {
            Ok(t) => t,
            Err(e) => {println!("Failed reading file. Error: {}", e); AxolotlVec::new()}
        };
        self.axolotls = tmp.axolotls;
    }

    pub fn save_file(&self) {
        let save = readinput::ask_for_save();
        let path = String::from(format!("./saves/{}.dat", save));

        if let Err(e) = prelude::save_file(path.as_str(), 0, self) {
            println!("Failed saving file. Error: {}", e);
        }
    }
}
impl AxolotlType {
    pub fn from(input: &str) -> AxolotlType {
        match input.to_lowercase().as_str() {
            "leucistic" => AxolotlType::Leucistic,
            "golden albino" => AxolotlType::GoldenAlbino,
            "wild type" => AxolotlType::WildType,
            "piebald" => AxolotlType::Piebald,
            "mosaic" => AxolotlType::Mosaic,
            "copper" => AxolotlType::Copper,
            "lavender" => AxolotlType::Lavender,
            "black melanoid" => AxolotlType::BlackMelanoid,
            "white albino" => AxolotlType::WhiteAlbino,
            "speckled leucistic" => AxolotlType::SpeckledLeucistic,
            "heavily marked melanoid" => AxolotlType::HeavilyMarkedMelanoid,
            "chimera" => AxolotlType::Chimera,
            "green fluorescent protein" => AxolotlType::GreenFluorescentProtein,
            "firefly" => AxolotlType::Firefly,
            "enigma" => AxolotlType::Enigma,
            _ => AxolotlType::Leucistic,
        }
    }
}
impl AxolotlFoods {
    pub fn from(input: &str) -> AxolotlFoods {
        match input.to_lowercase().as_str() {
            "big fish" => AxolotlFoods::BigFish,
            "little fish" => AxolotlFoods::LittleFish,
            "worms" => AxolotlFoods::Worms,
            "pet food" => AxolotlFoods::PetFood,
            _ => AxolotlFoods::Other(String::from(input)),
        }
    }
}