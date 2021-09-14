use crate::css::cssom::cssom::{StylingRule, CSSOM};
use crate::css::cssom::selector::{Selector, SelectorElm};
use crate::css::parser::parser::Parser as CSSParser;
use crate::html::dom::dom::{DOMNode, NodeType};
use crate::html::dom::elements::elements::HTMLElements;
use crate::html::parser::parser::Parser as HTMLParser;
use crate::render_tree::rectangle::Rectangle;
use crate::render_tree::render_object::{RenderObject, _RenderObject};
use std::collections::HashMap;

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

    pub fn prepare_iterator(&self) -> Vec<RenderObject> {
        let mut iterator = vec![];
        self.tree.prepare_iterator(&mut iterator);

        iterator
    }

    pub fn constructor(&mut self) {
        let head = match &self.dom.node_type {
            NodeType::dom_node(element_type) => {
                match element_type.tag_name {
                    // TODO htmlにもstyleが来るので, skipはまずい
                    HTMLElements::HtmlElement => {
                        // TODO　一旦<html>の下には<head>と<body>がこの順序で来ると仮定
                        &self.dom.children[0]
                    }
                    _ => &self.dom,
                }
            }
            NodeType::text_node(text) => &self.dom,
        };
        self.handle_head(&head.clone());

        // TODO dom.rsでやる
        // TODO styled DOM と Rendering Tree で分けた方が良い
        let dom = match &self.dom.node_type {
            NodeType::dom_node(element_type) => {
                match element_type.tag_name {
                    // TODO htmlにもstyleが来るので, skipはまずい
                    HTMLElements::HtmlElement => {
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

        self.layouting(700.0, 700.0);
    }

    pub fn layouting(&mut self, window_with: f32, window_height: f32) {
        let mut root_node = match self.tree {
            RenderObject::ViewPort(ref mut viewport) => viewport,
            _ => panic!("TODO"),
        };

        root_node.rectangle = Rectangle::new(0.0, 0.0, window_with, window_height);
        let mut big_brother = None;
        let parent = self.tree.clone();
        let mut i = 0;

        let mut root_node = match self.tree {
            RenderObject::ViewPort(ref mut viewport) => viewport,
            _ => panic!("TODO"),
        };

        while i < root_node.children.len() {
            let mut child = root_node.children.get_mut(i).unwrap();
            child.layouting_node(parent.clone(), big_brother);
            big_brother = Some(child.clone());
            i += 1;
        }
    }

    fn handle_head(&mut self, head: &DOMNode) {
        for head_el in head.children.clone() {
            println!("head_el: {:?}", head_el);
            match head_el.node_type {
                NodeType::dom_node(element_type) => match element_type.tag_name {
                    HTMLElements::StyleElement => {
                        let style_text = head_el.children[0].clone();
                        let style_text = match style_text.node_type {
                            NodeType::text_node(text) => text,
                            _ => panic!("TODO"),
                        };
                        let mut parser = CSSParser {
                            pos: 0,
                            input: style_text,
                        };
                        let mut cssom = parser.parse();
                        println!("cssom: {:?}", cssom);
                        self.cssom.append(&mut cssom);
                    }
                    _ => (),
                },
                _ => (),
            }
        }
    }

    //  TODO 名前変える
    fn traverse_single_dom(
        &mut self,
        dom_node: DOMNode,
        children_styles: Vec<(Selector, StylingRule)>,
    ) -> RenderObject {
        match dom_node.clone().node_type {
            NodeType::text_node(txt) => RenderObject::init_with_text(txt),
            NodeType::dom_node(element_type) => {
                match element_type.tag_name {
                    HTMLElements::StyleElement => {
                        let css_text = dom_node.clone().children[0].clone();
                        let css_text = match css_text.node_type {
                            NodeType::text_node(txt) => txt,
                            _ => panic!("text in style tag should be raw style"),
                        };

                        let mut css_parser = CSSParser {
                            pos: 0,
                            input: css_text,
                        };

                        let mut cssom = css_parser.parse();

                        self.cssom.append(&mut cssom);
                    }
                    _ => {
                        // do nothing
                    }
                };
                let raw_render_object = RenderObject::init_wiThElement(element_type);
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

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_render_tree() {
        let mut parser = HTMLParser {
            pos: 0,
            input: "<html><head></head><body><p id=\"id1\">hello browser!</p><p id=\"id2\"></p><p id=\"id3\"></p></body></html>".to_string(),
        };

        let dom = parser.parse();

        let mut parser = CSSParser {
            pos: 0,
            input: "#id1 { width: 120; height: 90; background: rgba(255, 0, 0, 1); margin: 0; }#id2 { width: 120; height: 90; background: rgba(0, 255, 0, 1); margin: 0; }#id3 { width: 120; height: 90; background: rgba(0, 0, 255, 1); margin: 0; }".to_string(),
        };

        let cssom = parser.parse();

        let mut render_tree = RenderTree::new(dom, cssom);
        render_tree.constructor();

        let mut base_style = HashMap::new();
        base_style.insert("width".to_string(), "120".to_string());
        base_style.insert("height".to_string(), "90".to_string());
        base_style.insert("margin".to_string(), "0".to_string());

        let mut style_1 = base_style.clone();
        style_1.insert("background".to_string(), "rgba(255, 0, 0, 1)".to_string());

        let mut style_2 = base_style.clone();
        style_2.insert("background".to_string(), "rgba(0, 255, 0, 1)".to_string());

        let mut style_3 = base_style.clone();
        style_3.insert("background".to_string(), "rgba(0, 0, 255, 1)".to_string());

        assert_eq!(
            render_tree.tree,
            RenderObject::ViewPort(_RenderObject {
                rectangle: Rectangle {
                    x: 0.0,
                    y: 0.0,
                    width: 700.0,
                    height: 700.0,
                },
                children: vec![RenderObject::Scroll(_RenderObject {
                    rectangle: Rectangle {
                        x: 0.0,
                        y: 0.0,
                        width: 700.0,
                        height: 270.0,
                    },
                    children: vec![
                        RenderObject::Block(_RenderObject {
                            rectangle: Rectangle {
                                x: 0.0,
                                y: 0.0,
                                width: 120.0,
                                height: 90.0,
                            },
                            children: vec![RenderObject::Text("hello browser!".to_string()),],
                            style: vec![StylingRule {
                                selector: vec![Selector {
                                    elm: SelectorElm::id("id1".to_string()),
                                    children: vec![]
                                }],
                                declarations: style_1
                            }]
                        },),
                        RenderObject::Block(_RenderObject {
                            rectangle: Rectangle {
                                x: 0.0,
                                y: 90.0,
                                width: 120.0,
                                height: 90.0,
                            },
                            children: vec![],
                            style: vec![StylingRule {
                                selector: vec![Selector {
                                    elm: SelectorElm::id("id2".to_string()),
                                    children: vec![]
                                }],
                                declarations: style_2
                            }]
                        },),
                        RenderObject::Block(_RenderObject {
                            rectangle: Rectangle {
                                x: 0.0,
                                y: 180.0,
                                width: 120.0,
                                height: 90.0,
                            },
                            children: vec![],
                            style: vec![StylingRule {
                                selector: vec![Selector {
                                    elm: SelectorElm::id("id3".to_string()),
                                    children: vec![]
                                }],
                                declarations: style_3
                            }]
                        },),
                    ],
                    style: vec![]
                },)],
                style: vec![],
            })
        );
    }
}
