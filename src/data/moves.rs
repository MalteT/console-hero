use super::helper::*;
use colored::*;
use pad::PadStr;
use regex::Regex;
use rustyline;
use rustyline::completion::Completer;
use serde_json;
use std::fmt;
use std::io;
use std::io::Read;
use std::io::{Error, ErrorKind::InvalidData};
use std::ops::{Deref, DerefMut};

/// Wrapper around a `Vec<Moves>`.
///
/// For implementing some functions and traits.
#[derive(Serialize, Deserialize, Debug)]
pub struct Moves {
    data: Vec<Move>,
}

/// Data about a move a character can execute.
///
/// ```text
/// ┏━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━┓
/// ┃ Dirty Fighter                          ┃
/// ┣━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━┫
/// ┃ Class: thief                           ┃
/// ┠────────────────────────────────────────┨
/// ┃ When using a precise or hand weapon,   ┃
/// ┃ your backstab deals an extra +1d8      ┃
/// ┃ damage and all other attacks deal +1d4 ┃
/// ┃ damage.                                ┃
/// ┗━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━┛
/// ```
#[derive(Serialize, Deserialize, Debug)]
pub struct Move {
    /// Name of the move.
    name: String,
    /// Unique identifier of the move.
    /// TODO: Fix the data, to remove the default tag!
    #[serde(default)]
    key: String,
    /// Simple description containing dice rolls.
    description: String,
    /// List of classes who might have this move.
    #[serde(default = "all_string")]
    classes: Vec<String>,
    /// A short explanation of this move.
    #[serde(default)]
    explanation: String,
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
        let width = 40;
        // Name
        let name = format!("{}", self.name.bold().yellow());
        // Description
        let desc = wrap(&self.description, width - 2, " ┃ ", " ┃");
        // Classes
        let classes = self.classes
            .iter()
            .map(|s| capitalize(s))
            .map(|s| format!(" {} ", s).on_bright_white().black())
            .map(|s| format!("{}", s));
        let count_classes = classes.len();
        let classes = concat(classes, ", ").pad_to_width(width + count_classes * 9);
        // Explanation
        let exp = if self.explanation == String::new() {
            format!("")
        } else {
            let exp = wrap(&self.explanation, width - 2, " ┃ ", " ┃");
            format!("\n ┠{}┨\n{}", thin_line(width), exp)
        };
        write!(
            f,
            "
 ┏{0}┓
 ┃ {2} ┃
 ┣{0}┫
 ┃ {3} ┃
 ┠{1}┨
 {4}{5}
 ┗{0}┛
",
            bold_line(width),
            thin_line(width),
            name.pad_to_width(width + 7),
            classes,
            desc.trim(),
            exp
        )
    }
}

/// Simple helper function for Serde to return the String `all`.
fn all_string() -> Vec<String> {
    vec![String::from("all")]
}
