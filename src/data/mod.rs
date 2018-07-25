use textwrap::wrap_iter;
use pad::PadStr;

mod moves;

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
}
