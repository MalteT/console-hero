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

use rustyline::completion::Completer;
use std::fs::File;
use std::io;

use data::Moves;
use data::Monsters;

fn main() -> io::Result<()> {
    let f = File::open("data/moves.json")?;
    let moves = Moves::parse(f)?;
    let f = File::open("data/monsters.json")?;
    let monsters = Monsters::parse(f)?;

    let data = Data::new(monsters, moves);

    let mut rl = rustyline::Editor::new()
        .history_ignore_dups(true)
        .history_ignore_space(true);
    rl.set_completer(Some(&data));
    loop {
        let readline = rl.readline(">> ").unwrap_or(String::from(""));
        rl.add_history_entry(&readline);
        match readline.as_str() {
            "quit" => break,
            m if m.starts_with("monster ") => {
                let monster = m.trim_left_matches("monster ");
                let monster = data.monsters.find(monster);
                match monster {
                    Some(monster) => println!("{}", monster),
                    None => println!("No match"),
                }
            },
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

struct Data {
    monsters: Monsters,
    moves: Moves,
}

impl Data {
    fn new(monsters: Monsters, moves: Moves) -> Self {
        Data {monsters, moves}
    }
}

impl Completer for Data {
    fn complete(&self, line: &str, pos: usize) -> rustyline::Result<(usize, Vec<String>)> {
        if line.starts_with("move ") {
            self.moves.complete(line, pos)
        } else if line.starts_with("monster ") {
            self.monsters.complete(line, pos)
        } else {
            Ok((pos, vec![]))
        }
    }
}
