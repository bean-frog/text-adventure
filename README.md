# text-adventure
#### A choose your own adventure reader written in Rust
+ This project was made for the 2024 APCSP Create Task, though it is released for public use :D
+ I used Rust because its fast and I thought this would be a good opportunity to learn it (first actual app in Rust for me!)
## General Information
+ The project contains 2 main parts: Reader (`./cyoa`), and Creator (`./creator`).
+ The reader is a CLI program written in Rust, designed to display a choose-your-own-adventure style game from a JSON file.
+ The creator is a web app built with React and Tailwind that is designed to provide a graphical way to create stories compatible with the reader.
+ More advanced users can create stories manually with JSON, as long as it matches the specified format.
+ Anyone is also free to make their own compatible creator, as long as the data it outputs matches the format as well.
  + Please note that the `win` key must exist on every entry, regardless of if it's an ending or not. 
  + Also, ensure that the options key also exists in every entry. For ending cards, simply have it be an empty object.
  + The option text supports any characters and formatting that the user's standard output is capable of handling. This includes line breaks (`\n`), and various special characters.
  + For any formatting clarifications, check the JSON section of the creator interface, as it is (probably) always correct.


```
{
  "title": "cool title for your story goes here",
  "entries": [

{
      "id": 1,
      "text": "The text that is typed out to the user",
      "options": {
        "1": {
          "text": "The user can choose this to go to entry 2 for example",
          "next_id": 2
        },
        "2": {
          "text": "The user can also choose this to go to entry 3",
          "next_id": 3
        }
      },
      "win": false
    }
    ... add as many entries as you want
   ]
}
```
+ The JSON data is theoretically infinitely scalable (I don't see any reason why it wouldn't be), with as many entries as you want.

## Reader information
+ code is under `./cyoa`
+ built executables for Arch Linux and Windows will be available in the Releases tab of this repo.
+ if these fail to run for any reason, building locally is your best option. Here's how:
	+ Install Cargo with your system's package manager (Pacman, Yay, Apt, Winget, Chocolatey, etc)
	+ clone this repository with git or `cd` to it if you have already.
	+ From ./cyoa, run `cargo build --release`
	+ Above step might take a while, beacuse Rust is like that.
	+ Once built, the native executable can be found under /target/release
+ The executable can be run through the command line with the command `./text-adventure <arg>`. If no args are supplied, a default built-in story will be used. 
+ To run your own story, simply provide the path to the json file as the arg, including the file extension (i.e. `./text-adventure path/to/your/story.json`). If it's incorrectly formatted or does not exist at that location, you'll get a message.

## Creator information 
+ I'll probably create a public build on my website to save some hassle, but if you really want to do it yourself here's how:
+ Install npm
+ clone this repository with git or `cd` to it if you have already.
+ from `./creator`, run `npm install` to get dependencies
+ now run either `npm run start` for a live server (localhost:3000), or `npm run build` for an optimized production build which will be placed in `/dist`


## Dev info
+ This is basically useless since I know no one cares or will care but here you go for the 0.1%
+ ### Creator
  + React webapp bootstrapped with create-react-app
  + As much of the styling as possible handled with TailwindCSS
  + the tailwind command (from the ./creator directory) is `npx tailwindcss -o src/index.css --watch`
  + Before any commits, run `grunt` to minify css
+ ### Reader
  + Code is pretty messy since I barely know rust
  + Do not expect adherence to best practices lol
  + uhhhhhhhhhhh idk
