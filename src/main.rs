use std::io;
use std::rc::Rc;

use markup5ever_rcdom as rcdom;

use html5ever::driver::ParseOpts;
use html5ever::tendril::TendrilSink;
use html5ever::{
    local_name, namespace_url, ns, parse_document, serialize, Attribute, LocalName, QualName,
};
use rcdom::{Handle, Node, NodeData, RcDom, SerializableHandle};

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

        match child.data {
            NodeData::Element { name: html_name!("h1"), ref attrs, .. } => {
                add_new_context(1, attrs.replace(vec![]));
            }
            NodeData::Element { name: html_name!("h2"), ref attrs, .. } => {
                add_new_context(2, attrs.replace(vec![]));
            }
            NodeData::Element { name: html_name!("h3"), ref attrs, .. } => {
                add_new_context(3, attrs.replace(vec![]));
            }
            NodeData::Element { name: html_name!("h4"), ref attrs, .. } => {
                add_new_context(4, attrs.replace(vec![]));
            }
            NodeData::Element { name: html_name!("h5"), ref attrs, .. } => {
                add_new_context(5, attrs.replace(vec![]));
            }
            NodeData::Element { name: html_name!("h6"), ref attrs, .. } => {
                add_new_context(6, attrs.replace(vec![]));
            }
            _ => {}
        }

        let last = stack.last().expect("empty context");
        link_nodes(&last.0, child);
    }
}

fn is_ayarifiable(node: &Handle) -> bool {

    let ayarify_attribute =
        QualName { prefix: None, ns: ns!(), local: LocalName::from("data-ayarify") };

    match &node.data {
        NodeData::Element { attrs, .. } => {
            attrs.borrow().iter().any(|a| a.name == ayarify_attribute)
        }
        _ => false,
    }
}

fn ayarify(root: &Handle) {
    if is_ayarifiable(root) {
        ayarify_all(root);
    } else {
        for child in root.children.borrow().iter() {
            ayarify(child);
        }
    }
}

fn main() {

    let stdin = io::stdin();
    let dom = parse_document(RcDom::default(), ParseOpts::default())
        .from_utf8()
        .read_from(&mut stdin.lock())
        .expect("reader failed");

    ayarify(&dom.document);

    let document: SerializableHandle = dom.document.into();
    serialize(&mut io::stdout(), &document, Default::default()).expect("serialization failed");
}
