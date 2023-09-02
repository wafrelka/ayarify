use std::io;
use std::io::{Cursor, Read, Write};
use std::rc::Rc;

use html5ever::driver::ParseOpts;
use html5ever::tendril::TendrilSink;
use html5ever::{local_name, namespace_url, ns, parse_document, serialize, LocalName, QualName};
use markup5ever_rcdom::{Node, NodeData, RcDom, SerializableHandle};

macro_rules! html_name {
    ($name:tt) => {
        QualName { prefix: None, ns: ns!(html), local: local_name!($name) }
    };
}

type Level = u8;

struct ContextStack {
    stack: Vec<(Rc<Node>, Level)>,
}

impl ContextStack {
    fn new() -> Self {
        Self { stack: Vec::new() }
    }

    fn last(&self) -> Option<&(Rc<Node>, Level)> {
        self.stack.last()
    }

    fn push(&mut self, item: (Rc<Node>, Level)) {
        if let Some(last) = self.stack.last() {
            assert!(last.1 < item.1);
        }
        self.stack.push(item);
    }

    fn sweep(&mut self, level: Level) {
        while self.stack.last().map_or(false, |c| c.1 >= level) {
            self.stack.pop();
        }
    }
}

fn compute_level(name: &QualName) -> Option<Level> {
    let mapping = vec![
        (html_name!("h1"), 1),
        (html_name!("h2"), 2),
        (html_name!("h3"), 3),
        (html_name!("h4"), 4),
        (html_name!("h5"), 5),
        (html_name!("h6"), 6),
    ];
    mapping.iter().find(|(tag, _)| tag == name).map(|(_, level)| *level)
}

fn ayarify_node(node: Rc<Node>) {
    let children = node.children.replace(Vec::new());
    let mut stack = ContextStack::new();
    stack.push((node, 0));

    let add = |stack: &ContextStack, child: Rc<Node>| {
        let last = stack.last().expect("context should not be empty");
        let parent = &last.0;
        child.parent.replace(Some(Rc::downgrade(parent)));
        parent.children.borrow_mut().push(child);
    };

    for child in children.into_iter() {
        if let NodeData::Element { name, attrs, .. } = &child.data {
            if let Some(level) = compute_level(name) {
                let new = Node::new(NodeData::Element {
                    name: html_name!("div"),
                    attrs: attrs.replace(Vec::new()).into(),
                    template_contents: None,
                    mathml_annotation_xml_integration_point: false,
                });
                stack.sweep(level);
                add(&stack, new.clone());
                stack.push((new, level));
            }
        }
        add(&stack, child);
    }
}

fn ayarify_tree(tree: &Rc<Node>) {
    for child in tree.children.borrow().iter() {
        ayarify_tree(child);
    }
    ayarify_node(tree.clone());
}

fn has_attribute(node: &Rc<Node>, attribute: &str) -> bool {
    let name = QualName { prefix: None, ns: ns!(), local: LocalName::from(attribute) };
    if let NodeData::Element { attrs, .. } = &node.data {
        attrs.borrow().iter().any(|a| a.name == name)
    } else {
        false
    }
}

fn ayarify_marked_tree(tree: &Rc<Node>, attribute: &str) {
    if has_attribute(tree, attribute) {
        ayarify_tree(tree);
    } else {
        for child in tree.children.borrow().iter() {
            ayarify_marked_tree(child, attribute);
        }
    }
}

pub fn ayarify<R: Read, W: Write>(src: &mut R, dest: W, attribute: Option<&str>) -> io::Result<()> {
    let dom = parse_document(RcDom::default(), ParseOpts::default()).from_utf8().read_from(src)?;

    if let Some(attribute) = attribute {
        ayarify_marked_tree(&dom.document, attribute);
    } else {
        ayarify_tree(&dom.document);
    }

    let document: SerializableHandle = dom.document.into();
    serialize(dest, &document, Default::default())?;

    Ok(())
}

pub fn ayarify_str(html: &str, attribute: Option<&str>) -> String {
    let mut reader = Cursor::new(html);
    let mut writer: Vec<u8> = Vec::new();
    ayarify(&mut reader, &mut writer, attribute).expect("io should not fail");
    String::from_utf8_lossy(&writer).into_owned()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_ayarify_str() {
        let html = r#"<!DOCTYPE html><html><head></head><body>
<h2>header1</h2><p>para1</p>
<div data-ayarify="">
<h2 class="cls">header2</h2><p>para2</p>
<h4>header3</h4><p>para3</p>
<h3>header4</h3><p>para4</p>
<h2>header5</h2><p>para5</p>
</div>
</body></html>"#;

        let expected = r#"<!DOCTYPE html><html><head></head><body>
<h2>header1</h2><p>para1</p>
<div data-ayarify="">
<div class="cls"><h2>header2</h2><p>para2</p>
<div><h4>header3</h4><p>para3</p>
</div><div><h3>header4</h3><p>para4</p>
</div></div><div><h2>header5</h2><p>para5</p>
</div></div>
</body></html>"#;

        let actual = ayarify_str(html, Some("data-ayarify"));
        assert_eq!(actual, expected);
    }
}
