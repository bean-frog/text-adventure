# Part 1: CLI-based Reader application (Rust)
```rust
use serde::Deserialize;
use slowprint::slow_print;
use std::collections::HashMap;
use std::env;
use std::fs;
use std::io::{self, Write};
use std::time::Duration;
use textwrap::fill;

// several large multiline strings of ASCII-art lettering are here in the actual program. They have been ommitted here.
static START_MSG: &str = r#"                          
"#;
static GAMEOVER_MSG: &str = r#"                                                                       
"#;
static WIN_MSG: &str = r#"                                                                                                           
"#;

// structs for game data type validation
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

```
# Part 2: GUI-based Creator web app (React/Tailwind)
```jsx
import React, { useState } from "react";

const initialData = {
  id: 1,
  text: "here's an option text. this is where you can ask the player something or give them information.",
  options: {
    1: {
      text: "here's an option.",
      next_id: 2,
    },
    2: {
      text: "here's another option. ",
      next_id: 3,
    },
  },
  win: false,
};

const App = () => {
  const [title, setTitle] = useState("An Unnamed Story");
  const [formData, setFormData] = useState({ ...initialData });
  const [cards, setCards] = useState([]);
  const [showJson, setShowJson] = useState(false);

  const handleChange = (e) => {
    const { name, value } = e.target;
    setFormData((prevData) => ({
      ...prevData,
      [name]: value,
    }));
  };

  const addCard = (e) => {
    const newCard = { ...formData, id: getMaxId() + 1 };
    setCards((prevCards) => [...prevCards, newCard]);
  };

  const getMaxId = () => {
    return cards.reduce(
      (maxId, card) => (card.id > maxId ? card.id : maxId),
      0,
    );
  };

  const handleOptionChange = (e, optionId, cardId) => {
    const { name, value } = e.target;
    setCards((prevCards) =>
      prevCards.map((card) => {
        if (card.id === cardId) {
          return {
            ...card,
            options: {
              ...card.options,
              [optionId]: {
                ...card.options[optionId],
                [name]: value,
              },
            },
          };
        }
        return card;
      }),
    );
  };

  const handleTitleChange = (e, cardId) => {
    const { value } = e.target;
    setCards((prevCards) =>
      prevCards.map((card) => {
        if (card.id === cardId) {
          return {
            ...card,
            text: value,
          };
        }
        return card;
      }),
    );
  };

  const toggleJsonView = () => {
    setShowJson((prevShowJson) => !prevShowJson);
  };

  const deleteCard = (id) => {
    setCards((prevCards) => prevCards.filter((card) => card.id !== id));
  };

  const handleEndingChange = (e, cardId) => {
    const isChecked = e.target.checked;
    setCards((prevCards) =>
      prevCards.map((card) => {
        if (card.id === cardId) {
          return {
            ...card,
            options: isChecked ? {} : initialData.options,
          };
        }
        return card;
      }),
    );
  };
  const handleChangeTitle = (e) => {
    setTitle(e.target.value);
  };

  const downloadJson = () => {
    const json = JSON.stringify({ title: title, entries: cards }, null, 2);
    const blob = new Blob([json], { type: "application/json" });
    const url = URL.createObjectURL(blob);
    const a = document.createElement("a");
    a.href = url;

    if (title) {
      let fixedTitle = title.trim().toLowerCase().replace(/ /g, "_");
      a.download = fixedTitle + ".json";
    } else {
      a.download = "data.json";
    }

    a.click();
    URL.revokeObjectURL(url);
  };

  return (
    <div className="flex flex-row justify-center h-screen w-screen bg-stone-900">
      <div className="w-1/2 p-4 bg-stone-500 text-white">
        <button
          onClick={addCard}
          className="bg-emerald-400/50 border-2 border-white text-white px-4 py-2 mr-2 rounded"
        >
          Add Card
        </button>
        <button
          onClick={toggleJsonView}
          className="bg-red-400/50 border-2 border-white text-white px-4 py-2 rounded mt-4"
        >
          {showJson ? "View Cards" : "View JSON"}
        </button>
        <button
          onClick={downloadJson}
          className="bg-sky-500/50 border-2 border-white text-white px-4 ml-2 py-2 rounded mt-4"
        >
          Download
        </button>
        <input
          type="text"
          value={title}
          onChange={handleChangeTitle}
          className="font-semibold text-xl bg-stone-600 border-2 border-white text-white rounded px-2 py-1 w-full mt-2"
          placeholder="Enter a cool title..."
        />
        {showJson ? (
          <>
            <h1 className="text-md text-white mt-2">
              You can manually edit this data, but it isn't recommended unless
              you know what you're doing.
            </h1>
            <pre
              contentEditable={true}
              className="overflow-y-auto h-screen max-h-full bg-stone-600 border-2 border-white text-white rounded px-4 py-2"
            >
              {JSON.stringify({ title: title, entries: cards }, null, 2)}
            </pre>
          </>
        ) : (
          <div className="overflow-y-auto h-screen max-h-full bg-stone-600 border-2 border-white text-white rounded px-4 py-2 mt-4">
            {cards.map((card) => (
              <div
                key={card.id}
                className="bg-stone-500 border border-gray-300 p-3 rounded mt-4"
              >
                <pre>ID: {card.id}</pre>
                <input
                  type="text"
                  value={card.text}
                  onChange={(e) => handleTitleChange(e, card.id)}
                  className="font-semibold text-xl bg-stone-600 border-2 border-white text-white rounded px-2 py-1 w-full mb-2"
                  placeholder="Enter card text..."
                />
                <div className="flex flex-col">
                  <label>
                    Ending:
                    <input
                      type="checkbox"
                      checked={Object.keys(card.options).length === 0}
                      onChange={(e) => handleEndingChange(e, card.id)}
                      className="ml-2 bg-stone-600 border-2 border-white text-white"
                    />
                  </label>
                  {Object.keys(card.options).length === 0 && (
                    <label>
                      Is Win:
                      <input
                        type="checkbox"
                        checked={card.win}
                        onChange={(e) => {
                          const isChecked = e.target.checked;
                          setCards((prevCards) =>
                            prevCards.map((c) => {
                              if (c.id === card.id) {
                                return {
                                  ...c,
                                  win: isChecked,
                                };
                              }
                              return c;
                            }),
                          );
                        }}
                        className="ml-2 bg-stone-600 border-2 border-white text-white"
                      />
                    </label>
                  )}
                </div>
                <ul>
                  {Object.keys(card.options).map((optionId) => (
                    <li
                      key={optionId}
                      className="mb-2 w-full transition-all duration-300"
                    >
                      <input
                        type="text"
                        value={card.options[optionId].text}
                        onChange={(e) =>
                          handleOptionChange(e, optionId, card.id)
                        }
                        name="text"
                        className="bg-stone-600 border-2 border-white text-white rounded px-2 py-1 w-96"
                        placeholder="Enter option text..."
                      />
                      <label className="ml-2">Next ID:</label>
                      <input
                        type="number"
                        value={card.options[optionId].next_id}
                        onChange={(e) =>
                          handleOptionChange(e, optionId, card.id)
                        }
                        name="next_id"
                        className="bg-stone-600 border-2 border-white text-white rounded px-2 py-1 w-20"
                        placeholder="Next ID"
                      />
                    </li>
                  ))}
                </ul>
                <button
                  onClick={() => deleteCard(card.id)}
                  className="bg-red-500 text-white px-2 py-1 rounded"
                >
                  Delete
                </button>
              </div>
            ))}
          </div>
        )}
      </div>
    </div>
  );
};

export default App;

```
