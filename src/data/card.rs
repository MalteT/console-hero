//! ```text
//!  ┏━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━┓
//!  ┃ +bonus                                 ┃
//!  ┣━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━┫
//!  ┃ It modifies your effectiveness in a    ┃
//!  ┃ specified situation. It might be “+1   ┃
//!  ┃ forward to spout lore” or “-1 ongoing  ┃
//!  ┃ to hack and slash.”                    ┃
//!  ┗━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━┛
//! ```
use pad::PadStr;
use regex::Regex;
use std::fmt;
use textwrap::wrap_iter;

/// A terminal card.
/// Builder for card like terminal output used for the monster, moves, etc cards.
#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Card {
    elements: Vec<Element>,
    border: Border,
    width: usize,
}

/// An element of the card.
#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd)]
enum Element {
    /// A light line.
    /// `───────────`
    LightLine,
    /// A heavy line.
    /// `━━━━━━━━━━━`
    HeavyLine,
    /// A text which will be word-wrapped to the appropriate width.
    Text(String),
    /// A line of text which can contain `{}` to specify the point of expanses.
    Line(String),
    /// A list of items to be displayed as a list.
    List(Vec<String>),
}

/// Border types to be used.
#[derive(Debug, Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
pub enum Border {
    /// The light border.
    /// ```text
    /// ┌───┐
    /// │   │
    /// └───┘
    /// ```
    Light,
    /// The heavy border.
    /// ```text
    /// ┏━━━┓
    /// ┃   ┃
    /// ┗━━━┛
    /// ```
    Heavy,
}

impl Card {
    /// Create a new Card.
    /// Uses the [heavy](Border::Heavy) border and a width of `40`.
    pub fn new() -> Self {
        Card {
            elements: Vec::new(),
            border: Border::Heavy,
            width: 40,
        }
    }
    /// Specify the `width` of the card.
    pub fn with_width(mut self, width: usize) -> Self {
        self.width = width;
        self
    }
    /// Use the light border for the card.
    pub fn with_light_border(mut self) -> Self {
        self.border = Border::Light;
        self
    }
    /// Use the heavy border for the card.
    pub fn with_heavy_border(mut self) -> Self {
        self.border = Border::Heavy;
        self
    }
    /// Add a light line.
    pub fn light_line(mut self) -> Self {
        self.elements.push(Element::LightLine);
        self
    }
    /// Add a heavy line.
    pub fn heavy_line(mut self) -> Self {
        self.elements.push(Element::HeavyLine);
        self
    }
    /// Add a text.
    pub fn text(mut self, s: &str) -> Self {
        self.elements.push(Element::Text(String::from(s.trim())));
        self
    }
    /// Add a single line, with spacing at `{}`.
    pub fn line(mut self, s: &str) -> Self {
        self.elements.push(Element::Line(String::from(s)));
        self
    }
    /// Add a list of items.
    pub fn list(mut self, list: Vec<String>) -> Self {
        self.elements.push(Element::List(list));
        self
    }
}

/// Calculate the width of a string containing escape codes for coloring.
pub fn terminal_string_width(s: &str) -> usize {
    let re = Regex::new(r"\x1B\[.*?m").unwrap();
    re.replace_all(s, "").len()
}

/// Wraps the given String by word wrapping at the given
/// `width` and adds the given `border` left and right to each line,
/// returning concatinated lines with `\n`s.
pub fn wrap(text: &str, width: usize, border: &str) -> String {
    wrap_iter(text, width)
        .map(|s| s.pad_to_width(width))
        .map(|s| format!("{0}{1}{0}\n", border, s))
        .fold(String::new(), |mut s, desc| {
            s += &desc;
            s
        })
        .trim_right_matches("\n")
        .to_string()
}

/// Make a list out of an Iterator over Strings, using the `border` left and right.
pub fn listify<'a, I>(items: I, bullet: char, width: usize, border: &str) -> String
where
    I: Iterator<Item = String>,
{
    items
        .map(|item| {
            let item = item.pad_to_width(width - 2);
            format!("{0}{1} {2}{0}\n", border, bullet, item)
        })
        .fold(String::new(), |mut s, item| {
            s += &item;
            s
        })
        .trim_right_matches("\n")
        .to_string()
}

impl fmt::Display for Card {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        let border = match self.border {
            Border::Heavy => " ┃ ",
            Border::Light => " ┃ ",
        };
        let line = match self.border {
            Border::Heavy => "━".repeat(self.width),
            Border::Light => "─".repeat(self.width),
        };
        let mut first = vec![match self.border {
            Border::Heavy => format!(" ┏{}┓", line),
            Border::Light => format!(" ┌{}┐", line),
        }];
        let els = self.elements.iter().map(|ref el| match el {
            Element::LightLine => format!(" ┠{}┨", line),
            Element::HeavyLine => format!(" ┣{}┫", line),
            Element::Text(s) => format!("{}", wrap(s, self.width - 2, border)),
            Element::Line(l) => format!("{0}{1}{0}", border, expand(l, self.width - 2)),
            Element::List(v) => format!(
                "{}",
                listify(v.iter().map(|v| v.clone()), '•', self.width - 2, border,)
            ),
        });
        let last = vec![match self.border {
            Border::Heavy => format!(" ┗{}┛\n", line),
            Border::Light => format!(" └{}┘\n", line),
        }];
        first.extend(els);
        first.extend(last);
        write!(
            f,
            "{}",
            first
                .iter()
                .fold(String::new(), |s, el| format!("{}\n{}", s, el))
        )
    }
}

/// Expands the given string `text` at `{}` to match the given `width`.
/// If the `text` is already wider than `width` do nothing
pub fn expand(text: &str, width: usize) -> String {
    if text.contains("{}") {
        let w = terminal_string_width(text);
        if w > width {
            text.replacen("{}", "", 1).to_string()
        } else {
            let parts: Vec<&str> = text.split("{}").collect();
            let left = parts[0];
            let right = parts[1];
            let lw = terminal_string_width(left);
            let rw = terminal_string_width(right);
            let missing = width - lw - rw;
            format!("{}{}{}", left, " ".repeat(missing), right)
        }
    } else {
        let add_len = text.len() - terminal_string_width(text) - 2;
        text.pad_to_width(width + add_len)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use colored::Colorize;

    #[test]
    fn basics() {
        let x = Card::new().with_heavy_border();
        assert_eq!(x.border, Border::Heavy);
        let x = Card::new().with_light_border();
        assert_eq!(x.border, Border::Light);
        let x = Card::new().list(vec![
            String::from("1"),
            String::from("2"),
            String::from("3"),
        ]);
        assert_eq!(
            x.elements[0],
            Element::List(vec![
                String::from("1"),
                String::from("2"),
                String::from("3"),
            ])
        );
        let x = Card::new().text("Hello World!");
        assert_eq!(x.elements[0], Element::Text(String::from("Hello World!")));
        let x = Card::new().heavy_line().text("In between!").light_line();
        assert_eq!(
            x.elements,
            vec![
                Element::HeavyLine,
                Element::Text(String::from("In between!")),
                Element::LightLine,
            ]
        );
    }

    #[test]
    fn terminal_string_width_test() {
        let x = String::from("Hello World");
        let x_red = format!("{}", x.red());
        let x_red_black = format!("{}", x_red.on_black());
        let x_blink = format!("{}", x_red_black.blink());
        let x_dimmed = format!("{}", x_blink.dimmed());
        assert_eq!(terminal_string_width(&x), 11);
        assert_eq!(terminal_string_width(&x_red), 11);
        assert_eq!(terminal_string_width(&x_red_black), 11);
        assert_eq!(terminal_string_width(&x_blink), 11);
        assert_eq!(terminal_string_width(&x_dimmed), 11);
    }
}
