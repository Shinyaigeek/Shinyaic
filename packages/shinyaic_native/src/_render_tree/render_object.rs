use crate::paint::font::PaintFont;
use shinyaic_core::css::cssom::cssom::StylingRule;
use shinyaic_core::html::dom::dom::{DOMNode, ElementType, NodeType};
use shinyaic_core::html::dom::elements::elements::HTMLElements;

#[derive(Debug, PartialEq, Clone)]
pub struct _RenderObject {
    pub children: Vec<RenderObject>,
    pub style: Vec<StylingRule>,
    pub img_href: Option<String>
}

#[derive(Debug, PartialEq, Clone)]
pub struct TextRenderObject {
    pub text: String,
    pub font: PaintFont,
}

#[derive(Debug, PartialEq, Clone)]
pub enum RenderObject {
    ViewPort(_RenderObject),
    Scroll(_RenderObject),
    Block(_RenderObject),
    Inline(_RenderObject),
    Text(TextRenderObject),
}

impl RenderObject {
    pub fn new() -> Self {
        Self::ViewPort(_RenderObject {
            children: vec![],
            style: vec![],
        })
    }

    pub fn layouting_node(
        &mut self,
        parent_node: Self,
        big_brother_node: Option<Self>,
        pad_left: Option<f32>,
        pad_top: Option<f32>,
    ) {
        let big_brother_node = match big_brother_node {
            Some(big_brother_node_) => Some(big_brother_node_),
            None => None,
        };

        let parent = self.clone();

        let styles = match self {
            Self::Text(_) => {
                vec![]
            }
            Self::Block(rendering_object)
            | Self::Inline(rendering_object)
            | Self::Scroll(rendering_object)
            | Self::ViewPort(rendering_object) => rendering_object.style.clone(),
        };

        for style in styles {
            if style.declarations.get(&"padding".to_string()).is_some() {
                let padding = style.declarations.get(&"padding".to_string()).unwrap();
                let padding = padding.split(" ").collect::<Vec<&str>>();

                // TODO
                let (_padding_width, _padding_height) = if padding.len() == 1 {
                    (
                        self.fix_unit_to_px(padding[0].to_string()).unwrap() * 2.0,
                        self.fix_unit_to_px(padding[0].to_string()).unwrap() * 2.0,
                    )
                } else if padding.len() == 2 {
                    (
                        self.fix_unit_to_px(padding[1].to_string()).unwrap() * 2.0,
                        self.fix_unit_to_px(padding[0].to_string()).unwrap() * 2.0,
                    )
                } else {
                    (
                        self.fix_unit_to_px(padding[1].to_string()).unwrap()
                            + self.fix_unit_to_px(padding[3].to_string()).unwrap(),
                        self.fix_unit_to_px(padding[0].to_string()).unwrap()
                            + self.fix_unit_to_px(padding[2].to_string()).unwrap(),
                    )
                };

                paddinged_height = _padding_height;
                paddinged_width = _padding_width;
            }
        }
    }

    pub fn init_with_text(
        txt: String,
        font: Option<PaintFont>,
    ) -> Self {

        let font = font.unwrap_or(PaintFont::new(None, None));

        Self::Text(TextRenderObject {
            text: txt,
            font,
        })
    }

    pub fn init_with_element(
        element_type: ElementType,
    ) -> Option<Self> {
        match element_type.tag_name {
            HTMLElements::BodyElement => Some(Self::Scroll(_RenderObject {
                children: vec![],
                style: vec![],
            })),
            HTMLElements::DivElement | HTMLElements::ParagraphElement | HTMLElements::H1Element => {
                Some(Self::Block(_RenderObject {
                    children: vec![],
                    style: vec![],
                }))
            }
            HTMLElements::AnchorElement | HTMLElements::SpanElement | HTMLElements::ImgElement => {
                Some(Self::Inline(_RenderObject {
                    children: vec![],
                    style: vec![],
                }))
            }
            _ => None,
        }
    }

    pub fn can_init_element(dom_node: &DOMNode) -> bool {
        let element_type = match &dom_node.node_type {
            NodeType::TextNode(_) => return false,
            NodeType::DomNode(element_type) => element_type,
        };
        let tag = &element_type.tag_name;
        tag == &HTMLElements::BodyElement
            || tag == &HTMLElements::DivElement
            || tag == &HTMLElements::ParagraphElement
            || tag == &HTMLElements::AnchorElement
            || tag == &HTMLElements::SpanElement
            || tag == &HTMLElements::H1Element
            || tag == &HTMLElements::ImgElement
    }

    pub fn can_init_text(dom_node: &DOMNode) -> bool {
        match &dom_node.node_type {
            NodeType::TextNode(_) => true,
            NodeType::DomNode(_) => false,
        }
    }

    pub fn push_child(&mut self, child: RenderObject) {
        match self {
            Self::Text(_) => {
                panic!("RenderObject::push_shild should not be called with text")
            }
            Self::ViewPort(render_object)
            | Self::Scroll(render_object)
            | Self::Inline(render_object)
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
            | Self::Inline(render_object)
            | Self::Block(render_object) => render_object.style = rules,
        };
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fix_unit_to_px() {
        let rendering_object = RenderObject::Block(_RenderObject {
            children: vec![],
            style: vec![],
        });

        assert_eq!(
            rendering_object.fix_unit_to_px("10px".to_string()),
            Some(10.0)
        );
    }

    #[test]
    fn test_fix_unit_to_px_without_px() {
        let rendering_object = RenderObject::Block(_RenderObject {
            children: vec![],
            style: vec![],
        });
        assert_eq!(
            rendering_object.fix_unit_to_px("10".to_string()),
            Some(10.0)
        );
    }
}
