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
/// # Panic
/// Panics if the `width` is zero!
pub fn wrap(text: &str, width: usize, border: &str) -> String {
    assert!(width > 0);
    if text.is_empty() {
        format!("{0}{1}{0}", border, expand(text, width))
    } else {
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
}

/// Create a list from an Iterator over Strings using the given `bullet` as bullet point
/// and put given `border` around each line.
/// A line is always as wide as `width`. Line wrapping included.
pub fn listify<I: Iterator<Item = String>>(
    items: I,
    bullet: char,
    width: usize,
    border: &str,
) -> String {
    let ret = items
        .map(|item| {
            let line = wrap_iter(&item, width - 2)
                .map(|s| s.pad_to_width(width - 2))
                .map(|s| format!("{0}  {1}{0}\n", border, s))
                .fold(String::new(), |mut s, line| {
                    s += &line;
                    s
                });
            let border_replace = format!("{} ", border);
            let border_bullet = format!("{}{}", border, bullet);
            line.replacen(&border_replace, &border_bullet, 1)
        })
        .fold(String::new(), |mut s, item| {
            s += &item;
            s
        })
        .trim_right_matches("\n")
        .to_string();
    if ret == String::new() {
        wrap("", width, border)
    } else {
        ret
    }
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
/// If the `text` is already wider than `width` do nothing.
/// Multiple `{}` will be ignored and removed, only the first one is used for expansion.
pub fn expand(text: &str, width: usize) -> String {
    let mut text = text.to_string();
    if !text.contains("{}") {
        text += "{}";
    }
    let text_stripped = text.replace("{}", "");
    let w = terminal_string_width(&text_stripped);
    if w > width {
        text_stripped
    } else {
        let mut parts = text.split("{}");
        let left = parts.next().unwrap();
        let right = concat(parts.map(|s| s.to_string()), "");
        let lw = terminal_string_width(left);
        let rw = terminal_string_width(&right);
        let missing = width - lw - rw;
        format!("{}{}{}", left, " ".repeat(missing), right)
    }
}

/// Concatenates the given `Vec<String>` to one String, seperated by `sep`.
pub fn concat<I: Iterator<Item = String>>(items: I, sep: &str) -> String {
    items.fold(String::new(), |mut s, item| {
        if s.len() > 0 {
            s += sep;
        }
        s += &item;
        s
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use colored::Colorize;
    use unicode_width::UnicodeWidthStr as UW;

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
        assert_eq!(x.width(), terminal_string_width(&x));
        assert!(x_red.width() != terminal_string_width(&x_red));

        let name = format!("{}", "Hello World".bold().yellow());
        assert_eq!(terminal_string_width(&name), 11);

        assert_eq!(terminal_string_width(""), 0);
    }

    #[test]
    fn wrap_test() {
        assert_eq!(
            wrap("Hello World", 3, "i"),
            "iHeli\nilo i\niWori\nild i".to_string()
        );
        assert_eq!(wrap("", 3, ""), String::from("   "));
        assert_eq!(wrap("", 4, "|"), String::from("|    |"));
        assert_eq!(wrap("Hey", 5, ","), String::from(",Hey  ,"));
        assert_eq!(wrap("Hello", 4, ""), String::from("Hell\no   "));
        assert_eq!(
            wrap("HelloWorld", 7, "/"),
            String::from("/HelloWo/\n/rld    /")
        );
        assert_eq!(wrap("Hey", 1, "e"), String::from("eHe\neee\neye"));
    }

    #[test]
    #[should_panic]
    fn wrap_test_fail() {
        wrap("Hey", 0, "x");
    }

    #[test]
    fn capitalize_test() {
        assert_eq!(capitalize("hello"), "Hello".to_string());
        assert_eq!(capitalize("ßello"), "SSello".to_string());
        assert_eq!(capitalize("0"), "0");
        assert_eq!(capitalize("-"), "-");
        assert_eq!(capitalize("-"), "-");
        assert_eq!(capitalize(""), "");
    }

    #[test]
    fn expand_test() {
        assert_eq!(expand("Hello{}World", 10), String::from("HelloWorld"),);
        assert_eq!(expand("{}X", 5), String::from("    X"),);
        assert_eq!(expand("X{}", 5), String::from("X    "),);
        assert_eq!(expand("X {}", 5), String::from("X    "),);
        assert_eq!(expand(" X{}", 5), String::from(" X   "),);
        assert_eq!(expand(" X{}Y ", 5), String::from(" X Y "),);
        assert_eq!(expand("{}", 10), " ".repeat(10),);
        assert_eq!(expand("{}X{}", 5), String::from("    X"),);
        assert_eq!(expand("Hello{}World", 10), String::from("HelloWorld"));
    }

    #[test]
    fn concat_test() {
        let mut array = vec![String::from("A"), String::from("B")];
        assert_eq!(concat(array.drain(..), "---"), String::from("A---B"));
        let mut array = vec![String::from("A")];
        assert_eq!(concat(array.drain(..), "123"), String::from("A"));
        let mut array: Vec<String> = vec![];
        assert_eq!(concat(array.drain(..), "|"), "");
    }

    #[test]
    fn listify_test() {
        fn ts(s: &str) -> String {
            s.to_string()
        }
        let mut a1 = vec!["Eins", "Zwei", "Drei"];
        let mut a2 = vec![];
        let mut a3 = vec!["Aliquam erat volutpat.  Nunc eleifend leo vitae magna.  In id erat non orci commodo lobortis."];
        let mut a4 = vec![
            "A long text, to long to display in one line",
            "Short one",
            "Yet another long one",
        ];
        assert_eq!(
            listify(a1.drain(..).map(ts), '-', 6, "r"),
            ts("r- Einsr\nr- Zweir\nr- Dreir"),
        );
        assert_eq!(listify(a2.drain(..).map(ts), '-', 5, "("), ts(&"(     ("),);
        assert_eq!(
            listify(a3.drain(..).map(ts), '#', 32, "+===+"),
            ts("+===+# Aliquam erat volutpat.  Nunc  +===+
+===+  eleifend leo vitae magna.  In +===+
+===+  id erat non orci commodo      +===+
+===+  lobortis.                     +===+")
        );
        assert_eq!(
            listify(a4.drain(..).map(ts), '⟶', 15, ":"),
            ts(":⟶ A long text, :
:  to long to   :
:  display in   :
:  one line     :
:⟶ Short one    :
:⟶ Yet another  :
:  long one     :")
        );
    }
}
