//! # Axolotl creator!
//! A place where you can create *ANY* axolotl you want!
//! ### Commands
//! - add axolotl, adds an axolotl to your collection..
//! - help, look at what you are looking at right now.
//! - list, list your buitiful axolotls.
//! - edit, change one of you axolotls.
//! - kill, please don't do it. If it happens 5 times im taking it away.
//! - load, read a save file.
//! - save, save your collection of axolotls to a file.
//! - stop, close program. 


pub mod structs;
pub mod tell;
pub mod readinput;

extern crate savefile;
#[macro_use]
extern crate savefile_derive;

fn main() {
    let mut axolotl_vec = structs::AxolotlVec::new();
    tell::intro();
    tell::help();
    readinput::handle_input(&mut axolotl_vec);
}
