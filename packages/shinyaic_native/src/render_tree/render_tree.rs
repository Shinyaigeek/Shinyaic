use crate::paint::font::PaintFont;
use crate::render_tree::rectangle::Rectangle;
use crate::render_tree::render_object::RenderObject;
use crate::render_tree::window_size::WindowSize;
use shinyaic_core::css::cssom::cssom::{StylingRule, CSSOM};
use shinyaic_core::css::cssom::selector::Selector;
use shinyaic_core::css::parser::parser::Parser as CSSParser;
use shinyaic_core::html::dom::dom::{DOMNode, NodeType};
use shinyaic_core::html::dom::elements::elements::HTMLElements;

#[derive(Debug, PartialEq, Clone)]
pub struct RenderTree {
    dom: DOMNode,
    cssom: CSSOM,
    pub tree: RenderObject,
    pub window_size: WindowSize,
}

impl RenderTree {
    pub fn new(dom: DOMNode, cssom: CSSOM) -> Self {
        Self {
            dom,
            cssom,
            tree: RenderObject::new(),
            window_size: WindowSize::new(900.0, 700.0),
        }
    }

    pub fn prepare_iterator(&self) -> Vec<RenderObject> {
        let mut iterator = vec![];
        self.tree.prepare_iterator(&mut iterator);

        iterator
    }

    pub fn constructor(&mut self) {
        let head = match &self.dom.node_type {
            NodeType::DomNode(element_type) => {
                match element_type.tag_name {
                    // TODO htmlにもstyleが来るので, skipはまずい
                    HTMLElements::HtmlElement => {
                        // TODO　一旦<html>の下には<head>と<body>がこの順序で来ると仮定
                        self.dom.children[0].clone()
                    }
                    _ => self.dom.clone(),
                }
            }
            NodeType::TextNode(_) => self.dom.clone(),
        };

        self.handle_head(&head);

        // TODO dom.rsでやる
        // TODO styled DOM と Rendering Tree で分けた方が良い
        let dom = match &self.dom.node_type {
            NodeType::DomNode(element_type) => {
                match element_type.tag_name {
                    // TODO htmlにもstyleが来るので, skipはまずい
                    HTMLElements::HtmlElement => {
                        // TODO　一旦<html>の下には<head>と<body>がこの順序で来ると仮定
                        self.dom.children[1].clone()
                    }
                    _ => self.dom.clone(),
                }
            }
            NodeType::TextNode(_) => self.dom.clone(),
        };
        let render_tree_under_viewport = self.traverse_single_dom(dom, vec![], None);

        self.tree.push_child(render_tree_under_viewport);

        self.layouting(self.window_size.width, self.window_size.height);
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

        let root_node = match self.tree {
            RenderObject::ViewPort(ref mut viewport) => viewport,
            _ => panic!("TODO"),
        };

        while i < root_node.children.len() {
            let child = root_node.children.get_mut(i).unwrap();
            child.layouting_node(parent.clone(), big_brother, None, None);
            big_brother = Some(child.clone());
            i += 1;
        }
    }

    fn handle_head(&mut self, head: &DOMNode) {
        for head_el in head.children.clone() {
            match head_el.node_type {
                NodeType::DomNode(element_type) => match element_type.tag_name {
                    HTMLElements::StyleElement => {
                        let style_text = head_el.children[0].clone();
                        let style_text = match style_text.node_type {
                            NodeType::TextNode(text) => text,
                            _ => panic!("TODO"),
                        };
                        let mut parser = CSSParser {
                            pos: 0,
                            input: style_text,
                        };
                        let mut cssom = parser.parse();
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
        font: Option<PaintFont>,
    ) -> RenderObject {
        match dom_node.clone().node_type {
            NodeType::TextNode(txt) => RenderObject::init_with_text(txt, None, font),
            NodeType::DomNode(element_type) => {
                match element_type.tag_name {
                    HTMLElements::StyleElement => {
                        let css_text = dom_node.clone().children[0].clone();
                        let css_text = match css_text.node_type {
                            NodeType::TextNode(txt) => txt,
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

                let raw_render_object = RenderObject::init_with_element(
                    element_type,
                    self.window_size.width,
                    self.window_size.height,
                );
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

                    if selector.clone().matches(&dom_node, &dom_node) {
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

                let current_font = match raw_render_object.clone() {
                    RenderObject::Text(_) => font,
                    RenderObject::ViewPort(render_object)
                    | RenderObject::Scroll(render_object)
                    | RenderObject::Block(render_object)
                    | RenderObject::Inline(render_object) => {
                        let styles = render_object.style;
                        let mut current_font = Option::<PaintFont>::None;
                        for style in styles {
                            if style.declarations.get("font-size").is_some() {
                                let font_size = style.declarations.get("font-size").unwrap();
                                let font_size = raw_render_object.fix_unit_to_px(font_size.clone());
                                current_font = Some(PaintFont::new(None, font_size));
                            }
                        }

                        current_font
                    }
                };

                // TODO 後で消す
                if dom_node.children.len() == 0 {
                    raw_render_object
                } else {
                    for child in dom_node.children {
                        if RenderObject::can_init_element(&child) {
                            raw_render_object.push_child(self.traverse_single_dom(
                                child,
                                passed_children_styles.clone(),
                                current_font.clone(),
                            ))
                        } else if RenderObject::can_init_text(&child) {
                            raw_render_object.push_child(self.traverse_single_dom(
                                child,
                                passed_children_styles.clone(),
                                current_font.clone(),
                            ))
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
    use crate::render_tree::render_object::_RenderObject;
    use shinyaic_core::css::cssom::selector::SelectorElm;
    use shinyaic_core::html::parser::parser::Parser as HTMLParser;
    use std::collections::HashMap;

    #[test]
    fn test_render_tree() {
        let mut parser = HTMLParser::new("<html><head></head><body><p id=\"id1\">hello browser!</p><p id=\"id2\"></p><p id=\"id3\"></p></body></html>".to_string());

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
                    width: 900.0,
                    height: 700.0,
                },
                img_href: None,
                children: vec![RenderObject::Scroll(_RenderObject {
                    rectangle: Rectangle {
                        x: 0.0,
                        y: 0.0,
                        width: 900.0,
                        height: 270.0,
                    },
                    img_href: None,
                    children: vec![
                        RenderObject::Block(_RenderObject {
                            rectangle: Rectangle {
                                x: 0.0,
                                y: 0.0,
                                width: 120.0,
                                height: 90.0,
                            },
                            img_href: None,
                            children: vec![RenderObject::init_with_text(
                                "hello browser!".to_string(),
                                Some(Rectangle {
                                    x: 0.0,
                                    y: 0.0,
                                    width: 120.0,
                                    height: 32.0,
                                }),
                                None
                            ),],
                            style: vec![StylingRule {
                                selector: vec![Selector {
                                    elm: SelectorElm::Id("id1".to_string()),
                                    children: vec![],

                                    pseudo_elements: None
                                }],
                                declarations: style_1
                            }],
                            window_size: WindowSize::new(900.0, 700.0)
                        },),
                        RenderObject::Block(_RenderObject {
                            rectangle: Rectangle {
                                x: 0.0,
                                y: 90.0,
                                width: 120.0,
                                height: 90.0,
                            },
                            img_href: None,
                            children: vec![],
                            style: vec![StylingRule {
                                selector: vec![Selector {
                                    elm: SelectorElm::Id("id2".to_string()),
                                    children: vec![],
                                    pseudo_elements: None
                                }],
                                declarations: style_2
                            }],
                            window_size: WindowSize::new(900.0, 700.0)
                        },),
                        RenderObject::Block(_RenderObject {
                            rectangle: Rectangle {
                                x: 0.0,
                                y: 180.0,
                                width: 120.0,
                                height: 90.0,
                            },
                            img_href: None,
                            children: vec![],
                            style: vec![StylingRule {
                                selector: vec![Selector {
                                    elm: SelectorElm::Id("id3".to_string()),
                                    children: vec![],
                                    pseudo_elements: None
                                }],
                                declarations: style_3
                            }],
                            window_size: WindowSize::new(900.0, 700.0)
                        },),
                    ],
                    style: vec![],
                    window_size: WindowSize::new(900.0, 700.0)
                },)],
                style: vec![],
                window_size: WindowSize::new(0.0, 0.0)
            })
        );
    }
}
