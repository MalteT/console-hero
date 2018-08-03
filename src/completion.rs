use data::Data;
use rustyline;
use rustyline::completion::Completer;

pub struct HeroCompleter<'a> {
    data: &'a Data,
}

macro_rules! comp {
    ($nr:expr) => {
        Ok(($nr, vec![]))
    };
    ($nr:expr, $( $str:expr ),*) => {
        Ok(($nr, vec![ $( $str.to_string() ),* ]))
    };
    ($nr:expr; $vec:expr) => {
        Ok(($nr, $vec))
    }
}

impl<'a> HeroCompleter<'a> {
    pub fn new(data: &'a Data) -> Self {
        HeroCompleter { data }
    }
}

impl<'a> Completer for HeroCompleter<'a> {
    fn complete(&self, line: &str, pos: usize) -> rustyline::Result<(usize, Vec<String>)> {
        let top_level = [
            "help", "info", "quit", "item", "monster", "move", "tag", "list",
        ];
        let matches: Vec<String> = top_level
            .iter()
            .filter(|com| com.starts_with(line))
            .map(|com| com.to_string())
            .collect();
        if !matches.is_empty() {
            comp!(0; matches)
        } else if line.starts_with("item ") {
            self.data.items.complete(line, pos)
        } else if line.starts_with("monster ") {
            self.data.monsters.complete(line, pos)
        } else if line.starts_with("move ") {
            self.data.moves.complete(line, pos)
        } else if line.starts_with("tag ") {
            self.data.tags.complete(line, pos)
        } else if line.starts_with("list ") {
            let sec_level = ["monsters", "moves", "items", "tags"];
            let line = line.trim_left_matches("list ");
            let matches = sec_level
                .iter()
                .filter(|com| com.starts_with(line))
                .map(|com| com.to_string())
                .collect();
            comp!(5; matches)
        } else {
            comp!(0)
        }
    }
}
