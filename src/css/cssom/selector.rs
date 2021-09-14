use crate::html::dom::dom::{DOMNode, ElementType, NodeType};

// TODO SelectorElmは正しいのか？, 属性selecytor
#[derive(Debug, PartialEq, Clone, Eq, Hash)]
pub enum SelectorElm {
    Id(String),
    Class(String),
    TagName(String),
    Asterisk(String),
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
    pub fn is_one_node_tree(&self) -> bool {
        self.children.len() == 0
    }

    pub fn pop_root_node_from_tree(&self) -> Vec<Selector> {
        let mut res = vec![];

        for child in self.children.clone() {
            match child {
                SelectorChildren::descendant_combinator(children)
                | SelectorChildren::child_combinator(children)
                | SelectorChildren::general_sibling_combinator(children)
                | SelectorChildren::adjacent_sibling_combinator(children) => {
                    for child in children {
                        res.push(child);
                    }
                }
            };
        }

        res
    }

    pub fn matches(self, elm: &DOMNode, parent_elm: &DOMNode) -> bool {
        let element_type = match &elm.node_type {
            NodeType::text_node(_) => {
                return false;
            }
            NodeType::dom_node(element_type) => element_type,
        };
        if self.children.len() == 0 {
            return match self.elm {
                SelectorElm::Id(id) => {
                    &id == element_type.attributes.get("id").unwrap_or(&"".to_string())
                }
                SelectorElm::Class(class) => {
                    &class
                        == element_type
                            .attributes
                            .get("class")
                            .unwrap_or(&"".to_string())
                }
                SelectorElm::TagName(tag_name) => {
                    tag_name == element_type.clone().tag_name.to_string()
                }
                SelectorElm::Asterisk(_) => true,
            };
        }

        for child in self.children {
            match child {
                SelectorChildren::descendant_combinator(children) => {
                    for descendant in children {
                        for child_elm in elm.clone().children {
                            if descendant.clone().matches(&child_elm, &elm) {
                                return true;
                            } else {
                                for grandchild_elm in child_elm.clone().children {
                                    if descendant.clone().matches(&grandchild_elm, &child_elm) {
                                        return true;
                                    }
                                }
                            }
                        }
                    }
                }
                SelectorChildren::child_combinator(children) => {
                    for child in children {
                        for child_elm in elm.clone().children {
                            if child.clone().matches(&child_elm, &elm) {
                                return true;
                            }
                        }
                    }
                }
                SelectorChildren::general_sibling_combinator(children) => {
                    for general_sibling in children {
                        for sibling_elm in parent_elm.clone().children {
                            if general_sibling.clone().matches(&sibling_elm, &parent_elm) {
                                return true;
                            }
                        }
                    }
                }
                SelectorChildren::adjacent_sibling_combinator(children) => {
                    for adjacent_sibling in children {
                        let elm_idx = parent_elm.children.iter().position(|x| &x == &elm);
                        let elm_idx = match elm_idx {
                            Some(idx) => idx,
                            None => panic!("on adjacent_sibling_combinator matches, elm idx should be existed but none")
                        };
                        let big_brother_sibling_elm = parent_elm.children[elm_idx + 1].clone();
                        let little_brother_sibling_elm = parent_elm.children[elm_idx - 1].clone();
                        if adjacent_sibling
                            .clone()
                            .matches(&big_brother_sibling_elm, &parent_elm)
                            || adjacent_sibling
                                .clone()
                                .matches(&little_brother_sibling_elm, &parent_elm)
                        {
                            return true;
                        }
                    }
                }
            }
        }

        return false;
    }
}
// .class
// .class, .class2
// .class > .class2

#[cfg(test)]
mod test {
    use super::*;
    use crate::html::dom::elements::elements::HTMLElements;
    use std::collections::HashMap;

    #[test]
    fn test_selector_match_simple_node() {
        let selector = Selector {
            elm: SelectorElm::TagName("div".to_string()),
            children: vec![],
        };

        let elm = DOMNode {
            node_type: NodeType::dom_node(ElementType {
                tag_name: HTMLElements::DivElement,
                attributes: HashMap::new(),
            }),
            children: vec![],
        };

        assert!(selector.matches(&elm, &elm));
    }

    #[test]
    fn test_selector_match_nested_nodes_with_descent_combinator() {
        let selector = Selector {
            elm: SelectorElm::TagName("div".to_string()),
            children: vec![SelectorChildren::descendant_combinator(vec![Selector {
                elm: SelectorElm::Id("hoge".to_string()),
                children: vec![],
            }])],
        };

        let mut attributes = HashMap::new();
        attributes.insert("id".to_string(), "hoge".to_string());

        let elm = DOMNode {
            node_type: NodeType::dom_node(ElementType {
                tag_name: HTMLElements::DivElement,
                attributes: HashMap::new(),
            }),
            children: vec![DOMNode {
                node_type: NodeType::dom_node(ElementType {
                    tag_name: HTMLElements::DivElement,
                    attributes: attributes.clone(),
                }),
                children: vec![],
            }],
        };

        assert!(selector.clone().matches(&elm, &elm));

        // * matches with descent combinator
        let elm = DOMNode {
            node_type: NodeType::dom_node(ElementType {
                tag_name: HTMLElements::DivElement,
                attributes: HashMap::new(),
            }),
            children: vec![DOMNode {
                node_type: NodeType::dom_node(ElementType {
                    tag_name: HTMLElements::ParagraphElement,
                    attributes: HashMap::new(),
                }),
                children: vec![DOMNode {
                    node_type: NodeType::dom_node(ElementType {
                        tag_name: HTMLElements::DivElement,
                        attributes,
                    }),
                    children: vec![],
                }],
            }],
        };

        assert!(selector.matches(&elm, &elm));
    }

    #[test]
    fn test_selector_match_nested_nodes_with_child_combinator() {
        let selector = Selector {
            elm: SelectorElm::TagName("div".to_string()),
            children: vec![SelectorChildren::child_combinator(vec![Selector {
                elm: SelectorElm::Id("hoge".to_string()),
                children: vec![],
            }])],
        };
        let mut attributes = HashMap::new();
        attributes.insert("id".to_string(), "hoge".to_string());
        let elm = DOMNode {
            node_type: NodeType::dom_node(ElementType {
                tag_name: HTMLElements::DivElement,
                attributes: HashMap::new(),
            }),
            children: vec![DOMNode {
                node_type: NodeType::dom_node(ElementType {
                    tag_name: HTMLElements::DivElement,
                    attributes: attributes.clone(),
                }),
                children: vec![],
            }],
        };
        assert!(selector.clone().matches(&elm, &elm));

        // * not matches descendant child
        let elm = DOMNode {
            node_type: NodeType::dom_node(ElementType {
                tag_name: HTMLElements::DivElement,
                attributes: HashMap::new(),
            }),
            children: vec![DOMNode {
                node_type: NodeType::dom_node(ElementType {
                    tag_name: HTMLElements::ParagraphElement,
                    attributes: HashMap::new(),
                }),
                children: vec![DOMNode {
                    node_type: NodeType::dom_node(ElementType {
                        tag_name: HTMLElements::DivElement,
                        attributes: attributes.clone(),
                    }),
                    children: vec![],
                }],
            }],
        };
        assert!(!selector.matches(&elm, &elm));
    }
}
