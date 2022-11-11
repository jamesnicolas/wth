use crate::Bookmark;
use crate::Content;
use roxmltree;
use tl;
use std::collections::HashSet;

pub fn html_string_to_bookmark(html: String) {
    let dom = tl::parse(&html, tl::ParserOptions::default()).unwrap();
    let mut bookmark_stack = vec![Bookmark::new_folder("".to_string())];
    let mut node_stack: Vec<tl::NodeHandle> = vec![];
    let mut last_header: Option<tl::NodeHandle> = None;
    let mut visited = HashSet::new();
    let children = dom.children().to_vec();
    for child in children {
        node_stack.push(child);
        loop {
            if let Some(node) = node_stack.pop() {
                match node.get(dom.parser()).expect("NodeHandle is valid for the parser it originated from") {
                    tl::Node::Tag(tag) => {
                        let name: String = tag.name().as_utf8_str().into();
                        match name.as_str() {
                            "A" => {
                                let title = tag.inner_text(dom.parser());
                                let url = tag.attributes().get("HREF").expect("<A> tags have an HREF attribute").expect("HREF attributes are strings");
                                bookmark_stack.last_mut().expect("The bookmark stack is non-empty while parsing")
                                    .add(Bookmark::new_link(title.to_string(), url.as_utf8_str().to_string())).expect("Bookmarks of type Folder can be appended to");
                                visited.insert(node);
                            },
                            "DL"=> {
                                let title = last_header.expect("DL always follows a Header tag").get(dom.parser())
                                    .expect("Header tags have inner text").inner_text(dom.parser());
                                bookmark_stack.push(Bookmark::new_folder(title.to_string()));
                            },
                            "H1" | "H3" => {
                                last_header = Some(node);
                                continue;
                            },
                            "DT" | "p" | "META" | "TITLE" => {
                                let children = node.get(dom.parser())
                                    .expect("NodeHandle is valid for the parser it originated from")
                                    .children()
                                    .expect("This is a Node::Tag");
                                node_stack.append(&mut children.top().to_vec().into_iter().rev().collect());
                                continue;
                            },
                            _ => continue,
                        };
                    },
                    _ => continue,
                }
            } else {
                break;
            }
        }
    }
}

pub fn xml_string_to_bookmark(xml: String) -> Result<Bookmark, String> {
    // TODO: need a more robust replace in case people have weird bookmark titles or something
    // TODO: Okay html really is not a subset of xml. Sorry future me. stop using an xml parser
    let contents = xml.replace("<DT>","").replace("<p>","").replace("&","&amp;");

    let document: roxmltree::Document;
    match roxmltree::Document::parse(&contents) {
        Ok(doc) => document = doc,
        Err(error) => {
            let lines = contents.split("\n");
            let pos = error.pos();
            let mut y = pos.row - 1;
            let x = pos.col - 1;

            for line in lines {
                if y == 0 {
                    println!("{}", line);
                    println!("{:width$}^", "x", width=x as usize);
                    break
                }
                y -= 1;
            }
            return Err(format!("Error parsing xml: {}", error));
        }
    }
    let mut root = Bookmark::new_folder("".to_string());

    fn set_title(bookmark: &mut Bookmark, xml_node: &roxmltree::Node, default: &str) {
        if let roxmltree::NodeType::Element = xml_node.node_type() {
            match xml_node.tag_name().name() {
                "H1" | "H3" | "A" => {
                    let child = xml_node.children().next().expect("name required");
                    if let roxmltree::NodeType::Text = child.node_type() {
                        let title = child.text().expect("Text.text() should always be Some(&str)").into();
                        bookmark.title = title;
                    }
                }
                _ => bookmark.title = default.into(),
            }
        } else {
            bookmark.title = default.into();
        }
    }

    fn h3_dl_to_bookmark(h3_node: roxmltree::Node, dl_node: roxmltree::Node) -> Bookmark {
        let mut bookmark = Bookmark::new_folder("".to_string());
        set_title(&mut bookmark, &h3_node, "undefined");
        traverse_children(&mut dl_node.children(), &mut bookmark);
        bookmark
    }

    fn traverse_children(xml_children: &mut roxmltree::Children, bookmark: &mut Bookmark) {
        let mut h3_option: Option<roxmltree::Node> = None;
        let mut child_option = xml_children.next();
        loop {
            if let Some(child) = child_option {
                if let Some(h3) = h3_option {
                    let dl = child;
                    if dl.tag_name().name() != "DL" {
                        child_option = xml_children.next();
                        continue;
                    } else {
                        if let Content::Folder(content) = &mut bookmark.content {
                            content.push(h3_dl_to_bookmark(h3, dl));
                        }
                        h3_option = None;
                        child_option = xml_children.next();
                        continue;
                    }
                }
                if child.tag_name().name() == "H3" || child.tag_name().name() == "H1" {
                    h3_option = Some(child);
                    child_option = xml_children.next();
                    continue;
                }
                if child.tag_name().name() == "A" {
                    let href = child.attribute("HREF").expect("<A> tag should always have HREF attributes");
                    let mut link = Bookmark::new_link("".into(), href.into());
                    set_title(&mut link, &child, href);

                    if let Content::Folder(content) = &mut bookmark.content {
                        content.push(link);
                    }
                    child_option = xml_children.next();
                    continue;
                }
                if child.has_children() {
                    traverse_children(&mut child.children(), bookmark);
                }
            } else {
                break;
            }
            child_option = xml_children.next();
        }
    }

    traverse_children(&mut document.root().children(), &mut root);
    return Ok(root)
}

#[cfg(test)]
mod test {
    use crate::parse::html_string_to_bookmark;
    #[test]
    fn basic() {
        html_string_to_bookmark("<p>test".into());
    }

    #[test]
    fn basic2() {
        html_string_to_bookmark("<p>test</p>".into());
    }
}
