use crate::html::dom::elements::elements::HTMLElements;
use std::collections::HashMap;

#[derive(Debug, PartialEq, Clone)]
pub enum NodeType {
    TextNode(String),
    DomNode(ElementType),
}

#[derive(Debug, PartialEq, Clone)]
pub struct DOMNode {
    pub node_type: NodeType,
    pub children: Vec<DOMNode>,
}

impl DOMNode {
    pub fn text(value: String) -> Self {
        Self {
            children: vec![],
            node_type: NodeType::TextNode(value),
        }
    }

    pub fn elem(
        tag_name: HTMLElements,
        attributes: HashMap<String, String>,
        children: Vec<DOMNode>,
    ) -> Self {
        Self {
            children,
            node_type: NodeType::DomNode(ElementType {
                tag_name,
                attributes,
            }),
        }
    }

    pub fn get_attribute(&self, attr: &str) -> Option<String> {
        match &self.node_type {
            NodeType::DomNode(dom_node) => match dom_node.attributes.get(attr) {
                Some(value) => Some(value.to_string()),
                None => None,
            },
            NodeType::TextNode(_) => None,
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct ElementType {
    pub tag_name: HTMLElements,
    pub attributes: HashMap<String, String>,
}

impl ElementType {}
