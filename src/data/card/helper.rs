use pad::PadStr;
use regex::Regex;
use textwrap::wrap_iter;
use unicode_width::UnicodeWidthStr as UW;

/// Calculate the width of a string containing escape codes for coloring.
pub fn terminal_string_width(s: &str) -> usize {
    let re = Regex::new(r"\x1B\[.*?m").unwrap();
    re.replace_all(s, "").width()
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

/// Capitalizes the given String `s`.
pub fn capitalize(s: &str) -> String {
    let mut c = s.chars();
    match c.next() {
        None => String::new(),
        Some(f) => f.to_uppercase().collect::<String>() + c.as_str(),
    }
}

/// Expands the given string `text` at `{}` to match the given `width`.
/// If the `text` is already wider than `width` do nothing
pub fn expand(text: &str, width: usize) -> String {
    let mut text = text.to_string();
    if !text.contains("{}") {
        text += "{} ";
    } else if text.ends_with("{}") {
        text += " ";
    }
    let w = terminal_string_width(&text);
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
}

/// Concatenates the given `Vec<String>` to one String, seperated by `sep`.
pub fn concat<'a, I>(items: I, sep: &str) -> String
where
    I: Iterator<Item = String>,
{
    items.fold(String::new(), |mut s, item| {
        if s.len() > 0 {
            s += sep;
        }
        s += &item;
        s
    })
}
