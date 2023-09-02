use std::collections::HashMap;
use std::io;
use std::rc::Rc;

use markup5ever_rcdom as rcdom;

use html5ever::driver::ParseOpts;
use html5ever::tendril::TendrilSink;
use html5ever::{
    local_name, namespace_url, ns, parse_document, serialize, Attribute, LocalName, QualName,
};
use rcdom::{Handle, Node, NodeData, RcDom, SerializableHandle};
use structopt::StructOpt;

macro_rules! html_name {
    ($name:tt) => {
        QualName { prefix: None, ns: ns!(html), local: local_name!($name) }
    };
}

fn ayarify_all(root: &Handle) {
    for child in root.children.borrow().iter() {
        ayarify_all(child);
    }

    let children = root.children.replace(vec![]);
    let mut stack = vec![(root.clone(), 0)];

    let link_nodes = |parent: &Handle, child: &Handle| {
        parent.children.borrow_mut().push(child.clone());
        child.parent.replace(Some(Rc::downgrade(parent)));
    };

    for child in children.iter() {
        let mut add_new_context = |level: i32, attrs: Vec<Attribute>| {
            while stack.last().map_or(false, |c| c.1 >= level) {
                stack.pop();
            }
            let elem = NodeData::Element {
                name: html_name!("div"),
                attrs: attrs.into(),
                template_contents: None,
                mathml_annotation_xml_integration_point: false,
            };
            let node = Node::new(elem);
            let last = stack.last().expect("empty context");
            link_nodes(&last.0, &node);
            stack.push((node, level));
        };

        if let NodeData::Element { ref name, ref attrs, .. } = child.data {
            let tags = HashMap::from([
                (html_name!("h1"), 1),
                (html_name!("h2"), 2),
                (html_name!("h3"), 3),
                (html_name!("h4"), 4),
                (html_name!("h5"), 5),
                (html_name!("h6"), 6),
            ]);
            for (tag, level) in tags {
                if tag == *name {
                    add_new_context(level, attrs.replace(vec![]));
                    break;
                }
            }
        }

        let last = stack.last().expect("empty context");
        link_nodes(&last.0, child);
    }
}

fn is_ayarifiable(node: &Handle, ayarify_attr_name: &str) -> bool {
    let ayarify_attribute =
        QualName { prefix: None, ns: ns!(), local: LocalName::from(ayarify_attr_name) };

    match &node.data {
        NodeData::Element { attrs, .. } => {
            attrs.borrow().iter().any(|a| a.name == ayarify_attribute)
        }
        _ => false,
    }
}

fn ayarify(root: &Handle, ayarify_attr_name: &str) {
    if is_ayarifiable(root, ayarify_attr_name) {
        ayarify_all(root);
    } else {
        for child in root.children.borrow().iter() {
            ayarify(child, ayarify_attr_name);
        }
    }
}

#[derive(Debug, StructOpt)]
struct Options {
    #[structopt(short, long)]
    attribute: Option<Option<String>>,
}

fn main() {
    let options = Options::from_args();

    let stdin = io::stdin();
    let dom = parse_document(RcDom::default(), ParseOpts::default())
        .from_utf8()
        .read_from(&mut stdin.lock())
        .expect("reader failed");

    if let Some(attr) = options.attribute {
        ayarify(&dom.document, attr.as_deref().unwrap_or("data-ayarify"));
    } else {
        ayarify_all(&dom.document);
    }

    let document: SerializableHandle = dom.document.into();
    serialize(&mut io::stdout(), &document, Default::default()).expect("serialization failed");
}

#[cfg(test)]
mod test {

    use html5ever::tree_builder::TreeBuilderOpts;

    use super::*;

    fn trim_spaces(s: &str) -> String {
        s.split('\n').map(|line| line.trim()).collect::<Vec<_>>().join("")
    }

    #[test]
    fn test_ayarify() {
        let html = r#"
            <!doctype html>
            <html>
                <head></head>
                <body>
                    <h2>header1</h2><p>para1</p>
                    <div data-ayarify="true">
                        <h2 class="cls">header2</h2><p>para2</p>
                        <h4>header3</h4><p>para3</p>
                        <h3>header4</h3><p>para4</p>
                        <h2>header5</h2><p>para5</p>
                    </div>
                </body>
            </html>
        "#;
        let expected = r#"
            <html>
                <head></head>
                <body>
                    <h2>header1</h2><p>para1</p>
                    <div data-ayarify="true">
                        <div class="cls">
                            <h2>header2</h2><p>para2</p>
                            <div><h4>header3</h4><p>para3</p></div>
                            <div><h3>header4</h3><p>para4</p></div>
                        </div>
                        <div><h2>header5</h2><p>para5</p></div>
                    </div>
                </body>
            </html>
        "#;

        let opts = ParseOpts {
            tree_builder: TreeBuilderOpts { drop_doctype: true, ..Default::default() },
            ..Default::default()
        };

        let dom = parse_document(RcDom::default(), opts)
            .from_utf8()
            .read_from(&mut trim_spaces(html).as_bytes())
            .unwrap();

        ayarify(&dom.document, "data-ayarify");

        let mut output = vec![];
        let document: SerializableHandle = dom.document.into();
        serialize(&mut output, &document, Default::default()).expect("serialization failed");

        let actual = String::from_utf8_lossy(&output);

        assert_eq!(actual, trim_spaces(expected));
    }
}
