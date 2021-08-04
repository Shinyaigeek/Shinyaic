use crate::css::cssom::cssom::StylingRule;
use crate::html::dom::dom::{DOMNode, ElementType, NodeType};
use crate::html::dom::elements::elements::HTMLElements;
use crate::render_tree::rectangle::Rectangle;

#[derive(Debug, PartialEq, Clone)]
pub struct _RenderObject {
    pub children: Vec<RenderObject>,
    pub style: Vec<StylingRule>,
    pub rectangle: Rectangle,
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
        Self::ViewPort(_RenderObject {
            children: vec![],
            style: vec![],
            rectangle: Rectangle::new(0.0, 0.0, 0.0, 0.0),
        })
    }

    // TODO position absoluteの時など, big_brotherがparentに入らなさそうな時
    // TODO font size == 高さと見做してするけど, のちになんとかした方が良さそう
    pub fn calc_rectangle(&mut self, parent_rect: &Rectangle, big_brother_rect: &Rectangle) {
        let rendering_object = match self {
            Self::Text(text) => {
                return;
            }
            Self::Block(rendering_object)
            | Self::Scroll(rendering_object)
            | Self::ViewPort(rendering_object) => rendering_object,
        };
    }

    fn calc_width(&self, parent_width: &f32) -> f32 {
        let rendering_object = match self {
            // TODO
            Self::Text(text) => {
                return 0.0;
            }
            Self::Block(rendering_object)
            | Self::Scroll(rendering_object)
            | Self::ViewPort(rendering_object) => rendering_object,
        };

        for style in rendering_object.clone().style {
            if style.declarations.get(&"width".to_string()).is_some() {
                let raw_width = style
                    .declarations
                    .get(&"width".to_string())
                    .unwrap()
                    .parse::<f32>();

                match raw_width {
                    Ok(width) => {
                        return width;
                    }
                    Err(e) => {
                        panic!(e);
                    }
                }
            }
        }

        let mut width = 0.0;

        for child in rendering_object.clone().children {
            width += child.calc_width(&rendering_object.rectangle.width);
        }

        width
    }

    pub fn init_with_text(txt: String) -> Self {
        Self::Text(txt)
    }

    pub fn init_with_element(element_type: ElementType) -> Option<Self> {
        match element_type.tag_name {
            HTMLElements::BODY_ELEMENT => Some(Self::Scroll(_RenderObject {
                children: vec![],
                style: vec![],
                rectangle: Rectangle::new(0.0, 0.0, 0.0, 0.0),
            })),
            HTMLElements::DIV_ELEMENT | HTMLElements::PARAGRAPH_ELEMENT => {
                Some(Self::Block(_RenderObject {
                    children: vec![],
                    style: vec![],
                    rectangle: Rectangle::new(0.0, 0.0, 0.0, 0.0),
                }))
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
        tag == &HTMLElements::BODY_ELEMENT
            || tag == &HTMLElements::DIV_ELEMENT
            || tag == &HTMLElements::PARAGRAPH_ELEMENT
    }

    pub fn can_init_text(dom_node: &DOMNode) -> bool {
        match &dom_node.node_type {
            NodeType::text_node(_) => true,
            NodeType::dom_node(element_type) => false,
        }
    }

    pub fn change_kind(&mut self, target: &str) -> Self {
        let (children, style, rectangle) = match self {
            Self::Text(_) => {
                panic!("RenderObject::change_kind should not be called with text")
            }
            Self::ViewPort(render_object)
            | Self::Scroll(render_object)
            | Self::Block(render_object) => (
                render_object.children.clone(),
                render_object.style.clone(),
                render_object.rectangle.clone(),
            ),
        };
        match target {
            "view_port" => Self::ViewPort(_RenderObject {
                children,
                style,
                rectangle,
            }),
            "scroll" => Self::Scroll(_RenderObject {
                children,
                style,
                rectangle,
            }),
            "block" => Self::Block(_RenderObject {
                children,
                style,
                rectangle,
            }),
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

    pub fn replace_style(&mut self, rules: Vec<StylingRule>) {
        match self {
            Self::Text(_) => {
                panic!("RenderObject::replace_style should not be called with text")
            }
            Self::ViewPort(render_object)
            | Self::Scroll(render_object)
            | Self::Block(render_object) => render_object.style = rules,
        };
    }
}
