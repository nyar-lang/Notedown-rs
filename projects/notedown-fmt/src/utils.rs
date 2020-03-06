use pangu::spacing;
use std::borrow::{Borrow, Cow};
pub use textwrap::{dedent, indent};

pub fn pangu_space(text: &str) -> Cow<str> {
    spacing(text)
}

pub fn count_indent(text: &str) -> usize {
    let mut spaces = 0;
    for c in text.chars() {
        match c {
            ' ' => spaces += 1,
            _ => break,
        }
    }
    return spaces;
}

pub fn dedent_less_than(input: &str, indent: usize) -> String {
    let new_text = input.lines().map(|line| {
        let mut max = indent;
        line.chars()
            .skip_while(|c| {
                if max == 0 {
                    false
                }
                else {
                    max -= 1;
                    c.is_whitespace()
                }
            })
            .collect::<String>()
    });
    new_text.collect::<Vec<_>>().join("\n")
}
