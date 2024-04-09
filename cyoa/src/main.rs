use std::collections::HashMap;
use std::env;
use std::fs;
use serde::Deserialize;
use std::io::{self, Write};
use textwrap::fill;
use slowprint::slow_print;
use std::time::Duration;

// structs for game data types
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
    win: bool
}

#[derive(Debug, Deserialize)]
struct GameData {
    title: String,
    entries: Vec<Entry>,
}

// Function to load game data from embedded JSON string (we have to use embedded string so that cargo includes the json file when building)
fn load_game_data_from_str(json_str: &str) -> Result<GameData, Box<dyn std::error::Error>> {
    let game_data: GameData = serde_json::from_str(json_str)?;
    Ok(game_data)
}

fn main() {
 
let start_msg = r#"
 ██████╗     ██╗   ██╗     ██████╗      █████╗ 
██╔════╝     ╚██╗ ██╔╝    ██╔═══██╗    ██╔══██╗
██║           ╚████╔╝     ██║   ██║    ███████║
██║            ╚██╔╝      ██║   ██║    ██╔══██║
╚██████╗HOOSE   ██║OUR    ╚██████╔╝WN  ██║  ██║DVENTURE
 ╚═════╝        ╚═╝        ╚═════╝     ╚═╝  ╚═╝
                                  
"#;
    println!("{}", start_msg);
   let game_data_json_str = include_str!("game_data.json");

    let game_data = match env::args().nth(1) {
        Some(file_path) => {
            if let Ok(json_content) = fs::read_to_string(&file_path) {
                match load_game_data_from_str(&json_content) {
                    Ok(data) => data,
                    Err(e) => {
                        eprintln!("Error loading game data from file {}: {}. Does the file contain valid json for this program?", &file_path, e);
                        return;
                    }
                }
            } else {
                eprintln!("Error reading game data from file: {}. Make sure it exists.", &file_path);
                return;
            }
        }
        None => {
            match load_game_data_from_str(game_data_json_str) {
                Ok(data) => data,
                Err(e) => {
                    eprintln!("Error loading game data: {}", e);
                    return;
                }
            }
        }
    };
println!("{}", game_data.title);
    let mut current_entry_id = 1;

  // main loop
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

}