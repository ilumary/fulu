# Fulu
## _Best Recipe Manager, Ever_

[![Build Status](https://travis-ci.org/joemccann/dillinger.svg?branch=main)](https://travis-ci.org/joemccann/dillinger)

Fulu is a Recipe Manager which intends to replace that old and crammed Book from your Grandma. It is purely written in Rust.

## Features (as of now)

- Sort your Recipes in Collections, each saved as a file
- Save and Load from Disk 
- View your Recipes in a stunning UI

Planned: 

- Editing of recipes within UI
- Add Photos to recipes
- Export Recipes to different File Formats

## Tech

Fulu uses a number of Rust Crates to enhance your experience:

- [egui] - Rust gui library
- [rfd] - File browser dialogs
- [recipeapi] - For managing your recipes

## Installation

Fulu Requires a working Rust compiler. I use version 1.57 of [Rustc](https://nodejs.org/).

Clone the repository and type following command inside the folder:
```sh
cargo run
```

## Development

Fulu is as of now in active Development. 

The whole intention of this project for me is to learn rust, so please don't take this to seriously.

Also I want to beat my little brother in a contest about who programs the better Recipe Manager. Obviously mine is better.

## License

You are free to do with my code whatever you want

[egui]: <https://github.com/joemccann/dillinger>
[rfd]: <https://github.com/joemccann/dillinger.git>
[recipeapi]: <https://github.com/ilumary/fulu/tree/main/recipeapi>