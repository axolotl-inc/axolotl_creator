use std::fs;
//use std::io::Error;
//use std::io::ErrorKind;
use crate::structs;
#[allow(unused)]

pub fn read_save(path: String, mut structure: structs::AxolotlVec) {
    let mut contents = String::new(); 
    match fs::read_to_string(path) {
        Ok(t) => contents = t,
        Err(_) => contents = "".to_string()
    };
    for line in contents.lines() {
        if line.contains("pogpogpogpgo") {
            println!("pog. line was {}", line);
        }
    }
}