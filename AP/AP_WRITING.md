# AP Create Task Written Responses

## 3a - Questions related to the overall program
### I. - Describe the creative expression that the program pursues.
- This program was created to provide a simple way for people to access Choose-Your-Own-Adventure style games from the comfort of the command line, inspired by early computer games where the terminal was all that was available. The program is easily run with a single command, and includes a default story called CSP Quest, as well as functionality for scalable custom stories that can be as long or short as the creator wants. Such stories can be easily made with a GUI in the second part of the project, a ReactJS web app. Responses to further questions will focus more on the terminal application rather than the web app.
### II. - What is the functionality of your program?
- The program is written in Rust for increased performance over interpreted languages like Python or Javascript, and for its increased memory safety over other compiled languages like C. The reader loads game data from a JSON file, which is either supplied by the user via environment arguments or a built in default story if no arguments are supplied. Once the data is loaded, the program verifies the structure and types to ensure that the data was meant for this application (and not just a random file). After verification the program prints a the title of the story, read from the data, and begins playing the story. The program prints a splash message, then the creator-defined story title, before iterating through every entry under the entries sub-list. For each entry that is printed, the user can choose one of the 2 options using the Standard Input made available by the system. The number that the user chooses determines which entry is displayed next. If there are no options, the program assumes it is the end of the story, and checks the win field to determine what is displayed. If win is true, a message stating that the user has won will be displayed, and a game over message if it is false.
### III. - Describe at least one input and output of the program.
- There are two main inputs for this program. The first is the initial loading of the JSON file where the list is first introduced to the program, and the second is the user selecting their choice, which happens every time an option is presented. The output of the program is the Standard Output, which is being accessed using the println! macro and displays the title and options to the user.
### IV. - Describe one piece of documentation that would be appropriate to include with or in your program.
- I’ve written a README.md file that has documentation for both general use and development of each of the two parts of this project (CLI reader program and webapp for GUI based story creation). The purpose of this file is to accurately display information about both parts of the program and their use, hopefully answering any questions that the user has. It is accessible in the repository of this project on Github, bean-frog/text-adventure.

## 3b - Questions related to the list being used in the program
### I. - Example of the list.
- Below is a snippet of the JSON file that contains story data for the built in story. A JSON file is fundamentally the same as a list, just in a separate file that needs to be loaded and parsed before use. Parts have been omitted, indicated by 3 dots like so: ...
```json
{
  "title": "CSP Quest\n",
  "entries": [
    {
      "id": 1,
      "text": "You slowly gain consciousness. As your eyes open, you discover that you are in an abandoned laboratory, ...",
      "options": {
        "1": {
          "text": "Search the room",
          "next_id": 2
        },
        "2": {
          "text": "Leave the room immediately",
          "next_id": 9
        }
      },
      "win": false
    }
    ... (many more entries exist, formatted similarly)
  ]
}
```
- Below is the Rust code responsible for the loading and validation of this data. Validation is handled by serde, an external crate ("crate" is the name for external code in rust, similar to a library).
```rust
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

fn load_game_data_from_str(json_str: &str) -> Result<GameData, Box<dyn std::error::Error>> {
    let game_data: GameData = serde_json::from_str(json_str)?;
    Ok(game_data)
}
```
### II. - Identify the name of the list being used in this response and describe what the data contained in the list represents in your program.
- Once loaded, the data within the JSON file is stored with the name "game_data".
### III. - Explain how the selected list manages complexity in your program.
- Without this JSON file, the program would have to have the entire story hardcoded. Not only would this drastically increase the file size, it would make it essentially impossible to have the level of expandability and freedom that the current program has. In its current state, the program can read any story with one simple requirement: the JSON data must simply match the correct structure and types. If the story data were hardcoded, the only way to expand or change the story would be to edit main.rs and rebuild the application from source which is both difficult and unfeasible for the average end user.

## 3c - Questions related to the custom block (procedure) you developed that has an input and implements an algorithm
### I. - Student-developed procedure
- Below is the function that gets and prints the available options to the user.
```rust
fn print_options(entry_id: &Entry) {
    println!("\nChoose an option:");
    for (i, option) in entry_id.options.iter() {
        println!("{}. {}", i, option.text);
    }
    println!("> ")
}

```
- And here is the code surrounding the call to this function in the main function.
```rust
  if current_entry.win == true {
            return;
        } else {
            print_options(current_entry);

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
[...]
}
```
### II. - Explain in detailed steps how the algorithm implemented in the identified procedure works.
- The given procedure has one argument called entry_id, which is of a custom type called Entry, defined in one of the same structs used to validate the data. When called, the function prints out a prompt to the user, before iterating over every item contained in the options list of the given entry. For every option it finds, the number and associated text is printed out to the standard output. After that, it prints a character that prompts the user to enter an option, which is processed and validated outside of the function, part of which is also shown in the second snippet.
- To provide an example, if the options for a given entry are as follows:
```json
"options": {
    "1": {
      "text": "Do something",
      "next_id": 2
    },
    "2": {
      "text": "Do something else",
      "next_id": 3
    }
}
```
- then the function would be responsible for printing this to the user:
```
Choose an option:
1. Do something
2. Do something else
> 
```
### III - Consider the first iteration statement included in your procedure. Identify the number of times the body of your iteration statement will execute. Describe a condition or error that would cause your iteration statement to not terminate and cause an infinite loop. If no such condition or error exists, explain how the loop could be modified to cause an infinite loop.
- The iteration in this function is the for statement. This executes an arbitrary number of times, defined by how many options the creator of the story has defined for that given entry. However, in the default story, this number is always 2. In Rust, the built in function iter() is used to signify iteration over an array or vector type. If the loop was modified so that iter() is not used, the for loop would never iterate, staying stuck in place. Below is a rewritten version without iter() that would likely cause an error like what was described.
```rust
fn print_options(entry_id: &Entry) {
    println!("\nChoose an option:");
    for (i, option) in entry_id.options {
        println!("{}. {}", i, option.text);
    }
    println!("> ");
}
```

### IV.
-
## 3d
### I.
- 
### II.
-
### III.
-