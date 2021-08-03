use crate::html::dom::dom::{DOMNode, ElementType, NodeType};

// TODO SelectorElmは正しいのか？, 属性selecytor
#[derive(Debug, PartialEq, Clone, Eq, Hash)]
pub enum SelectorElm {
    id(String),
    class(String),
    tag_name(String),
    asterisk(String),
}

#[derive(Debug, PartialEq, Clone, Eq, Hash)]
pub enum SelectorChildren {
    descendant_combinator(Vec<Selector>),
    child_combinator(Vec<Selector>),
    general_sibling_combinator(Vec<Selector>),
    adjacent_sibling_combinator(Vec<Selector>),
}

#[derive(Debug, PartialEq, Clone, Eq, Hash)]
pub struct Selector {
    pub elm: SelectorElm,
    pub children: Vec<SelectorChildren>,
}

impl Selector {
    pub fn matches(self, elm: &DOMNode) -> bool {
        let element_type = match elm.node_type {
            NodeType::text_node(text_node) => {
                return false;
            },
            NodeType::dom_node(element_type) => {
                element_type
            }
        };
        if self.children.len() == 0 {
            return match self.elm {
                SelectorElm::id(id) => id == element_type.attributes.get("id".to_string()),
                SelectorElm::class(class) => class == element_type.attributes.get("class".to_string()),
                SelectorElm::tag_name(tag_name) => tag_name == element_type.tag_name,
                SelectorElm::asterisk(asterisk) => {
                    true
                },
            }
        }

        for &child in self.children {
            match child {
                SelectorChildren::descendant_combinator(children) => { 
                    for &descendant in children {
                        for child in elm.children {
                            return descendant.matches(child);
                        }
                    }
                    return false;
                },
                SelectorChildren::child_combinator(children) => {
                    for &child in children {
                        for child_elm in elm.children {
                            return child.matches(child_elm);
                        }
                    }
                    return false;
                },
                SelectorChildren::general_sibling_combinator(children) => {
                    for &sibling in elm.children {
                        for child in children {
                            return child.matches(sibling);
                        }
                    }
                    return false;
                },
                SelectorChildren::adjacent_sibling_combinator(children) => {
                    for &sibling in elm.children {
                        for child in children {
                            return child.matches(sibling);
                        }
                    }
                    return false;
                },
            }
        }

        return false;
    }
}
// .class
// .class, .class2
// .class > .class2
