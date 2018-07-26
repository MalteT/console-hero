use pad::PadStr;
use textwrap::wrap_iter;
use unicode_width::UnicodeWidthStr as UW;

mod monsters;
mod moves;

pub use self::monsters::Monster;
pub use self::monsters::Monsters;
pub use self::moves::Move;
pub use self::moves::Moves;

/// Wraps the given String by word wrapping at the given
/// `width` and adds the given `left` and `right` Strings to each line,
/// returning concatinated lines with `\n`s.
fn wrap(text: &str, width: usize, left: &str, right: &str) -> String {
    wrap_iter(text, width)
        .map(|s| s.pad_to_width(width))
        .map(|s| format!("{}{}{}\n", left, s, right))
        .fold(String::new(), |mut s, desc| {
            s += &desc;
            s
        })
        .trim_right_matches("\n")
        .to_string()
}

/// Expands the given string `text` at `{}` to match the given `width`.
/// If the `text` is already wider than `width` do nothing
fn expand(text: &str, width: usize) -> String {
    let w = UW::width(text);
    if w > width {
        text.replacen("{}", "", 1).to_string()
    } else {
        let parts: Vec<&str> = text.split("{}").collect();
        let left = parts[0];
        let right = parts[1];
        let lw = UW::width(left);
        let rw = UW::width(right);
        let missing = width - lw - rw;
        format!("{}{}{}", left, " ".repeat(missing), right)
    }
}

/// Concatenates the given `Vec<String>` to one String, seperated by `sep`.
fn concat<'a, I>(items: I, sep: &str) -> String
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

/// Capitalizes the given String `s`.
fn capitalize(s: &str) -> String {
    let mut c = s.chars();
    match c.next() {
        None => String::new(),
        Some(f) => f.to_uppercase().collect::<String>() + c.as_str(),
    }
}

/// A thin line of length `width`.
fn thin_line(width: usize) -> String {
    "─".repeat(width).to_string()
}

/// A bold line of length `width`.
fn bold_line(width: usize) -> String {
    "━".repeat(width).to_string()
}

/// Make a list out of an Iterator over Strings.
fn listify<'a, I>(items: I, bullet: char, width: usize, left: &str, right: &str) -> String
where
    I: Iterator<Item = String>,
{
    items
        .map(|item| {
            let item = item.pad_to_width(width - 2);
            format!("{}{} {}{}\n", left, bullet, item, right)
        })
        .fold(String::new(), |mut s, item| {
            s += &item;
            s
        })
        .trim_right_matches("\n")
        .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic() {
        assert_eq!(
            wrap("Hello World", 3, ">", "<"),
            ">Hel<\n>lo <\n>Wor<\n>ld <".to_string()
        );
        assert_eq!(capitalize("hello"), "Hello".to_string());
        assert_eq!(capitalize("ßello"), "SSello".to_string());
        assert_eq!(bold_line(0), String::new());
        assert_eq!(
            bold_line(10),
            String::from("━━━━━━━━━━")
        );
        assert_eq!(
            expand("Hello{}World", 30),
            String::from("Hello                    World")
        );
        assert_eq!(expand("Hello{}World", 10), String::from("HelloWorld"));
        let array = vec![String::from("A"), String::from("B")];
        assert_eq!(
            concat(array.iter().map(|s| s.clone()), "---"),
            String::from("A---B")
        );
    }

    #[test]
    fn border_cases() {
        assert_eq!(expand("Hello{}World{}", 15), "Hello     World".to_string());
    }
}
