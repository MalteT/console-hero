//! Dungeon World Moves
//!
//! ```text
//!  ┏━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━┓
//!  ┃ Anointed                       Cleric  ┃
//!  ┣━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━┫
//!  ┃  Requires  Chosen One                  ┃
//!  ┠━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━┨
//!  ┃ Choose one spell in addition to the    ┃
//!  ┃ one you picked for chosen one. You are ┃
//!  ┃ granted that spell as if it was one    ┃
//!  ┃ level lower.                           ┃
//!  ┗━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━┛
//! ```

use super::card::helper::*;
use super::card::Card;
use colored::*;
use regex::Regex;
use rustyline;
use rustyline::completion::Completer;
use serde_json;
use std::fmt;
use std::io;
use std::io::Read;
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
///  ┏━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━┓
///  ┃ Anointed                       Cleric  ┃
///  ┣━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━┫
///  ┃  Requires  Chosen One                  ┃
///  ┠━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━┨
///  ┃ Choose one spell in addition to the    ┃
///  ┃ one you picked for chosen one. You are ┃
///  ┃ granted that spell as if it was one    ┃
///  ┃ level lower.                           ┃
///  ┗━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━┛
/// ```
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
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
    /// Replaces this move
    #[serde(default)]
    replaces: String,
    /// The following move is required for this
    #[serde(default)]
    requires: String,
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
        let mut data: Vec<Move> = serde_json::from_reader(reader)?;
        let lookup = Moves { data: data.clone() };
        data.iter_mut().for_each(|ref mut mv| {
            if mv.replaces != String::new() {
                let repl = lookup
                    .get(&mv.replaces)
                    .expect(&format!("Key {} does not exist!", mv.replaces))
                    .name
                    .clone();
                mv.replaces = repl;
            }
            if mv.requires != String::new() {
                let repl = lookup
                    .get(&mv.requires)
                    .expect(&format!("Key {} does not exist!", mv.requires))
                    .name
                    .clone();
                mv.requires = repl;
            }
        });

        Ok(Moves { data })
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
    /// Find the move with the given `key`.
    /// If none is found, [None](Option::None) is returned.
    pub fn get(&self, key: &str) -> Option<&Move> {
        self.data.iter().find(|mv| mv.key == key)
    }
    /// List all moves whose name match the given `regex`.
    pub fn list(&self, re: &str) {
        println!(">> {}", "Moves".bold());
        let re = Regex::new(&format!("(?i){}", re)).unwrap();
        self.data
            .iter()
            .filter(|mv| re.is_match(&mv.name))
            .map(|mv| println!("   {}", mv.name))
            .collect::<Vec<()>>();
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
        // Classes
        let classes = self.classes
            .iter()
            .map(|s| capitalize(s))
            .map(|s| format!(" {} ", s).on_bright_white().black())
            .map(|s| format!("{}", s));
        let classes = concat(classes, ", ");
        // Combine name and classes
        let name_classes = format!("{}{{}}{}", name, classes);
        // Requires tag
        let has_requires = self.requires != String::new();
        let req = format!("{} {}", " Requires ".on_red().black(), self.requires);
        // Has replaces tag
        let has_replaces = self.replaces != String::new();
        let rep = format!(
            "{} {}",
            " Replaces ".on_bright_white().black(),
            self.replaces
        );
        // Has an explanation
        let has_explanation = self.explanation != String::new();
        // Create the card and write it
        write!(
            f,
            "{}",
            Card::new()
                .with_width(width)
                .with_heavy_border()
                .line(&name_classes)
                .heavy_line()
                .line_if(&req, has_requires)
                .line_if(&rep, has_replaces)
                .light_line_if(has_requires || has_replaces)
                .text(&self.description)
                .light_line_if(has_explanation)
                .text_if(&self.explanation, has_explanation)
        )
    }
}

/// Simple helper function for Serde to return the String `all`.
fn all_string() -> Vec<String> {
    vec![String::from("all")]
}
