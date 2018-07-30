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

impl Border {
    /// Get a line of the given `width`.
    pub fn line(&self, width: usize) -> String {
        match *self {
            Border::Heavy => "━".repeat(width),
            Border::Light => "─".repeat(width),
        }
    }
    /// Get the first line with the given `width`.
    pub fn head(&self, width: usize) -> String {
        let line = self.line(width);
        match *self {
            Border::Heavy => format!(" ┏{}┓", line),
            Border::Light => format!(" ┌{}┐", line),
        }
    }
    /// Get the end line with the given `width`.
    pub fn end(&self, width: usize) -> String {
        let line = self.line(width);
        match self {
            Border::Heavy => format!(" ┗{}┛\n", line),
            Border::Light => format!(" └{}┘\n", line),
        }
    }
}
