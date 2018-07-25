extern crate regex;
extern crate rustyline;
extern crate serde;
extern crate serde_json;
extern crate textwrap;
#[macro_use]
extern crate serde_derive;
extern crate colored;
extern crate pad;

mod data;

use std::fs::File;
use std::io;

use data::Moves;

fn main() -> io::Result<()> {
    let f = File::open("data/moves.json")?;
    let moves = Moves::parse(f)?;

    let mut rl = rustyline::Editor::new()
        .history_ignore_dups(true)
        .history_ignore_space(true);
    rl.set_completer(Some(&moves));
    loop {
        let readline = rl.readline(">> ").unwrap_or(String::from(""));
        rl.add_history_entry(&readline);
        match readline.as_str() {
            "quit" => break,
            m if m.starts_with("move ") => {
                let mv = m.trim_left_matches("move ");
                let mv = moves.find(mv);
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
