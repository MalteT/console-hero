mod helper;
mod monsters;
mod moves;
mod tags;

pub use self::monsters::Monster;
pub use self::monsters::Monsters;
pub use self::moves::Move;
pub use self::moves::Moves;
pub use self::tags::Tag;
pub use self::tags::Tags;

use rustyline;
use rustyline::completion::Completer;
use std::fs::File;
use std::io;

/// Data wrapper.
/// This wrapper contains the following data:
/// - monsters. See [Monsters](self::monsters::Monsters)
/// - moves. See [Moves](self::moves::Moves)
/// - tags. See [Tags](self::tags::Tags)
pub struct Data {
    pub monsters: Monsters,
    pub moves: Moves,
    pub tags: Tags,
}

impl Data {
    /// Create a new Data object wrapping `monsters`' and `moves`' data.
    pub fn new(monsters: Monsters, moves: Moves, tags: Tags) -> Self {
        Data {
            monsters,
            moves,
            tags,
        }
    }
    /// Create a new Data object by parsing the files given by their paths:
    /// - `monster_file` for monsters data
    /// - `moves_file` for moves data
    pub fn from(monster_file: &str, moves_file: &str, tags_file: &str) -> io::Result<Self> {
        let f = File::open(monster_file)?;
        let moves = Moves::parse(f)?;
        let f = File::open(moves_file)?;
        let monsters = Monsters::parse(f)?;
        let f = File::open(tags_file)?;
        let tags = Tags::parse(f)?;

        Ok(Data::new(monsters, moves, tags))
    }
}

impl Completer for Data {
    fn complete(&self, line: &str, pos: usize) -> rustyline::Result<(usize, Vec<String>)> {
        if line.starts_with("move ") {
            self.moves.complete(line, pos)
        } else if line.starts_with("monster ") {
            self.monsters.complete(line, pos)
        } else if line.starts_with("tag ") {
            self.tags.complete(line, pos)
        } else {
            Ok((pos, vec![]))
        }
    }
}
