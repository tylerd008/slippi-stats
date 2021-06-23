# slippi-stats
While the slippi game browser gives a decent analysis of each .slp file, it doesn't include any utilities for aggregating stats over all your games. This tool seeks to provide a means to gather aggregate stats and present as a command based terminal program using [peppi](https://github.com/hohav/peppi).

## Building
There is no release yet, as I would like to add a bit more before publishing a release to minimize cache rebuildings, so you'll have to build it yourself for now. To do this install rustup and download the source code. This uses nightly rust, so set the source's directory to nightly rust with the command `rustup override set nightly`. Then run `cargo build --release` to build the program.

## Usage
Run the .exe. You'll be prompted for yournetplay code, then the path where your replays are stored. After this the program will process all the replays, extracting data from them. This could take a while on a first run before it processes all of them, but on subsequent runs it won't as the data gets cached. Then from there you can use the commands to get data.

## Current Utilites
Currently there are four main commands:
- `player` - Commands for getting overall data for the player: `winrate`, `matchups`, `overview`.
- `character` - Commands for getting stats of a certain character: `winrate`, `stages`, and `matchups`.
- `stage` - Commands for data of a certain stage: `winrate`, `characters`, and `matchups`.
- `matchup` - Gives data for a given matchup.
- `last` - Prints the results of the last given number of games.
- `change cache` - Load data from a different directory.

## Future Plans
- More detailed stats involving moves used, damage dealt, and stocks taken.
- Some sort of better presentation.

## License
This program uses the Apache 2.0 license.