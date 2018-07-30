use super::card::helper::*;
use super::card::Card;
use colored::*;
use regex::Regex;
use rustyline;
use rustyline::completion::Completer;
use serde_json;
use std::collections::BTreeMap;
use std::fmt;
use std::io;
use std::io::Read;
use std::io::{Error, ErrorKind::InvalidData};
use std::ops::{Deref, DerefMut};

/// Wrapper around a `Vec<Item>`.
///
/// For implementing some functions and traits.
pub struct Items {
    data: Vec<Item>,
}

/// Item data.
#[derive(Serialize, Deserialize, Debug)]
pub struct Item {
    // Name of the item
    name: String,
    // Plural form of the name, if not default
    #[serde(default)]
    plural_name: String,
    // Unique identifier of the item
    key: String,
    // Description of the item
    #[serde(default)]
    description: String,
    // Valued tags of the item
    // #[serde(skip)]
    tags: Vec<ItemTag>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(untagged)]
enum ItemTag {
    MapI(BTreeMap<String, u16>),
    MapS(BTreeMap<String, String>),
    Tag(String),
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
enum MatchType {
    None,
    Description,
    Name,
}

impl Items {
    /// Parse the data given through the reader into `Items`.
    pub fn parse<R: Read>(reader: R) -> io::Result<Self> {
        serde_json::from_reader(reader)
            .map(|data| Items { data })
            .map_err(|e| Error::new(InvalidData, e))
    }
    /// Find an item that matches the given String `regex`.
    /// Matches the given fields in the given order:
    /// - `name`
    /// - `description`
    /// The first item whose name matches is returned,
    /// otherwise the first whose description matches, ...
    pub fn find(&self, re: &str) -> Option<&Item> {
        let mut best = (MatchType::None, None);
        let re = Regex::new(&format!("(?i){}", re)).unwrap();
        for item in &self.data {
            if re.is_match(&item.name) {
                best = (MatchType::Name, Some(item));
                break;
            } else if best.0 < MatchType::Description && re.is_match(&item.description) {
                best = (MatchType::Description, Some(item));
            }
        }
        best.1
    }
}

impl fmt::Display for Item {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        let width = 40;
        // Name
        let name = format!("{}", self.name.bold().yellow());
        // Tags
        let tags = self.tags.iter().map(|tag| format!("{}", tag));
        let tags = concat(tags, ", ");
        // Create the card and show it
        write!(
            f,
            "{}",
            Card::new()
                .with_width(width)
                .with_heavy_border()
                .line(&name)
                .heavy_line()
                .line(&tags)
                .light_line()
                .text(&self.description)
        )
    }
}

impl fmt::Display for ItemTag {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        let (key, value) = match self {
            ItemTag::MapI(map) => {
                if let Some((k, v)) = map.iter().next() {
                    (k, format!("{}", v))
                } else {
                    return write!(f, "BUG WITH TAGS");
                }
            }
            ItemTag::MapS(map) => {
                if let Some((k, v)) = map.iter().next() {
                    (k, format!("{}", v))
                } else {
                    return write!(f, "BUG WITH TAGS");
                }
            }
            ItemTag::Tag(s) => return write!(f, "{}", capitalize(s)),
        };
        match key.as_str() {
            "weight" => write!(f, "{} KG", value),
            x => write!(f, "{} {}", value, capitalize(x)),
        }
    }
}

impl Deref for Items {
    type Target = Vec<Item>;
    fn deref(&self) -> &Self::Target {
        &self.data
    }
}

impl DerefMut for Items {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.data
    }
}

impl Completer for Items {
    fn complete(&self, line: &str, pos: usize) -> rustyline::Result<(usize, Vec<String>)> {
        if line.starts_with("item ") && pos >= 5 {
            let part_item = line.trim_left_matches("item ").to_lowercase();
            let mut ret = Vec::new();
            for item in &self.data {
                if item.name.to_lowercase().starts_with(&part_item) {
                    ret.push(item.name.clone());
                }
            }
            Ok((5, ret))
        } else {
            Ok((pos, vec![]))
        }
    }
}
