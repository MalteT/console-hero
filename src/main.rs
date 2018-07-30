extern crate regex;
extern crate rustyline;
extern crate serde;
extern crate serde_json;
extern crate textwrap;
#[macro_use]
extern crate serde_derive;
extern crate colored;
extern crate pad;
extern crate unicode_width;
#[macro_use]
extern crate clap;
#[macro_use]
extern crate version;
extern crate d20;

mod data;
mod die;
#[cfg(test)]
mod tests;

use clap::App;
use data::Data;
use rustyline::error::ReadlineError;
use std::io;

fn main() -> io::Result<()> {
    let cli_yaml = load_yaml!("../cli.yml");
    let matches = App::from_yaml(cli_yaml).version(version!()).get_matches();

    // Load data
    let data = Data::from(
        matches.value_of("move_data").unwrap(),
        matches.value_of("monster_data").unwrap(),
        matches.value_of("tag_data").unwrap(),
        matches.value_of("item_data").unwrap(),
    )?;

    // Execute single command if specified
    let mut subcommand_given = true;
    if let Some(matches) = matches.subcommand_matches("item") {
        search_item(&data, matches.value_of("REGEX").unwrap());
    } else if let Some(matches) = matches.subcommand_matches("monster") {
        search_monster(&data, matches.value_of("REGEX").unwrap());
    } else if let Some(matches) = matches.subcommand_matches("move") {
        search_move(&data, matches.value_of("REGEX").unwrap());
    } else if let Some(matches) = matches.subcommand_matches("tag") {
        search_tag(&data, matches.value_of("REGEX").unwrap());
    } else if let Some(matches) = matches.subcommand_matches("roll") {
        roll_dice(matches.value_of("D20_EXPR").unwrap());
    } else {
        subcommand_given = false;
    }

    // Enter interactive mode if no command is given or --interactive is specified
    if !subcommand_given || matches.is_present("interactive") {
        // Interactive
        let mut rl = rustyline::Editor::new()
            .history_ignore_dups(true)
            .history_ignore_space(true);
        rl.set_completer(Some(&data));

        // Loop until user wants to exit
        loop {
            // Read the input
            let readline = rl.readline(" > ");
            let readline = match readline {
                Ok(line) => {
                    rl.add_history_entry(&line);
                    line
                }
                Err(ReadlineError::Interrupted) | Err(ReadlineError::Eof) => break,
                Err(err) => {
                    println!("Error: {:?}", err);
                    break;
                }
            };

            match readline.as_str() {
                "quit" | "exit" | "q" => break,
                "help" | "info" => print_help(),
                m if m.starts_with("roll") => roll_dice(m),
                m if m.starts_with("item ") => search_item(&data, m),
                m if m.starts_with("monster ") => search_monster(&data, m),
                m if m.starts_with("move ") => search_move(&data, m),
                m if m.starts_with("tag ") => search_tag(&data, m),
                _ => {}
            }
        }
    }

    Ok(())
}

/// Prints some usage information about the interactive mode.
fn print_help() {
    println!(
        r#"
COMMANDS:
    help | info       Print this usage information
    quit              Exit interactive mode
    item    REGEX     Find the first item matching the given REGEX
    monster REGEX     Find the first monster matching the given REGEX
    move    REGEX     Find the first move matching the given REGEX
    tag     REGEX     Find the first tag matching the given REGEX
"#
    );
}

/// Try to parse the given string into a dice roll
fn roll_dice<'a>(s: &'a str) {
    let s = s.trim_left_matches("roll ");
    die::roll(s);
}

/// Search for an item
fn search_item(data: &Data, re: &str) {
    let item = re.trim_left_matches("item ");
    let item = data.items.find(item);
    match item {
        Some(item) => println!("{}", item),
        None => println!("No match"),
    }
}

/// Search for a monster
fn search_monster(data: &Data, re: &str) {
    let monster = re.trim_left_matches("monster ");
    let monster = data.monsters.find(monster);
    match monster {
        Some(monster) => println!("{}", monster),
        None => println!("No match"),
    }
}

/// Search for a move
fn search_move(data: &Data, re: &str) {
    let mv = re.trim_left_matches("move ");
    let mv = data.moves.find(mv);
    match mv {
        Some(mv) => println!("{}", mv),
        None => println!("No match"),
    }
}

/// Search for a tag
fn search_tag(data: &Data, re: &str) {
    let tag = re.trim_left_matches("tag ");
    let tag = data.tags.find(tag);
    match tag {
        Some(tag) => println!("{}", tag),
        None => println!("No match"),
    }
}
