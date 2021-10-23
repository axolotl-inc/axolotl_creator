//! # Axolotl creator!
//! A program in which you can create *ANY* axolotl you want!
//! ### Commands
//! - add, adds an axolotl to your collection.
//! - help, print help menu.
//! - list, list your beautiful axolotls.
//! - edit, change one of your axolotls.
//! - kill, please don't do it.
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
