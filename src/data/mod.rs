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

/// Data wrapper.
/// This wrapper contains the following data:
/// - monsters. See [Monsters](self::monsters::Monsters)
/// - moves. See [Moves](self::moves::Moves)
pub struct Data {
    pub monsters: Monsters,
    pub moves: Moves,
}

impl Data {
    /// Create a new Data object wrapping `monsters`' and `moves`' data.
    pub fn new(monsters: Monsters, moves: Moves) -> Self {
        Data { monsters, moves }
    }
    /// Create a new Data object by parsing the files given by their paths:
    /// - `monster_file` for monsters data
    /// - `moves_file` for moves data
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
