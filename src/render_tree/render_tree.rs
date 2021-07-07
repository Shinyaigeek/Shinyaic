use crate::css::cssom::cssom::CSSOM;
use crate::html::dom::dom::{DOMNode, NodeType};
use crate::html::dom::elements::elements::HTMLElements;
use crate::render_tree::render_object::RenderObject;

pub struct RenderTree {
    dom: DOMNode,
    cssom: CSSOM,
    tree: RenderObject,
}

impl RenderTree {
    pub fn new(dom: DOMNode, cssom: CSSOM) -> Self {
        Self {
            dom,
            cssom,
            tree: RenderObject::new(),
        }
    }

    pub fn constructor(&self) -> RenderObject {
        // TODO dom.rsでやる
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
        let render_tree = RenderTree::traverse_single_dom(dom.clone());

        render_tree
    }

    //  TODO 名前変える
    fn traverse_single_dom(dom_node: DOMNode) -> RenderObject {
        match dom_node.node_type {
            NodeType::text_node(txt) => RenderObject::init_with_text(txt),
            NodeType::dom_node(element_type) => {
                println!("element: {:?}", element_type);
                let mut raw_render_object = RenderObject::init_with_element(element_type);
                let mut raw_render_object = match raw_render_object {
                    Some(raw_render_object) => raw_render_object,
                    //  TODO
                    None => {
                        panic!("traverse_single_dom")
                    }
                };
                // TODO 後で消す
                if dom_node.children.len() == 0 {
                    raw_render_object
                } else {
                    for child in dom_node.children {
                        if RenderObject::can_init_element(&child) {
                            raw_render_object.push_child(RenderTree::traverse_single_dom(child))
                        }
                    }

                    raw_render_object
                }
            }
        }
    }
}
