use super::*;
use colored::*;
use pad::{Alignment, PadStr};
use regex::Regex;
use rustyline;
use rustyline::completion::Completer;
use serde_json;
use std::fmt;
use std::io;
use std::io::Read;
use std::io::{Error, ErrorKind::InvalidData};
use std::ops::{Deref, DerefMut};

#[derive(Serialize, Deserialize, Debug)]
pub struct Monsters {
    data: Vec<Monster>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Monster {
    key: String,
    #[serde(default)]
    name: String,
    #[serde(default)]
    tags: Vec<String>,
    #[serde(default)]
    armor: u8,
    #[serde(default)]
    hp: u8,
    #[serde(default)]
    instinct: String,
    #[serde(default)]
    moves: Vec<String>,
    #[serde(default)]
    description: String,
    #[serde(default)]
    attacks: Vec<Attack>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Attack {
    name: String,
    damage: String,
    tags: Vec<String>,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
enum MatchType {
    None,
    Description,
    Name,
}

impl Monsters {
    /// Parse the data given through the reader into `Monsters`.
    pub fn parse<R: Read>(reader: R) -> io::Result<Self> {
        serde_json::from_reader(reader)
            .map(|data| Monsters { data })
            .map_err(|e| Error::new(InvalidData, e))
    }
    /// Find a move that matches the given String `regex`.
    /// Matches the given fields in the given order:
    /// - `name`
    /// - `description`
    /// The first move whose name matches is returned,
    /// otherwise the first whose description matches, ...
    pub fn find(&self, re: &str) -> Option<&Monster> {
        let mut best = (MatchType::None, None);
        let re = Regex::new(&format!("(?i){}", re)).unwrap();
        for monster in &self.data {
            if re.is_match(&monster.name) {
                best = (MatchType::Name, Some(monster));
                break;
            } else if best.0 < MatchType::Description && re.is_match(&monster.description) {
                best = (MatchType::Description, Some(monster));
            }
        }
        best.1
    }
}

impl fmt::Display for Monster {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        let width = 60;
        // Name
        let name = self.name.bold();
        let desc = wrap(&self.description, width - 2, " ┃ ", " ┃");
        let hp = format!(" {} HP ", self.hp).on_red();
        let armor = format!(" {} Armor ", self.armor).on_blue();
        let hp_armor = format!("{} {}", hp, armor);
        // Name and HP and Armor
        let nha = format!("{} {{}} {}", name, hp_armor);
        let nha = expand(&nha, width + 18);
        // Tags
        let tags = self.tags.iter().map(|tag| capitalize(tag));
        let tags = concat(tags, ", ").pad_to_width_with_alignment(width - 2, Alignment::Right);
        // Instinct
        let instinct_str = "Instinct:".on_bright_white().black();
        let instinct = format!("{} {}!", instinct_str, self.instinct).pad_to_width(width + 11 - 2);
        // Moves
        let moves = self.moves.iter().map(|s| {
            let mut s = s.clone();
            s += ".";
            s
        });
        // Attacks
        let attacks = self.attacks
            .iter()
            .map(|attack| format!("{}", attack))
            .map(|attack| expand(&attack, width - 2));
        let mut attacks = concat(attacks, ", ");
        if attacks == "" {
            attacks = "no attacks".pad_to_width(width - 2);
        }
        write!(
            f,
            "
 ┏{0}┓
 ┃ {2} ┃
 ┣{0}┫
 ┃ {7} ┃
 ┣{0}┫
 ┃ {3} ┃
 ┠{1}┨
{4}
 ┠{1}┨
 ┃ {5} ┃
 ┠{1}┨
{6}
 ┗{0}┛
",
            bold_line(width),
            thin_line(width),
            nha,
            tags,
            desc,
            instinct,
            listify(moves, '•', width - 2, " ┃ ", " ┃"),
            attacks
        )
    }
}

impl Deref for Monsters {
    type Target = Vec<Monster>;
    fn deref(&self) -> &Self::Target {
        &self.data
    }
}

impl DerefMut for Monsters {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.data
    }
}

impl fmt::Display for Attack {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        let tags = concat(self.tags.iter().map(|tag| capitalize(tag)), ", ");
        let s = format!("{} ({}) {{}} {}", self.name, self.damage, tags);
        write!(f, "{}", s)
    }
}

impl Completer for Monsters {
    fn complete(&self, line: &str, pos: usize) -> rustyline::Result<(usize, Vec<String>)> {
        if line.starts_with("monster ") && pos >= 8 {
            let part_monster = line.trim_left_matches("monster ").to_lowercase();
            let mut ret = Vec::new();
            for monster in &self.data {
                if monster.name.to_lowercase().starts_with(&part_monster) {
                    ret.push(monster.name.clone());
                }
            }
            Ok((8, ret))
        } else {
            Ok((pos, vec![]))
        }
    }
}
