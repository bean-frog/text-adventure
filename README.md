# text-adventure
#### A choose your own adventure reader written in Rust, made for the 2024 APCSP Create Task

## General Information
	+ project contains 2 main parts: Reader (./cyoa), and Creator (./creator)
	+ The reader is a CLI program written in Rust, designed to display a choose your own adventure style game from a json file.
	+ The creator is a webapp built with React and Tailwind that is designed to provide a graphical way to create reader-compatible stories.
	+ More advanced users can create stories manually with json, as long as it matches this format:

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
      }
    }
    ... add as many entries as you want
   ]
}
```
	+ The JSON data is infinitely scalable, with as many entries as you want.

## Reader information
	+ code is under ./cyoa
	+ built executables for Arch Linux and Windows will be available in the Releases tab of this repo.
	+ if these fail to run for any reason, building locally is your best option. Here's how:
		+ Install Cargo with your system's package manager (Pacman, Yay, Apt, Winget, Chocolatey, etc)
		+ clone this repository with git or `cd` to it if you have already.
		+ From ./cyoa, run `cargo build --release`
		+ Above step might take a while, beacuse Rust is like that.
		+ Once built, the native executable can be found under /target/release
	+ The executable can be run through the command line with the command `./text-adventure <arg>`. If no args are supplied, a default built-in story will be used. 
	+ To run your own story, simply provide the path to the json file as the arg, including the file extension (i.e. ./text-adventure path/to/your/story.json). If it's incorrectly formatted or does not exist at that location, you'll get a message.

## Creator information 
	+ I'll probably create a public build on my website to save some hassle, but if you really want to do it yourself here's how:
	+ Install npm
	+ clone this repository with git or `cd` to it if you have already.
	+ from ./creator, run `npm install` to get dependencies
	+ now run either `npm run start` for a live server (localhost:3000), or `npm run build` for an optimized production build which will be placed in /dist
