use crate::html::dom::elements::elements::HTMLElements;
use konnnyaku_client::Client;
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

    pub fn get_external_css(&self) -> String {
        let mut css_source = String::from("");

        match &self.node_type {
            NodeType::DomNode(dom_node) => match dom_node.tag_name {
                HTMLElements::LinkElement => {
                    if dom_node.attributes.get("rel").unwrap_or(&"".to_string()) == "stylesheet" {
                        let external_css_href = dom_node.attributes.get("href").unwrap();
                        let external_css_source = Client::get(external_css_href.to_string());
                        let external_css_source = external_css_source.body;
                        css_source.push_str(&external_css_source)
                    }
                }
                _ => {
                    for child in self.children.clone() {
                        css_source.push_str(&child.get_external_css());
                    }
                }
            },
            NodeType::TextNode(_) => {
                return css_source;
            }
        };

        css_source
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct ElementType {
    pub tag_name: HTMLElements,
    pub attributes: HashMap<String, String>,
}

impl ElementType {}
