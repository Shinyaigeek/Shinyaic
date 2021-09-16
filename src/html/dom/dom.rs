use crate::html::dom::elements::elements::HTMLElements;
use std::collections::HashMap;

#[derive(Debug, PartialEq, Clone)]
pub enum NodeType {
    text_node(String),
    dom_node(ElementType),
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
            node_type: NodeType::text_node(value),
        }
    }

    pub fn elem(
        tag_name: HTMLElements,
        attributes: HashMap<String, String>,
        children: Vec<DOMNode>,
    ) -> Self {
        Self {
            children,
            node_type: NodeType::dom_node(ElementType {
                tag_name,
                attributes,
            }),
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct ElementType {
    pub tag_name: HTMLElements,
    pub attributes: HashMap<String, String>,
}

impl ElementType {}
