use std::fmt;
use std::iter::Peekable;

enum TagTypu {
    Heading1,
    Heading3,
    Title,
    DataTable,
    DataList,
    Anchor,
    Paragraph,
    Comment,
    Other,
}
struct Tag {
    typu: TagTypu,
    href: Option<String>,
    text: Option<String>,
}

enum Mode {
    Opening,
    Escape,
}

enum LexToken {
    OpenTagStart(TagTypu),
    AttributeKey(),
    AttributeValue(),
    OpenTagEnd,
    CloseTag(TagTypu),
    Text(String),
    Comment(String),
    DocType(String),
}

struct Pos {
    col: u32,
    row: u32,
}

impl fmt::Display for Pos {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "row: {} col: {}", self.row, self.col)
    }
}

#[derive(Debug, Clone)]
struct LexError {
    pos: Pos,
    message: String,
    context: Option<String>,
}

impl fmt::Display for LexError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "LexError: {} at {}", self.message, self.pos)?;
        if let Some(context) = self.context {
            write!(f, "{}\n", context);
            write!(f, "{:width$}^\n", width=self.pos.col as usize)
        }
    }
}

fn get_open_tag_start<T: Iterator<Item = char>>(iter: &mut Peekable<T>) -> Result<LexToken, LexError> {
    let tag_name: [char; 3] = ['\0';3];
    let index: usize = 0;
    while let Some(c) = iter.peek() {
        assert_eq!(index < 3, true);
        match c {
            ' ' | '>' => { 
                tag_name[index] = '\0';
                break;
            },
            _ => {
                tag_name[index] = c;
            }
        }
    }
}

pub fn lex_tokens(content: String) -> Vec<LexToken> {
    let char_stack = Vec::new();
    let mode = Mode::Opening;
    let mut iter = content.chars();
    let mut result = Vec::new();

    let mut it = input.chars().peekable();
    while let Some(&c) = it.peek() {
        match c {
            '<' => {
                result.push(LexToken::OpenTag(n));
            }
        }
    }
}

#[cfg(test)]
mod test {
    #[test]
    fn basic() {
        let input = "<A HREF=\"www.google.com\">Google</A>";
    }
}
