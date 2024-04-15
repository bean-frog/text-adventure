use serde::Deserialize;
use slowprint::slow_print;
use std::collections::HashMap;
use std::env;
use std::fs;
use std::io::{self, Write};
use std::time::Duration;
use textwrap::fill;

// defining all multiline strings here (instead of before theyre called) bc why not
// these big letters look really cool fr
// maybe have the win/gameover messages be customizable by the creator??
static START_MSG: &str = r#"
 ██████╗     ██╗   ██╗     ██████╗      █████╗ 
██╔════╝     ╚██╗ ██╔╝    ██╔═══██╗    ██╔══██╗
██║           ╚████╔╝     ██║   ██║    ███████║
██║            ╚██╔╝      ██║   ██║    ██╔══██║
╚██████╗HOOSE   ██║OUR    ╚██████╔╝WN  ██║  ██║DVENTURE
 ╚═════╝        ╚═╝        ╚═════╝     ╚═╝  ╚═╝
                                  
"#;
static GAMEOVER_MSG: &str = r#"
                                                                       ,---. 
 ,----.                                 ,-----.                        |   | 
'  .-./    ,--,--.,--,--,--. ,---.     '  .-.  ',--.  ,--.,---. ,--.--.|  .' 
|  | .---.' ,-.  ||        || .-. :    |  | |  | \  `'  /| .-. :|  .--'|  |  
'  '--'  |\ '-'  ||  |  |  |\   --.    '  '-'  '  \    / \   --.|  |   `--'  
 `------'  `--`--'`--`--`--' `----'     `-----'    `--'   `----'`--'   .--.  
                                                                       '--'  
"#;
static WIN_MSG: &str = r#"
                                                      ,---. 
,--.   ,--.                 ,--.   ,--.               |   | 
 \  `.'  /,---. ,--.,--.    |  |   |  | ,---. ,--,--, |  .' 
  '.    /| .-. ||  ||  |    |  |.'.|  || .-. ||      \|  |  
    |  | ' '-' ''  ''  '    |   ,'.   |' '-' '|  ||  |`--'  
    `--'  `---'  `----'     '--'   '--' `---' `--''--'.--.  
                                                      '--'                                                          
"#;
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
    win: bool,
}

#[derive(Debug, Deserialize)]
struct GameData {
    title: String,
    entries: Vec<Entry>,
}

// Function to load game data from embedded JSON string (for whatever reason cargo only includes it in the build if we do it this way)
fn load_game_data_from_str(json_str: &str) -> Result<GameData, Box<dyn std::error::Error>> {
    let game_data: GameData = serde_json::from_str(json_str)?;
    Ok(game_data)
}

fn main() {
    println!("{}", START_MSG);

    let game_data_json_str = include_str!("game_data.json"); // initial value for JSON. overwritten if provided path exists/is valid.

    //get first environment arg and determine if its a valid path
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
                eprintln!(
                    "Error reading game data from file: {}. Make sure it exists.",
                    &file_path
                );
                return;
            }
        }
        //user is stupid and/or can't read and supplied a bad file. Use default instead, because we have to do everything for people these days smh
        None => match load_game_data_from_str(game_data_json_str) {
            Ok(data) => data,
            Err(e) => {
                eprintln!("Error loading game data: {}", e);
                return;
            }
        },
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
            if current_entry.win == true {
                println!("{}", WIN_MSG);
                break;
            } else if current_entry.win == false {
                println!("{}", GAMEOVER_MSG);
                println!("\nWould you like to go back one choice and retry? (y/n)");
                let mut retry_choice = String::new();
                io::stdin()
                    .read_line(&mut retry_choice)
                    .expect("Failed to read line");
                if retry_choice.trim().to_lowercase() == "y" {
                    if let Some(prev_entry_id) = game_data
                        .entries
                        .iter()
                        .find(|&entry| {
                            entry
                                .options
                                .values()
                                .any(|opt| opt.next_id == current_entry_id)
                        })
                        .map(|entry| entry.id)
                    {
                        current_entry_id = prev_entry_id;
                        continue;
                    }
                } else {
                    break;
                }
            } else {
                eprintln!("Hmm, looks like the JSON was configured incorrectly. Win key for entry {} is either nonexistent or not a boolean (true/false)", current_entry_id)
            }
        }
        if current_entry.win == true {
            return;
        } else {
            println!("\nChoose an option:");

            for (i, option) in current_entry.options.iter() {
                println!("{}. {}", i, option.text);
            }

            print!("> ");
            io::stdout().flush().unwrap();

            let mut input = String::new();
            io::stdin()
                .read_line(&mut input)
                .expect("Failed to read line");
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
}
