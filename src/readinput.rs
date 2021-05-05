use std::io::stdin;
use structs::Axolotl;
use crate::{ tell, structs };


pub fn handle_input(vec: &mut structs::AxolotlVec) {
    let mut msg = String::new();
    loop {
        stdin().read_line(&mut msg).expect("Failed to read line. Program exiting.");
        if msg.trim() == String::from("help") {
            tell::help();
        } else if msg.trim() == String::from("add axolotl") {
            add_axolotl(vec);
        } else if msg.trim() == String::from("show my axolotls") {
            vec.list_axolotls();
        } else if msg.trim() == String::from("edit axolotl") {
            edit_axolotl(vec);
        } else if msg.trim() == String::from("kill axolotl") {
            kill_axolotl(vec);
        } else if msg.trim() == String::from("save to file") {
            vec.save_file();
        } else if msg.trim() == String::from("read from save") {
            vec.read_save();
        } else if msg.trim() == String::from("axolotls be win") {
            println!("\nYour right about that!");
            println!("#_____#  ");
            println!("( o~o )  ");
            println!("o)   (o  ");
            println!("(_____)//");
            println!("");
        } else if msg.trim() == String::from("stop") {
            break;
        } else {
            println!("Error reading command or message, please try again");
        }
        msg = String::new()
    }
}

fn kill_axolotl(vec: &mut structs::AxolotlVec) -> usize {
    println!("Please enter the index of your axolotl you want to kill.");
    let mut index_tmp = String::new();
    stdin().read_line(&mut index_tmp).expect("Failed to read line.");
    let index = match index_tmp.trim().parse::<usize>() {
        Ok(t) => t,
        Err(e) => {
            println!("Failed to readline. Error: {}", e);
            kill_axolotl(vec)
        }
    };
    if index >= vec.axolotls.len() {
        println!("To big of an index.");
        kill_axolotl(vec);
    }
    else {
        vec.axolotls.remove(index);
        println!("Killed axolotl. YOUR STUPID");
    }
    index
}

fn edit_axolotl(vec: &mut structs::AxolotlVec) {
    println!("Please enter the index of your axolotl you want to change.");
    let mut index_tmp = String::new();
    stdin().read_line(&mut index_tmp).expect("Failed to read line.");
    let index = index_tmp.trim().parse::<usize>().unwrap();
    println!("get ready to change {}...\n", vec.axolotls[index].name);

    println!("What would you like the type/color of axolotl to be?");
    println!("Choices: Leucistic, Golden Albino, Wild Type, Piebald, Mosaic, Copper, Lavender, Black Melanoid, White Albino, Speckled Leucistic, Chimera, Heavily Marked Melanoid, Green Fluorescent Protein, Firefly, Enigma,");
    println!("Please type in your desired type: ");
    let mut color = String::new();
    stdin().read_line(&mut color).expect("Failed to read line. Program exiting.");
    let color_type = structs::AxolotlType::from(&color.trim());

    println!("What would you like your axolotls name to be?");
    let mut name = String::new();
    stdin().read_line(&mut name).expect("Failed to read line. Program exiting.");
    name = name.trim().to_string();

    println!("What is {}'s favorite food?", name);
    println!("The choices are: Small Fish, Little Fish, Worms, Pet Food, Other(String)");
    println!("Please type in {}'s disered food: ", name);
    let mut food = String::new();
    stdin().read_line(&mut food).expect("Failed to read line. Profram exiting.");
    let favorite_food = structs::AxolotlFoods::from(&food.trim());

    println!("Who is {}'s owener?", name);
    let mut owner = String::new();
    stdin().read_line(&mut owner).expect("Failed to read line. Program exiting.");
    owner = owner.trim().to_string();

    println!("Where does {} live?", name);
    let mut lives = String::new();
    stdin().read_line(&mut lives).expect("Failed to read line. Program exiting.");
    lives = lives.trim().to_string();

    println!("{} is alive!\n", name);

    vec.axolotls[index] = Axolotl{color_type, name, favorite_food, owner, lives};

}

fn add_axolotl(vec: &mut structs::AxolotlVec) {
    println!("What would you like the type/color of axolotl to be?");
    println!("Choices: Leucistic, Golden Albino, Wild Type, Piebald, Mosaic, Copper, Lavender, Black Melanoid, White Albino, Speckled Leucistic, Chimera, Heavily Marked Melanoid, Green Fluorescent Protein, Firefly, Enigma,");
    println!("Please type in your desired type: ");
    let mut color = String::new();
    stdin().read_line(&mut color).expect("Failed to read line. Program exiting.");
    let color_type = structs::AxolotlType::from(&color.trim());

    println!("What would you like your axolotls name to be?");
    let mut name = String::new();
    stdin().read_line(&mut name).expect("Failed to read line. Program exiting.");
    name = name.trim().to_string();

    println!("What is {}'s favorite food?", name);
    println!("The choices are: Big Fish, Little Fish, Worms, Pet Food, Other(String)");
    println!("Please type in {}'s disered food: ", name);
    let mut food = String::new();
    stdin().read_line(&mut food).expect("Failed to read line. Profram exiting.");
    let favorite_food = structs::AxolotlFoods::from(&food.trim());

    println!("Who is {}'s owener?", name);
    let mut owner = String::new();
    stdin().read_line(&mut owner).expect("Failed to read line. Program exiting.");
    owner = owner.trim().to_string();

    println!("Where does {} live?", name);
    let mut lives = String::new();
    stdin().read_line(&mut lives).expect("Failed to read line. Program exiting.");
    lives = lives.trim().to_string();

    println!("{} is alive!\n", name);

    vec.append_axolotl(
        color_type, name, favorite_food, owner, lives
    );
}

pub fn ask_for_path() -> String {
    println!("What path do you want to read/write from?");
    let mut path = String::new();
    stdin().read_line(&mut path).expect("Failed to read line. Program exiting.");
    path.trim().to_string()
}