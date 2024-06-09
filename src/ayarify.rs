use std::mem::take;

use ego_tree::{NodeId, NodeRef};
use html5ever::interface::ElementFlags;
use html5ever::tree_builder::TreeSink;
use html5ever::{local_name, namespace_url, ns, QualName};
use scraper::{Html, Node};

use crate::layer::{compute_layer_parents, Level};

macro_rules! html_name {
    ($name:tt) => {
        QualName { prefix: None, ns: ns!(html), local: local_name!($name) }
    };
}

fn get_header_level_from_qual_name(name: &QualName) -> Option<Level> {
    let level = |n: u8| -> Level { Level::new(n).unwrap() };
    let mapping = [
        (html_name!("h1"), level(1)),
        (html_name!("h2"), level(2)),
        (html_name!("h3"), level(3)),
        (html_name!("h4"), level(4)),
        (html_name!("h5"), level(5)),
        (html_name!("h6"), level(6)),
    ];
    mapping.into_iter().find(|(tag, _)| tag == name).map(|(_, lv)| lv)
}

fn get_header_level(node: NodeRef<Node>) -> Option<Level> {
    node.value().as_element().and_then(|e| get_header_level_from_qual_name(&e.name))
}

fn wrap_header(document: &mut Html, node_id: NodeId) -> NodeId {
    let mut node = document.tree.get_mut(node_id).unwrap();
    let attrs = match node.value() {
        Node::Element(elem) => take(&mut elem.attrs),
        _ => Default::default(),
    };
    let attrs = attrs.into_iter().map(|(name, value)| html5ever::Attribute { name, value });
    let wrapper_id =
        document.create_element(html_name!("div"), attrs.collect(), ElementFlags::default());
    document.append(&wrapper_id, html5ever::interface::NodeOrText::AppendNode(node_id));
    wrapper_id
}

fn ayarify_node(document: &mut Html, node_id: NodeId) {
    let node = document.tree.get(node_id).unwrap();

    let children: Vec<_> = node.children().map(|c| (get_header_level(c), c.id())).collect();
    let parents = compute_layer_parents(children.iter().map(|(l, _)| l).copied());

    let mut replaced = Vec::new();

    for (index, parent) in parents.into_iter().enumerate() {
        let (level, id) = children[index];

        document.remove_from_parent(&id);

        let id = match level {
            Some(_) => wrap_header(document, id),
            _ => id,
        };

        let new_parent = if index == parent { node_id } else { replaced[parent] };
        document.append(&new_parent, html5ever::interface::NodeOrText::AppendNode(id));

        replaced.push(id);
    }
}

fn ayarify_tree(document: &mut Html, node_id: NodeId) {
    let node = document.tree.get(node_id).unwrap();
    let children: Vec<_> = node.children().map(|c| c.id()).collect();
    for child in children {
        ayarify_tree(document, child);
    }
    ayarify_node(document, node_id);
}

fn has_attribute(node: NodeRef<Node>, attribute: &str) -> bool {
    if let Node::Element(elem) = node.value() {
        elem.attrs().any(|(name, _)| name.eq_ignore_ascii_case(attribute))
    } else {
        false
    }
}

fn ayarify_marked_tree(document: &mut Html, node_id: NodeId, attribute: &str) {
    let node = document.tree.get(node_id).unwrap();
    if has_attribute(node, attribute) {
        ayarify_tree(document, node_id);
    } else {
        let children: Vec<_> = node.children().map(|c| c.id()).collect();
        for child in children {
            ayarify_marked_tree(document, child, attribute);
        }
    }
}

pub fn ayarify(src: &str, attribute: Option<&str>) -> String {
    let mut document = Html::parse_document(src);
    let root = document.root_element().id();
    if let Some(attribute) = attribute {
        ayarify_marked_tree(&mut document, root, attribute);
    } else {
        ayarify_tree(&mut document, root);
    }

    document.html()
}

#[cfg(test)]
mod test {
    use regex::Regex;

    use super::*;

    fn trim_spaces(html: &str) -> String {
        let before = Regex::new(r"\s+<").unwrap();
        let after = Regex::new(r">\s+").unwrap();
        let html = before.replace_all(html, "<");
        after.replace_all(&html, ">").to_string()
    }

    #[rstest::rstest]
    #[case::sample(include_str!("../testdata/sample.html"), include_str!("../testdata/sample.expected.html"))]
    #[test]
    fn test_ayarify(#[case] input: &str, #[case] expected: &str) {
        let actual = ayarify(input, Some("data-ayarify"));
        assert_eq!(trim_spaces(&actual), trim_spaces(expected));
    }
}
