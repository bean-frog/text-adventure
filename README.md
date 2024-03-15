# text-adventure
a text based adventure game written in rust for the 2024 AP CSP Create Task

plot:
You awaken in an abandoned laboratory that once hosted groundbreaking experiments in cyborg technology. 
however, as the lab stayed in ruin, several mutated creatures developed out of the ashes of the once bustling laboratory.
these mutants are armed to the teeth with computer science powers, and will do everything they can to stop your escape.

can you survive long enough to escape the lab?


## dev info
- the program is designed to work with any json file as long as it follows the same formatting.
- the json is (theoretically) infinitely scalable, with as many stages as you want.
- JSON format:
```
{
  "entries": [

{
      "id": 1,
      "text": "The text that is typed out to the user",
      "options": {
        "1": {
          "text": "The user can choose this to go to entry 2",
          "next_id": 2
        },
        "2": {
          "text": "The user can also choose this to go to entry 3",
          "next_id": 3
        }
      }
    }
    ... add as many entries as you want
   ]
}
```
- diagram.drawio contains the flowchart that I used to organize my thoughts, and can be opened on app.diagrams.net