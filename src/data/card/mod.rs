//! Terminal Card
//!
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

mod border;
pub mod helper;

pub use self::helper::*;

use self::border::Border;
use std::fmt;

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
    #[allow(dead_code)]
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
    /// Add a light line if `pred` is `true`.
    pub fn light_line_if(self, pred: bool) -> Self {
        if pred {
            self.light_line()
        } else {
            self
        }
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
    /// Add a text if `pred` is `true`.
    pub fn text_if(self, s: &str, pred: bool) -> Self {
        if pred {
            self.text(s)
        } else {
            self
        }
    }
    /// Add a single line, with spacing at `{}`.
    pub fn line(mut self, s: &str) -> Self {
        self.elements.push(Element::Line(String::from(s)));
        self
    }
    /// Add a single line, with spacing at `{}` if `pred` is `true`.
    pub fn line_if(self, s: &str, pred: bool) -> Self {
        if pred {
            self.line(s)
        } else {
            self
        }
    }
    /// Add a list of items.
    pub fn list(mut self, list: Vec<String>) -> Self {
        self.elements.push(Element::List(list));
        self
    }
}

impl fmt::Display for Card {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        let border = match self.border {
            Border::Heavy => " ┃ ",
            Border::Light => " ┃ ",
        };
        let mut first = vec![self.border.head(self.width)];
        let els = self.elements.iter().map(|ref el| match el {
            Element::LightLine => format!(" ┠{}┨", Border::Light.line(self.width)),
            Element::HeavyLine => format!(" ┣{}┫", Border::Heavy.line(self.width)),
            Element::Text(s) => format!("{}", wrap(s, self.width - 2, border)),
            Element::Line(l) => format!("{0}{1}{0}", border, expand(l, self.width - 2)),
            Element::List(v) => format!(
                "{}",
                listify(v.iter().map(|v| v.clone()), '•', self.width - 2, border,)
            ),
        });
        let last = vec![self.border.end(self.width)];
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

#[cfg(test)]
mod tests {
    use super::*;

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
}
