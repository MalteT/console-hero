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
use std::io::{Error, ErrorKind::InvalidData};
use std::ops::{Deref, DerefMut};

/// Wrapper around a `Vec<Monster>`.
///
/// For implementing some functions and traits.
#[derive(Serialize, Deserialize, Debug)]
pub struct Monsters {
    data: Vec<Monster>,
}

/// Monster data.
///
/// ```text
/// ┏━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━┓
/// ┃ Apocalypse Dragon                         26 HP   5 Armor  ┃
/// ┣━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━┫
/// ┃ Bite (b[2d12]+9)        Reach, Forceful, Messy, 4 piercing ┃
/// ┣━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━┫
/// ┃                            Solitary, Huge, Magical, Divine ┃
/// ┠────────────────────────────────────────────────────────────┨
/// ┃ The end of all things shall be a burning—of tree and earth ┃
/// ┃ and of the air itself. It shall come upon the plains and   ┃
/// ┃ mountains not from beyond this world but from within it.   ┃
/// ┃ Birthed from the womb of deepest earth shall come the      ┃
/// ┃ Dragon that Will End the World. In its passing all will    ┃
/// ┃ become ash and bile and the Dungeon World a dying thing    ┃
/// ┃ will drift through planar space devoid of life. They say   ┃
/// ┃ to worship the Apocalypse Dragon is to invite madness.     ┃
/// ┃ They say to love it is to know oblivion. The awakening is  ┃
/// ┃ coming.                                                    ┃
/// ┠────────────────────────────────────────────────────────────┨
/// ┃ Instinct: To end the world!                                ┃
/// ┠────────────────────────────────────────────────────────────┨
/// ┃ • Set a disaster in motion.                                ┃
/// ┃ • Breathe forth the elements.                              ┃
/// ┃ • Act with perfect foresight.                              ┃
/// ┗━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━┛
/// ```
#[derive(Serialize, Deserialize, Debug)]
pub struct Monster {
    /// Unique identifier for the monster.
    key: String,
    /// Name of the monster.
    /// TODO: Fix data!
    #[serde(default)]
    name: String,
    /// List of tags, this monster has.
    #[serde(default)]
    tags: Vec<String>,
    /// Amount of armor this monster has.
    #[serde(default)]
    armor: u8,
    /// Amount of HP this monster has.
    #[serde(default)]
    hp: u8,
    /// Basic instinct of this monster.
    #[serde(default)]
    instinct: String,
    /// List of moves which are common to this monster.
    #[serde(default)]
    moves: Vec<String>,
    /// A description about this monster.
    #[serde(default)]
    description: String,
    /// List of attacks this monster can make.
    #[serde(default)]
    attacks: Vec<Attack>,
}

/// Attack struct containing:
/// - name
/// - damage
/// - tags
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
    /// List all monsters whose name match the given `regex`.
    pub fn list(&self, re: &str) {
        println!(">> {}", "Monsters".bold());
        let re = Regex::new(&format!("(?i){}", re)).unwrap();
        self.data
            .iter()
            .filter(|monster| re.is_match(&monster.name))
            .map(|monster| println!("   {}", monster.name))
            .collect::<Vec<()>>();
    }
}

impl fmt::Display for Monster {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        let width = 60;
        // Name
        let name = self.name.bold().yellow();
        let name_hp_armor = format!(
            "{} {{}} {} {}",
            name,
            format!(" {} HP ", self.hp).on_red(),
            format!(" {} Armor ", self.armor).on_blue(),
        );
        // Has description
        let has_description = self.description != String::new();
        // Attacks
        // TODO Handle multiple attacks
        assert!(self.attacks.len() <= 1);
        let attack = self.attacks
            .first()
            .map(|a| format!("{}", a))
            .unwrap_or(String::new());
        // Tags
        let tags = self.tags.iter().map(|tag| capitalize(tag));
        let tags = format!(" {{}}{}", concat(tags, ", "));
        // Has Tags
        let has_tags = tags != String::from(" {}");
        // Instinct
        let instinct = " Instinct ".on_bright_white().black();
        let instinct = format!("{} {}!", instinct, self.instinct);
        // Has instinct
        let has_instinct = self.instinct != String::new();
        // Moves
        let moves = self.moves
            .iter()
            .map(|s| {
                let mut s = s.clone();
                s += ".";
                s
            })
            .collect();
        // Create the card and print it
        write!(
            f,
            "{}",
            Card::new()
                .with_heavy_border()
                .with_width(width)
                .line(&name_hp_armor)
                .heavy_line()
                .line_if(&attack, !self.attacks.is_empty())
                .light_line_if(!self.attacks.is_empty())
                .line_if(&tags, has_tags)
                .light_line_if(has_tags)
                .text_if(&self.description, has_description)
                .light_line_if(has_description)
                .line_if(&instinct, has_instinct)
                .light_line_if(has_instinct)
                .list(moves)
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
