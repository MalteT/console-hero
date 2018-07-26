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

/// Wrapper around a `Vec<Tag>`.
///
/// For implementing some functions and traits.
pub struct Tags {
    data: Vec<Tag>,
}

/// Tag data.
/// ```text
/// TODO
/// ```
#[derive(Serialize, Deserialize, Debug)]
pub struct Tag {
    /// Name of the tag.
    name: String,
    /// Unique identifier for this tag.
    key: String,
    /// A description about this tag.
    description: String,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
enum MatchType {
    None,
    Description,
    Name,
}

impl Tags {
    /// Parse the data given through the reader into `Tag`.
    pub fn parse<R: Read>(reader: R) -> io::Result<Self> {
        serde_json::from_reader(reader)
            .map(|data| Tags { data })
            .map_err(|e| Error::new(InvalidData, e))
    }
    /// Find a tag that matches the given String `re`.
    /// Matches the given fields in the given order:
    /// - `name`
    /// - `description`
    /// The first tag whose name matches is returned,
    /// otherwise the first whose description matches.
    pub fn find(&self, re: &str) -> Option<&Tag> {
        let mut best = (MatchType::None, None);
        let re = Regex::new(&format!("(?i){}", re)).unwrap();
        for tag in &self.data {
            if re.is_match(&tag.name) {
                best = (MatchType::Name, Some(tag));
                break;
            } else if best.0 < MatchType::Description && re.is_match(&tag.description) {
                best = (MatchType::Description, Some(tag));
            }
        }
        best.1
    }
}

impl fmt::Display for Tag {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        let width = 60;
        // Name
        let name = capitalize(&self.name)
            .pad_to_width(width - 2)
            .bold()
            .yellow();
        // Description
        let desc = wrap(&self.description, width - 2, " ┃ ", " ┃");
        write!(
            f,
            "
 ┏{0}┓
 ┃ {1} ┃
 ┣{0}┫
{2}
 ┗{0}┛
",
            bold_line(width),
            name,
            desc
        )
    }
}

impl Deref for Tags {
    type Target = Vec<Tag>;
    fn deref(&self) -> &Self::Target {
        &self.data
    }
}

impl DerefMut for Tags {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.data
    }
}

impl Completer for Tags {
    fn complete(&self, line: &str, pos: usize) -> rustyline::Result<(usize, Vec<String>)> {
        if line.starts_with("tag ") && pos >= 4 {
            let part_tag = line.trim_left_matches("tag ").to_lowercase();
            let mut ret = Vec::new();
            for tag in &self.data {
                if tag.name.to_lowercase().starts_with(&part_tag) {
                    ret.push(tag.name.clone());
                }
            }
            Ok((4, ret))
        } else {
            Ok((pos, vec![]))
        }
    }
}
