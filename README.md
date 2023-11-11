# AutomaTomas

is an improvement of the [AutomaTom](https://github.com/it-2001/AutomaTom) cellular automaton engine.

While Automatom is an engine that uses Lua to define the rules of the automaton, AutomaTomas is a game with pre-defined rules. This allows for far more complex and interesting automata to be created as well as almost 1000x speed improvement.

## Installation

### Windows

Download the latest release from the [releases page](https://github.com/it-2001/automatomas/releases) and run the executable.

### Linux

The Linux version is not yet available. You can build it yourself by cloning the repository and running `cargo build --release` in the root directory. The executable will be located in `target/release/automatomas`.

## How to play

### Controls

- `Left click` - Place a cell
- `Right click` - Remove a cell

> Note: If no cell type is selected, left clicking will trigger the mechanics of clicked cell.

### Settings

The left panel contains the settings for the automaton. Try them all out to see what they do!

### Cell types

The right panel contains the cell types. Click on a cell type to select it. Then, click on a cell to change it to the selected type.