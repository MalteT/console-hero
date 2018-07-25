use colored::*;
use regex::Regex;
use rustyline;
use rustyline::completion::Completer;
use serde_json;
use std::fmt;
use std::io;
use std::io::Read;
use std::io::{Error, ErrorKind::InvalidData};
use std::ops::{Deref, DerefMut};
use textwrap::wrap_iter;

#[derive(Serialize, Deserialize, Debug)]
pub struct Moves {
    data: Vec<Move>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Move {
    name: String,
    // TODO: Fix the data, to remove the default tag!
    #[serde(default)]
    key: String,
    description: String,
    #[serde(default = "all_string")]
    classes: Vec<String>,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
enum MatchType {
    None,
    Classes,
    Description,
    Name,
}

impl Moves {
    /// Parse the data given through the reader into `Moves`.
    pub fn parse<R: Read>(reader: R) -> io::Result<Self> {
        serde_json::from_reader(reader)
            .map(|data| Moves { data })
            .map_err(|e| Error::new(InvalidData, e))
    }
    /// Find a move that matches the given String `regex`.
    /// Matches the given fields in the given order:
    /// - `name`
    /// - `description`
    /// - `classes`
    /// The first move whose name matches is returned,
    /// otherwise the first whose description matches, ...
    pub fn find(&self, re: &str) -> Option<&Move> {
        let mut best = (MatchType::None, None);
        let re = Regex::new(&format!("(?i){}", re)).unwrap();
        for mv in &self.data {
            if re.is_match(&mv.name) {
                best = (MatchType::Name, Some(mv));
                break;
            } else if best.0 < MatchType::Description && re.is_match(&mv.description) {
                best = (MatchType::Description, Some(mv));
            } else if best.0 < MatchType::Classes {
                for class in &mv.classes {
                    if re.is_match(&class) {
                        best = (MatchType::Classes, Some(mv));
                    }
                }
            }
        }
        best.1
    }
}

impl Deref for Moves {
    type Target = Vec<Move>;
    fn deref(&self) -> &Self::Target {
        &self.data
    }
}

impl DerefMut for Moves {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.data
    }
}

impl Completer for Moves {
    fn complete(&self, line: &str, pos: usize) -> rustyline::Result<(usize, Vec<String>)> {
        if line.starts_with("move ") && pos >= 5 {
            let part_mv = line.trim_left_matches("move ").to_lowercase();
            let mut ret = Vec::new();
            for mv in &self.data {
                if mv.name.to_lowercase().starts_with(&part_mv) {
                    ret.push(mv.name.clone());
                }
            }
            Ok((5, ret))
        } else {
            Ok((pos, vec![]))
        }
    }
}

impl fmt::Display for Move {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        let classes: String = self.classes.iter().fold(String::new(), |mut s, class| {
            if s.len() > 0 {
                s += ", ";
            }
            s += &class;
            s
        });
        let desc = wrap_iter(&self.description, 38)
            .map(|s| format!(" ┃ {:<38} ┃\n", s))
            .fold(String::new(), |mut s, desc| {
                s += &desc;
                s
            });
        write!(f, "
 ┏━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━┓
 ┃ {:<38} ┃
 ┣━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━┫
 ┃ {} {:<31} ┃
 ┠────────────────────────────────────────┨
 {}
 ┗━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━┛
", self.name.bold().yellow(), "Class:", classes, desc.trim())
    }
}

fn all_string() -> Vec<String> {
    vec![String::from("all")]
}
