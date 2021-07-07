use crate::html::dom::dom::{DOMNode, ElementType, NodeType};
use crate::html::dom::elements::elements::HTMLElements;

#[derive(Debug, PartialEq, Clone)]
pub struct _RenderObject {
    pub children: Vec<RenderObject>,
}

#[derive(Debug, PartialEq, Clone)]
pub enum RenderObject {
    ViewPort(_RenderObject),
    Scroll(_RenderObject),
    Block(_RenderObject),
    Text(String),
}

impl RenderObject {
    pub fn new() -> Self {
        Self::ViewPort(_RenderObject { children: vec![] })
    }

    pub fn init_with_text(txt: String) -> Self {
        Self::Text(txt)
    }

    pub fn init_with_element(element_type: ElementType) -> Option<Self> {
        match element_type.tag_name {
            HTMLElements::BODY_ELEMENT => Some(Self::Scroll(_RenderObject { children: vec![] })),
            HTMLElements::DIV_ELEMENT | HTMLElements::PARAGRAPH_ELEMENT => {
                Some(Self::Block(_RenderObject { children: vec![] }))
            }
            _ => None,
        }
    }

    pub fn can_init_element(dom_node: &DOMNode) -> bool {
        let element_type = match &dom_node.node_type {
            NodeType::text_node(_) => return false,
            NodeType::dom_node(element_type) => element_type,
        };
        let tag = &element_type.tag_name;
        tag == &HTMLElements::BODY_ELEMENT || tag == &HTMLElements::DIV_ELEMENT || tag == &HTMLElements::PARAGRAPH_ELEMENT
    }

    pub fn change_kind(&mut self, target: &str) -> Self {
        let children = match self {
            Self::Text(_) => {
                panic!("RenderObject::change_kind should not be called with text")
            }
            Self::ViewPort(render_object)
            | Self::Scroll(render_object)
            | Self::Block(render_object) => render_object.children.clone(),
        };
        match target {
            "view_port" => Self::ViewPort(_RenderObject { children }),
            "scroll" => Self::Scroll(_RenderObject { children }),
            "block" => Self::Block(_RenderObject { children }),
            _ => {
                panic!("RenderObject::change_kind should be viewport or scroll or block")
            }
        }
    }

    pub fn push_child(&mut self, child: RenderObject) {
        match self {
            Self::Text(_) => {
                panic!("RenderObject::push_shild should not be called with text")
            }
            Self::ViewPort(render_object)
            | Self::Scroll(render_object)
            | Self::Block(render_object) => render_object.children.push(child),
        };
    }
}
