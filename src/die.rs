use colored::Colorize;
use d20;

/// Execute a d20 expression.
///
/// Roll a die using a d20 expression. The expression should be of the form:
///
/// ```text
///         <roll> ::= <some_die> | [ "-" ] <constant> | <roll> [ <add_del> <roll> ]
///     <some_die> ::= <constant> "d" <constant>
///     <constant> ::= "0" | "1" | "2" | "3" | "4" | "5" | "6" | "7" | "8" | "9" | <constant>
///      <add_del> ::= "+" | "-"
/// ```
///
/// # Examples
///
/// |      Expression | Meaning                                                  |
/// |             --- | ---                                                      |
/// |             1d6 | Roll a normal die                                        |
/// |            1d20 | Roll a die with 20 sides                                 |
/// |            4d20 | Roll 4 die with 20 sides                                 |
/// |          9d4+14 | Roll 9 die with 4 sides and add 14                       |
/// |       3d3-9+2d6 | Roll 3 die with 3 sides subtract 9 and add 2 d6          |
/// |              -9 | Return -9                                                |
/// | -9+25-2+14-7+21 | Abuse this program to calculate the answer to everything |
///
pub fn roll<'a>(s: &'a str) {
    let mut s = s.to_owned();
    if s.starts_with("d") {
        s = format!("1{}", s);
    }
    let roll = d20::roll_dice(&s);
    match roll {
        Ok(roll) => {
            let total = format!("{}", roll.total).bold();
            println!("\n >> {}\n", total);
        }
        Err(fail) => {
            println!("Error: {}", fail);
        }
    }
}
