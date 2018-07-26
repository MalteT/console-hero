mod helper;
mod monsters;
mod moves;

pub use self::monsters::Monster;
pub use self::monsters::Monsters;
pub use self::moves::Move;
pub use self::moves::Moves;

use rustyline;
use rustyline::completion::Completer;
use std::fs::File;
use std::io;

pub struct Data {
    pub monsters: Monsters,
    pub moves: Moves,
}

impl Data {
    pub fn new(monsters: Monsters, moves: Moves) -> Self {
        Data { monsters, moves }
    }
    pub fn from(monster_file: &str, moves_file: &str) -> io::Result<Self> {
        let f = File::open(monster_file)?;
        let moves = Moves::parse(f)?;
        let f = File::open(moves_file)?;
        let monsters = Monsters::parse(f)?;

        Ok(Data::new(monsters, moves))
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
