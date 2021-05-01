pub mod structs;
pub mod tell;
pub mod readinput;
pub mod read_file;

fn main() {
    let mut axolotl_vec = structs::AxolotlVec::new();
    tell::intro();
    tell::help();
    readinput::handle_input(&mut axolotl_vec);
    read_file::read_save(String::from("test.txt"), axolotl_vec);
}