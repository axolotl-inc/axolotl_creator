//! # Axolotl creator!
//! A place where you can create *ANY* axolotl you want!
//! ### Commands
//! - add axolotl, adds an axolotl to your collection..
//! - help, look at what you are looking at right now.
//! - show my axolotls, list your buitiful axolotls.
//! - edit axolotl, change one of you axolotls.
//! - kill axolotl, please don't do it. If it happens 5 times im taking it away.
//! - read from save, read a save file.
//! - save to file, save your collection of axolotls to a file.
//! - stop, close program.
//!
//! # Panics!
//!
//! When doing user input, if the prgramm failes to read a line, it will panic!  


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
