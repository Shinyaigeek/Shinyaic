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
    pub fn matches(self, elm: &DOMNode, parent_elm: &DOMNode) -> bool {
        let element_type = match elm.node_type {
            NodeType::text_node(text_node) => {
                return false;
            }
            NodeType::dom_node(element_type) => element_type,
        };
        if self.children.len() == 0 {
            return match self.elm {
                SelectorElm::id(id) => {
                    &id == element_type.attributes.get("id").unwrap_or(&"".to_string())
                }
                SelectorElm::class(class) => {
                    &class
                        == element_type
                            .attributes
                            .get("class")
                            .unwrap_or(&"".to_string())
                }
                SelectorElm::tag_name(tag_name) => tag_name == element_type.tag_name.to_string(),
                SelectorElm::asterisk(asterisk) => true,
            };
        }

        for child in self.children {
            match child {
                SelectorChildren::descendant_combinator(children) => {
                    for descendant in children {
                        for child_elm in elm.children {
                            if descendant.matches(&child_elm, &elm) {
                                return true;
                            } else {
                                for grandchild_elm in child_elm.children {
                                    if descendant.matches(&grandchild_elm, &child_elm) {
                                        return true;
                                    }
                                }
                            }
                        }
                    }
                }
                SelectorChildren::child_combinator(children) => {
                    for child in children {
                        for child_elm in elm.children {
                            if child.matches(&child_elm, &elm) {
                                return true;
                            }
                        }
                    }
                }
                SelectorChildren::general_sibling_combinator(children) => {
                    for general_sibling in children {
                        for sibling_elm in parent_elm.children {
                            if general_sibling.matches(&sibling_elm, &parent_elm) {
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
                        let big_brother_sibling_elm = parent_elm.children[elm_idx + 1];
                        let little_brother_sibling_elm = parent_elm.children[elm_idx - 1];
                        if adjacent_sibling.matches(&big_brother_sibling_elm, &parent_elm)
                            || adjacent_sibling.matches(&little_brother_sibling_elm, &parent_elm)
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
