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

impl TagTypu {
    fn from_string(s: String) -> Self {
        match s.as_str() {
            "H1" | "h1" => Heading1,
            "H3" | "h3" => Heading3,
            "TITLE" | "title" => Title,
            "DT" | "dt" => DataTable,
            "DL" | "dl" => DataList,
            "A" | "a" => Anchor,
            "P" | "p" => Paragraph,
            _ => Other,
        }
    }
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

#[derive(Debug, Clone)]
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
        let res = write!(f, "LexError: {} at {}", self.message, self.pos);
        if let Some(context) = self.context {
            write!(f, "{}\n", context);
            write!(f, "{width:width$}^\n", width=self.pos.col as usize)
        } else {
            res
        }
    }
}

fn get_tag<T: Iterator<Item = char>>(iter: &mut Peekable<T>) -> Result<LexToken, LexError> {
    let tag_name: [char; 3] = ['\0';3];
    let open = true;
    let index: usize = 0;
    while let Some(c) = iter.peek() {
        assert_eq!(index < 3, true);
        match c {
            ' ' | '>' => { 
                tag_name[index] = '\0';
                break;
            },
            '/' => {
                open = false;
                break;
            },
            _ => {
                tag_name[index] = *c;
            },
        }
    }
    let tag_name: String = tag_name[0..index].iter().collect();
    let tag_type = TagTypu::from_string(tag_name);
    Ok(LexToken::OpenTagStart(tag_type))
}

pub fn lex_tokens(content: String) -> Vec<LexToken> {
    let mut result = Vec::new();
    let mut it = content.chars().peekable();
    while let Some(&c) = it.peek() {
        match c {
            '<' => {
                if let Ok(token) = get_tag(&mut it) {
                    result.push(token);
                }
            }
            '>' => {
                result.push(LexToken::OpenTagEnd);
            }
        }
    }
    result
}

#[cfg(test)]
mod test {
    #[test]
    fn basic() {
        let input = "<A HREF=\"www.google.com\">Google</A>";
    }
}
