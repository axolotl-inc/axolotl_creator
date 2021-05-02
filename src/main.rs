pub mod structs;
pub mod tell;
pub mod readinput;

fn main() {
    let mut axolotl_vec = structs::AxolotlVec::new();
    tell::intro();
    tell::help();
    readinput::handle_input(&mut axolotl_vec);
}