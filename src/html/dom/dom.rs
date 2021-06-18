use std::collections::HashMap;

#[derive(Debug, PartialEq, Clone)]
pub enum NodeType {
    text_node(String),
    dom_node(ElementType),
}

#[derive(Debug, PartialEq, Clone)]
pub struct DOMNode {
    children: Vec<DOMNode>,
    node_type: NodeType,
}

impl DOMNode {
    pub fn text(value: String) -> Self {
        Self {
            children: vec![],
            node_type: NodeType::text_node(value),
        }
    }

    pub fn elem(
        tag_name: String,
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
    // TODO できればenum
    tag_name: String,
    attributes: HashMap<String, String>,
}

impl ElementType {
    pub fn get_attributes_value(&self, key: String) -> String {
        if self.attributes.get(&key).is_none() {
            return self.attributes.get(&key).unwrap().to_string();
        }

        "".to_string()
    }
}
