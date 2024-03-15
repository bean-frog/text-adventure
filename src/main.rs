use std::collections::HashMap;
use std::fs;
use serde::Deserialize;
use std::io::{self, Write};
use textwrap::fill;
use slowprint::slow_print;
use std::time::Duration;

// structs for game data
#[derive(Debug, Deserialize)]
struct Option {
    text: String,
    next_id: u32,
}

#[derive(Debug, Deserialize)]
struct Entry {
    id: u32,
    text: String,
    options: HashMap<u32, Option>,
}

#[derive(Debug, Deserialize)]
struct GameData {
    entries: Vec<Entry>,
}

// function to load game data and parse with serde
fn load_game_data(filename: &str) -> Result<GameData, Box<dyn std::error::Error>> {
    let contents = fs::read_to_string(filename)?;
    let game_data: GameData = serde_json::from_str(&contents)?;
    Ok(game_data)
}

fn main() {
 let start_msg = r#"
         ██████╗███████╗██████╗      ██████╗ ██╗   ██╗███████╗███████╗████████╗
        ██╔════╝██╔════╝██╔══██╗    ██╔═══██╗██║   ██║██╔════╝██╔════╝╚══██╔══╝
        ██║     ███████╗██████╔╝    ██║   ██║██║   ██║█████╗  ███████╗   ██║   
        ██║     ╚════██║██╔═══╝     ██║▄▄ ██║██║   ██║██╔══╝  ╚════██║   ██║   
        ╚██████╗███████║██║         ╚██████╔╝╚██████╔╝███████╗███████║   ██║   
         ╚═════╝╚══════╝╚═╝          ╚══▀▀═╝  ╚═════╝ ╚══════╝╚══════╝   ╚═╝   
                                                                       
    "#;
    println!("{}", start_msg);
    let filename = "src/game_data.json";
    let game_data = match load_game_data(filename) {
        Ok(data) => data,
        Err(e) => {
            eprintln!("Error loading game data: {}", e);
            return;
        }
    };
    let mut current_entry_id = 1;

    // main game loop

loop {
    let current_entry = match game_data.entries.iter().find(|e| e.id == current_entry_id) {
        Some(entry) => entry,
        None => {
            eprintln!("Entry with id {} not found.", current_entry_id);
            break;
        }
    };

let wrapped_text = fill(&current_entry.text, 100);
let delay = Duration::from_millis(10); 
slow_print(&wrapped_text, delay);

    if current_entry.options.is_empty() {
        break;
    }

    println!("\nChoose an option:");

    for (i, option) in current_entry.options.iter() {
        println!("{}. {}", i, option.text);
    }

    print!("> ");
    io::stdout().flush().unwrap();

    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let choice: u32 = match input.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Invalid input. Please enter a number.");
            continue;
        }
    };

    match current_entry.options.get(&choice) {
        Some(option) => {
            current_entry_id = option.next_id;
        }
        None => {
            println!("Invalid option. Please choose a valid option.");
        }
    }
}


    println!("\nGame over!");
}
