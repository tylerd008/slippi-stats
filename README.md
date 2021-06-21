# slippi-stats
While the slippi game browser gives a decent analysis of each .slp file, it doesn't include any utilities for aggregating stats over all your games. This tool seeks to provide a means to gather aggregate stats and present as a command based terminal program using [peppi](https://github.com/hohav/peppi).

## Building
There is no release yet, as I would like to add a bit more before publishing a release to minimize cache rebuildings, so you'll have to build it yourself for now. To do this install rustup and download the source code. This uses nightly rust, so set the source's directory to nightly rust with the command `rustup override set nightly`. Then run `cargo build --release` to build the program.

## Usage
The program uses 2 command line arguments to run, the first being your Slippi netplay code, and the second being the directory where your replays are stored. So for a player with the code PLYR#123 and replays in B:\replays, they would run `slippi_stats.exe PLYR#123 B:\replays`. Then after that you will be presented with the command prompts to retrieve various data. Type `help` at any point using the program if you aren't sure what you can do.  
Do note that on a first run it will take some time before you can get to the input part of the program. This is because the program has to run through all the data of every replay in the directory to get data points from it. But after that the data will be stored in a cache file in the replays directory, so subsequent uses while not take as long to start up. However, in the future if more data gets added, the cache will have to be rebuilt entirely to accomodate.

## Current Utilites
Currently there are four main commands:
- `player` - Commands for getting overall data for the player: `winrate`, `matchups`, `overview`.
- `character` - Commands for getting stats of a certain character: `winrate`, `stages`, and `matchups`.
- `stage` - Commands for data of a certain stage: `winrate`, `characters`, and `matchups`.
- `matchup` - Gives data for a given matchup.
- `last` - Prints the results of the last given number of games.

## Future Plans
- More detailed stats involving moves used, damage dealt, and stocks taken.
- Some sort of better presentation.

## License
This program uses the Apache 2.0 license.