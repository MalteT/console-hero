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

mod data;

use data::Data;
use rustyline::error::ReadlineError;
use std::io;

fn main() -> io::Result<()> {
    let data = Data::from("data/moves.json", "data/monsters.json")?;

    let mut rl = rustyline::Editor::new()
        .history_ignore_dups(true)
        .history_ignore_space(true);
    rl.set_completer(Some(&data));
    loop {
        let readline = rl.readline(">> ");
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
            "quit" => break,
            m if m.starts_with("monster ") => {
                let monster = m.trim_left_matches("monster ");
                let monster = data.monsters.find(monster);
                match monster {
                    Some(monster) => println!("{}", monster),
                    None => println!("No match"),
                }
            }
            m if m.starts_with("move ") => {
                let mv = m.trim_left_matches("move ");
                let mv = data.moves.find(mv);
                match mv {
                    Some(mv) => println!("{}", mv),
                    None => println!("No match"),
                }
            }
            _ => {}
        }
    }

    Ok(())
}
