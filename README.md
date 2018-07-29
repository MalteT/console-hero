# Console-Hero

A [Dungeon World](http://www.dungeon-world.com/) command line helper.
But has autocompletion and history, thanks to [rustyline](https://github.com/kkawakam/rustyline).
The data used in this project comes from the [DungeonWorldData](https://github.com/Vindexus/DungeonWorldData) project.

## Note

As of yet only supports querying moves, monsters, tags and items.

## Build

Build and execute with `cargo run --release`.

## Usage

```
> cargo run -- help

USAGE:
    console-hero [FLAGS] [OPTIONS] [SUBCOMMAND]

FLAGS:
    -h, --help           Prints help information
    -I, --interactive    Enter interactive mode (default, if no subcommand is given)
    -V, --version        Prints version information

OPTIONS:
    -i, --item_data <FILE>       Specify a custom item data file [default: data/items.json]
    -m, --monster_data <FILE>    Specify a custom monster data file [default: data/monsters.json]
    -o, --move_data <FILE>       Specify a custom move data file [default: data/moves.json]
    -t, --tag_data <FILE>        Specify a custom tag data file [default: data/tags.json]

SUBCOMMANDS:
    help       Prints this message or the help of the given subcommand(s)
    item       Find the first item matching the given regex
    monster    Find the first monster matching the given regex
    move       Find the first move matching the given regex
    tag        Find the first tag matching the given regex
```

## Example
The image misses some colors, but they already work!

![Usage example](screenshot/screenshot.png)


## TODO

- [x] Add moves from [moves.json](data/moves.json)
- [x] Add monsters from [monsters.json](data/monsters.json)
- [x] ~~Add missing moves from [special_moves.json](data/special_moves.json) and [basic_moves.json](data/basic_moves.json)~~ Not relevant.
- [x] Add tags from [tags.json](data/tags.json)
- [ ] Add classes from [classes.json](data/classes.json)
- [x] Add items from [items.json](data/items.json)
- [ ] Complete completion (commands, ...)
- [x] Add option to evaluate commands non-interactively
- [x] Add help command `--help`/`help`
- [x] Unify formatting
- [ ] Add tests
- [ ] Add option to explain all attributes of a thing.
      Something like `tags of ...` to list tags from `...` with description.I
- [ ] Add a dice already, will you?!

---

Thanks to the creators of Dungeon World!
