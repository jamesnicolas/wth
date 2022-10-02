use crate::Bookmark;
use crate::Content;
use roxmltree;

pub fn xml_string_to_bookmark(xml: String) -> Result<Bookmark, String> {
    // TODO: need a more robust replace in case people have weird bookmark titles or something
    let contents = xml.replace("<DT>","").replace("<p>","");

    let document: roxmltree::Document;
    match roxmltree::Document::parse(&contents) {
        Ok(doc) => document = doc,
        Err(error) => return Err(error.to_string()),
    }
    let mut root = Bookmark::new();

    fn traverse(xml_node: &roxmltree::Node, parent_tag: &str, parent_bookmark: &mut Bookmark) {
        let mut tag_name = "";
        let mut bookmark = Bookmark::new();
        match xml_node.node_type() {
            roxmltree::NodeType::Element =>  {
                tag_name = xml_node.tag_name().name();
                match tag_name {
                    "A" => {
                        let attribute = xml_node.attribute("HREF")
                            .expect("<A> tags should always have HREF attributes");
                        // since we're in an A tag, we can get the first child which should be text,
                        // which should be the title
                        let title: &str;
                        match xml_node.children().next() {
                            Some(text) => title = match text.node_type() {
                                roxmltree::NodeType::Text => text.text()
                                    .expect("Text.text() should always be Some(&str)"),
                                _ => attribute,
                            },
                            None => title = attribute,
                        };

                        bookmark.title = title.to_string();
                        bookmark.content = Content::Link(attribute.to_string());

                        if let Content::Folder(content) = &mut parent_bookmark.content {
                            content.push(bookmark);
                        }

                    },
                    _ => {},
                }
            },
            roxmltree::NodeType::Root => (),
            roxmltree::NodeType::Text => {
                match parent_tag {
                    "H1" | "H3" => {
                        parent_bookmark.title = xml_node.text().expect("Text.text() should always be Some(&str)").to_string();
                    },
                    _ => (),
                }
            },
            roxmltree::NodeType::Comment => return,
            roxmltree::NodeType::PI => return,
        }
        let mut children = xml_node.children();
        let mut option_child = children.next();
        loop {
            match option_child {
                Some(child) => {
                    match child.tag_name().name() {
                        "H1" | "H3" => {
                            let mut new_parent_bookmark = Bookmark::new();
                            traverse(&child, "H3", &mut new_parent_bookmark);
                            option_child = children.nth(1);

                            let dl_children = option_child.expect("DL should always follow H3").children();
                            for dl_child in dl_children {
                                traverse(&dl_child, "DL", &mut new_parent_bookmark);
                            }

                            if let Content::Folder(content) = &mut parent_bookmark.content {
                                content.push(new_parent_bookmark);
                            }
                        },
                        "DL" => {
                            option_child = children.next();
                            continue;
                        },
                        _ => traverse(&child, tag_name, parent_bookmark),
                    }
                }
                None => break,
            }
            option_child = children.next();
        }
    }

    traverse(&document.root(), "", &mut root);
    return Ok(root)
}
