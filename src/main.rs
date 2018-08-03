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

mod completion;
mod data;
mod die;
#[cfg(test)]
mod tests;

use clap::App;
use completion::HeroCompleter;
use data::Data;
use rustyline::error::ReadlineError;
use std::io;

fn main() -> io::Result<()> {
    let cli_yaml = load_yaml!("../cli.yml");
    let app = App::from_yaml(cli_yaml).version(version!());
    let matches = app.clone().get_matches();

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
    } else if let Some(matches) = matches.subcommand_matches("list") {
        list(
            &data,
            matches.value_of("CATEGORY").unwrap(),
            matches.value_of("REGEX").unwrap(),
        );
    } else {
        subcommand_given = false;
    }

    if !subcommand_given || matches.is_present("interactive") {
        interactive(data)?;
    }

    Ok(())
}

/// Interactive mode.
fn interactive(data: Data) -> io::Result<()> {
    // Initialize clap
    let yaml_config = load_yaml!("../interactive.yml");
    let mut app = App::from_yaml(yaml_config).version(version!());

    // Initialize Rustyline
    let mut rl = rustyline::Editor::new()
        .history_ignore_dups(true)
        .history_ignore_space(true);
    let compl = HeroCompleter::new(&data);
    rl.set_completer(Some(compl));

    // Loop until the user wants to exit
    loop {
        // Read the next input line
        let matches = match rl.readline(" > ") {
            Ok(line) => {
                rl.add_history_entry(&line);
                let mut args = vec!["console_hero"];
                args.extend(line.split(" "));
                match app.clone().get_matches_from_safe(args) {
                    Ok(matches) => matches,
                    Err(_) => {
                        app.print_help()
                            .map_err(|e| io::Error::new(io::ErrorKind::InvalidInput, e))?;
                        continue;
                    }
                }
            }
            Err(ReadlineError::Interrupted) | Err(ReadlineError::Eof) => break,
            Err(err) => {
                println!("Error: {:?}", err);
                break;
            }
        };

        let concat = |s, arg| {
            if s == String::new() {
                format!("{}", arg)
            } else {
                format!("{} {}", s, arg)
            }
        };
        // Parse input
        if let Some(matches) = matches.subcommand_matches("item") {
            let re = matches
                .values_of("REGEX")
                .unwrap()
                .fold(String::new(), concat);
            search_item(&data, &re);
        } else if let Some(matches) = matches.subcommand_matches("monster") {
            let re = matches
                .values_of("REGEX")
                .unwrap()
                .fold(String::new(), concat);
            println!("{}", re);
            search_monster(&data, &re);
        } else if let Some(matches) = matches.subcommand_matches("move") {
            let re = matches
                .values_of("REGEX")
                .unwrap()
                .fold(String::new(), concat);
            search_move(&data, &re);
        } else if let Some(matches) = matches.subcommand_matches("tag") {
            let re = matches
                .values_of("REGEX")
                .unwrap()
                .fold(String::new(), concat);
            search_tag(&data, &re);
        } else if let Some(matches) = matches.subcommand_matches("roll") {
            roll_dice(matches.value_of("D20_EXPR").unwrap());
        } else if let Some(matches) = matches.subcommand_matches("list") {
            list(
                &data,
                matches.value_of("CATEGORY").unwrap(),
                matches.value_of("REGEX").unwrap(),
            );
        } else if let Some(_) = matches.subcommand_matches("quit") {
            break;
        } else if let Some(_) = matches.subcommand_matches("info") {
            app.print_long_help()
                .map_err(|e| io::Error::new(io::ErrorKind::InvalidInput, e))?;
        }
    }

    Ok(())
}

/// Lists items of the given `category` that match the given `regex`.
fn list(data: &Data, category: &str, regex: &str) {
    match category {
        "monsters" => data.monsters.list(regex),
        "moves" => data.moves.list(regex),
        "items" => data.items.list(regex),
        "tags" => data.tags.list(regex),
        "all" => {
            data.monsters.list(regex);
            data.moves.list(regex);
            data.items.list(regex);
            data.tags.list(regex);
        }
        re if regex == ".*" => {
            // If category is nothing of the above assume it's a regex
            // And rerun this function
            let regex = re;
            let category = "all";
            list(data, category, regex)
        }
        _ => print_help(),
    }
}

/// Prints some usage information about the interactive mode.
fn print_help() {
    println!(
        r#"
COMMANDS:
    help | info            Print this usage information
    quit                   Exit interactive mode
    item    REGEX          Find the first item matching the given REGEX
    monster REGEX          Find the first monster matching the given REGEX
    move    REGEX          Find the first move matching the given REGEX
    tag     REGEX          Find the first tag matching the given REGEX
    list CATEGORY [REGEX]  List all elements of the given CATEGORY matching REGEX

CATEGORY: One of
  - mon[sters]
  - mov[es]
  - t[ags]
  - i[tems]
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
