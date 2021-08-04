use crate::css::cssom::cssom::{StylingRule, CSSOM};
use crate::css::cssom::selector::Selector;
use crate::html::dom::dom::{DOMNode, NodeType};
use crate::html::dom::elements::elements::HTMLElements;
use crate::render_tree::render_object::RenderObject;

pub struct RenderTree {
    dom: DOMNode,
    cssom: CSSOM,
    pub tree: RenderObject,
}

impl RenderTree {
    pub fn new(dom: DOMNode, cssom: CSSOM) -> Self {
        Self {
            dom,
            cssom,
            tree: RenderObject::new(),
        }
    }

    pub fn constructor(&mut self) {
        // TODO dom.rsでやる
        // TODO styled DOM と Rendering Tree で分けた方が良い
        let dom = match &self.dom.node_type {
            NodeType::dom_node(element_type) => {
                match element_type.tag_name {
                    // TODO htmlにもstyleが来るので, skipはまずい
                    HTMLElements::HTML_ELEMENT => {
                        // TODO　一旦<html>の下には<head>と<body>がこの順序で来ると仮定
                        &self.dom.children[1]
                    }
                    _ => &self.dom,
                }
            }
            NodeType::text_node(text) => &self.dom,
        };
        let render_tree_under_viewport = self.traverse_single_dom(dom.clone(), vec![]);

        self.tree.push_child(render_tree_under_viewport);
    }

    pub fn layouting(&mut self, window_with: u32, window_height: u32) {}

    //  TODO 名前変える
    fn traverse_single_dom(
        &self,
        dom_node: DOMNode,
        children_styles: Vec<(Selector, StylingRule)>,
    ) -> RenderObject {
        match dom_node.clone().node_type {
            NodeType::text_node(txt) => RenderObject::init_with_text(txt),
            NodeType::dom_node(element_type) => {
                let mut raw_render_object = RenderObject::init_with_element(element_type);
                let mut raw_render_object = match raw_render_object {
                    Some(raw_render_object) => raw_render_object,
                    //  TODO
                    None => {
                        panic!("traverse_single_dom")
                    }
                };

                let mut style = vec![];

                let mut passed_children_styles = vec![];

                for child_style_rule in children_styles {
                    let selector = child_style_rule.0;
                    let styling_rule = child_style_rule.1;

                    if (selector.clone().matches(&dom_node, &dom_node)) {
                        if selector.is_one_node_tree() {
                            style.push(styling_rule);
                        } else {
                            let popped_selectors = selector.pop_root_node_from_tree();
                            for popped_selector in popped_selectors {
                                passed_children_styles
                                    .push((popped_selector, styling_rule.clone()));
                            }
                        }
                    }
                }

                for style_rule in &self.cssom {
                    // TODO これだと body p {} は p ではなく <body><p /></body> な body にマッチしてしまう
                    // TODO: FIXME 第二引数の parent_elm に自Nodeを渡してしまっているが, css selector tree の root node の兄弟要素を結合子で指定し得ないので動いている, がミスリーディングなコードである
                    if style_rule.clone().matches(&dom_node, &dom_node) {
                        let matched_styling_rule_selectors = style_rule.clone().selector;
                        for matched_styling_rule_selector in matched_styling_rule_selectors {
                            if matched_styling_rule_selector.clone().is_one_node_tree() {
                                style.push(style_rule.clone());
                            } else {
                                let popped_selectors = matched_styling_rule_selector
                                    .clone()
                                    .pop_root_node_from_tree();
                                for popped_selector in popped_selectors {
                                    passed_children_styles
                                        .push((popped_selector, style_rule.clone()));
                                }
                            }
                        }
                    }
                }

                raw_render_object.replace_style(style);

                // TODO 後で消す
                if dom_node.children.len() == 0 {
                    raw_render_object
                } else {
                    for child in dom_node.children {
                        if RenderObject::can_init_element(&child) {
                            raw_render_object.push_child(
                                self.traverse_single_dom(child, passed_children_styles.clone()),
                            )
                        } else if RenderObject::can_init_text(&child) {
                            raw_render_object.push_child(
                                self.traverse_single_dom(child, passed_children_styles.clone()),
                            )
                        }
                    }

                    raw_render_object
                }
            }
        }
    }
}
