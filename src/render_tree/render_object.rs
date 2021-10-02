use crate::css::cssom::cssom::StylingRule;
use crate::html::dom::dom::{DOMNode, ElementType, NodeType};
use crate::html::dom::elements::elements::HTMLElements;
use crate::paint::font::PaintFont;
use crate::render_tree::pt::fix_unit_to_px;
use crate::render_tree::rectangle::Rectangle;
use crate::render_tree::window_size::WindowSize;

#[derive(Debug, PartialEq, Clone)]
pub struct _RenderObject {
    pub children: Vec<RenderObject>,
    pub style: Vec<StylingRule>,
    pub rectangle: Rectangle,
    pub window_size: WindowSize,
}

#[derive(Debug, PartialEq, Clone)]
pub struct TextRenderObject {
    pub text: String,
    pub rectangle: Rectangle,
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
            rectangle: Rectangle::new(0.0, 0.0, 0.0, 0.0),
            window_size: WindowSize::new(0.0, 0.0),
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

        let parent_rectangle = match parent_node.clone() {
            Self::Text(_) => panic!("TODO"),
            Self::Scroll(parent_node)
            | Self::ViewPort(parent_node)
            | Self::Block(parent_node)
            | Self::Inline(parent_node) => parent_node.rectangle,
        };

        let big_brother_rectangle = match big_brother_node.clone() {
            None => None,
            Some(big_brother_node_) => match big_brother_node_ {
                Self::Text(_) => panic!("TODO"),
                Self::Scroll(big_brother)
                | Self::ViewPort(big_brother)
                | Self::Inline(big_brother)
                | Self::Block(big_brother) => Some(big_brother.rectangle),
            },
        };

        self.calc_rectangle(
            &parent_rectangle,
            &big_brother_rectangle,
            pad_left,
            if big_brother_node.is_none() {
                pad_top
            } else {
                None
            },
        );

        let parent = self.clone();

        let mut paddinged_width = 0.0;
        let mut paddinged_height = 0.0;

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

                // TODO
                let padding = fix_unit_to_px(padding.to_string());

                match padding {
                    Some(_padding) => {
                        paddinged_width = _padding.clone();
                        paddinged_height = _padding.clone();
                    }
                    None => {
                        panic!("TODO");
                    }
                }
            }
        }

        match self {
            Self::Text(_) => return,
            Self::Block(rendering_object)
            | Self::Inline(rendering_object)
            | Self::Scroll(rendering_object)
            | Self::ViewPort(rendering_object) => {
                let mut big_brother_node: Option<Self> = None;

                let mut i = 0;

                let children_length = rendering_object.children.len();

                while i < children_length {
                    let child = rendering_object.children.get_mut(i).unwrap();

                    child.layouting_node(
                        parent.clone(),
                        big_brother_node.clone(),
                        Some(paddinged_width),
                        Some(paddinged_height),
                    );
                    println!("child: {:?}", child);
                    println!("---------");
                    big_brother_node = Some(child.clone());
                    i += 1;
                }
            }
        }
    }

    pub fn prepare_iterator(&self, iterator: &mut Vec<Self>) {
        iterator.push(self.clone());

        let rendering_object = match self {
            Self::Text(_) => {
                return;
            }
            Self::Block(rendering_object)
            | Self::Inline(rendering_object)
            | Self::Scroll(rendering_object)
            | Self::ViewPort(rendering_object) => rendering_object,
        };

        if rendering_object.children.len() > 0 {
            for child in &rendering_object.children {
                child.prepare_iterator(iterator);
            }
        }
    }

    // TODO position absoluteの時など, big_brotherがparentに入らなさそうな時
    // TODO font size == 高さと見做してするけど, のちになんとかした方が良さそう
    pub fn calc_rectangle(
        &mut self,
        parent_rect: &Rectangle,
        big_brother_rect: &Option<Rectangle>,
        pad_left: Option<f32>,
        pad_top: Option<f32>,
    ) {
        println!("rect: {:#?}", parent_rect);
        let width = self.calc_width(&(parent_rect.width - pad_left.unwrap_or(0.0) * 2.0));
        let height = self.calc_height(&parent_rect.height, &width);

        let rendering_object = match self {
            Self::Text(text_render_object) => {
                text_render_object.rectangle =
                    Rectangle::new(parent_rect.x, parent_rect.y, width, height);
                return;
            }
            Self::Block(rendering_object)
            | Self::Inline(rendering_object)
            | Self::Scroll(rendering_object)
            | Self::ViewPort(rendering_object) => rendering_object,
        };

        // TODO
        rendering_object.rectangle = Rectangle::new(0.0, 0.0, width, height);

        let mut margined_top = 0.0;
        let mut margined_left = 0.0;

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
            if style.declarations.get(&"margin".to_string()).is_some() {
                let margin = style.declarations.get(&"margin".to_string()).unwrap();
                let margin = margin.split(" ").collect::<Vec<&str>>();

                if margin.len() == 1 {
                    let margin = margin[0];
                    // TODO
                    let margin = fix_unit_to_px(margin.to_string());

                    match margin {
                        Some(_margin) => {
                            margined_top = _margin.clone();
                            margined_left = _margin.clone();
                        }
                        None => {
                            panic!("TODO");
                        }
                    }
                } else if margin.len() == 2 {
                    let margin_vertical = margin[0];
                    let margin_horizontal = margin[1];
                    // TODO
                    let margin_vertical = if margin_vertical == "auto" {
                        let parent_node_height = parent_rect.height;

                        let self_node_height = match self {
                            Self::Text(_) => panic!("TODO"),
                            Self::Scroll(self_node)
                            | Self::ViewPort(self_node)
                            | Self::Block(self_node)
                            | Self::Inline(self_node) => self_node.rectangle.height,
                        };

                        Some((parent_node_height - self_node_height) / 2.0)
                    } else {
                        fix_unit_to_px(margin_vertical.to_string())
                    };
                    let margin_horizontal = if margin_horizontal == "auto" {
                        let parent_node_width = parent_rect.width;

                        let self_node_width = match self {
                            Self::Text(_) => panic!("TODO"),
                            Self::Scroll(self_node)
                            | Self::ViewPort(self_node)
                            | Self::Block(self_node)
                            | Self::Inline(self_node) => self_node.rectangle.width,
                        };

                        Some((parent_node_width - self_node_width) / 2.0)
                    } else {
                        fix_unit_to_px(margin_horizontal.to_string())
                    };

                    margined_top = margin_vertical.unwrap_or(0.0);
                    margined_left = margin_horizontal.unwrap_or(0.0);
                } else {
                    panic!("TODO");
                }
            }
        }

        let x = self.calc_x(
            &parent_rect,
            Some(pad_left.unwrap_or(0.0) + margined_left),
            &big_brother_rect,
        );
        let y = self.calc_y(
            &parent_rect,
            Some(pad_top.unwrap_or(0.0) + margined_top),
            &big_brother_rect,
        );

        let rendering_object = match self {
            Self::Text(_) => {
                return;
            }
            Self::Block(rendering_object)
            | Self::Inline(rendering_object)
            | Self::Scroll(rendering_object)
            | Self::ViewPort(rendering_object) => rendering_object,
        };

        rendering_object.rectangle = Rectangle::new(x, y, width, height);
    }

    fn calc_width(&self, parent_width: &f32) -> f32 {
        let rendering_object = match self {
            // TODO
            Self::Text(_) => {
                return parent_width.clone();
            }
            Self::Block(rendering_object)
            | Self::Inline(rendering_object)
            | Self::Scroll(rendering_object)
            | Self::ViewPort(rendering_object) => rendering_object,
        };

        let mut width = parent_width.clone();
        let mut paddinged_width = 0.0;

        for style in rendering_object.clone().style {
            if style.declarations.get(&"width".to_string()).is_some() {
                let raw_width = style.declarations.get(&"width".to_string()).unwrap();

                let raw_width = fix_unit_to_px(raw_width.to_string());

                match raw_width {
                    Some(_width) => {
                        width = _width;
                    }
                    None => {
                        panic!("TODO");
                    }
                }
            }

            if style.declarations.get(&"padding".to_string()).is_some() {
                let padding = style.declarations.get(&"padding".to_string()).unwrap();

                // TODO
                let padding = fix_unit_to_px(padding.to_string());

                match padding {
                    Some(_padding) => {
                        paddinged_width = _padding;
                    }
                    None => {
                        panic!("TODO");
                    }
                }
            }
        }

        let width = match self {
            // TODO
            Self::Text(_) => {
                return 0.0;
            }
            Self::Block(_) | Self::Inline(_) | Self::Scroll(_) | Self::ViewPort(_) => {
                width + paddinged_width * 2.0
            }
        };

        width.clone()
    }

    fn calc_height(&self, _parent_height: &f32, parent_width: &f32) -> f32 {
        let rendering_object = match self {
            // TODO
            Self::Text(text) => {
                return text
                    .font
                    .get_font_rendered_size(parent_width.clone(), text.text.clone())
                    .height as f32
            }
            Self::Block(rendering_object)
            | Self::Inline(rendering_object)
            | Self::Scroll(rendering_object)
            | Self::ViewPort(rendering_object) => rendering_object,
        };

        let mut height = Option::<f32>::None;
        let mut paddinged_height = 0.0;

        for style in rendering_object.clone().style {
            if style.declarations.get(&"height".to_string()).is_some() {
                let raw_height = style
                    .declarations
                    .get(&"height".to_string())
                    .unwrap()
                    .parse::<f32>();

                match raw_height {
                    Ok(_height) => {
                        height = Some(_height);
                    }
                    Err(e) => {
                        panic!("{:?}", e);
                    }
                }
            }

            if style.declarations.get(&"padding".to_string()).is_some() {
                let padding = style.declarations.get(&"padding".to_string()).unwrap();

                // TODO
                let padding = fix_unit_to_px(padding.to_string());

                match padding {
                    Some(_padding) => {
                        paddinged_height = _padding;
                    }
                    None => {
                        panic!("TODO");
                    }
                }
            }
        }

        if height.is_some() {
            return height.unwrap() + paddinged_height * 2.0;
        }

        let height = match self {
            Self::Text(_) => {
                return 0.0;
            }
            Self::Block(rendering_object)
            | Self::Inline(rendering_object)
            | Self::Scroll(rendering_object)
            | Self::ViewPort(rendering_object) => {
                let mut height = 0.0;
                for child in rendering_object.clone().children {
                    height += child.calc_height(&rendering_object.rectangle.height, &parent_width);
                }
                height
            }
        };

        height + paddinged_height * 2.0
    }

    fn calc_x(
        &self,
        parent_rect: &Rectangle,
        pad_left: Option<f32>,
        _big_brother_rect: &Option<Rectangle>,
    ) -> f32 {
        let x = match self {
            // TODO
            Self::Text(_) => parent_rect.x,
            Self::Block(_) | Self::Inline(_) | Self::Scroll(_) | Self::ViewPort(_) => parent_rect.x,
        };

        x + pad_left.unwrap_or(0.0)
    }

    fn calc_y(
        &self,
        parent_rect: &Rectangle,
        pad_top: Option<f32>,
        big_brother_rect: &Option<Rectangle>,
    ) -> f32 {
        let big_brother_rect = match big_brother_rect {
            Some(big_brother_rect) => big_brother_rect,
            None => {
                return parent_rect.y + pad_top.unwrap_or(0.0);
            }
        };

        let y = match self {
            Self::Text(_) => parent_rect.y,
            Self::Block(_) | Self::Inline(_) | Self::Scroll(_) | Self::ViewPort(_) => {
                big_brother_rect.y + big_brother_rect.height
            }
        };

        y + pad_top.unwrap_or(0.0)
    }

    pub fn init_with_text(
        txt: String,
        rectangle: Option<Rectangle>,
        font: Option<PaintFont>,
    ) -> Self {
        let rectangle = rectangle.unwrap_or(Rectangle {
            x: 0.0,
            y: 45.0,
            width: 900.0,
            height: 700.0,
        });

        let font = font.unwrap_or(PaintFont::new(None, None));

        Self::Text(TextRenderObject {
            text: txt,
            rectangle,
            font,
        })
    }

    pub fn init_with_element(
        element_type: ElementType,
        window_width: f32,
        window_height: f32,
    ) -> Option<Self> {
        match element_type.tag_name {
            HTMLElements::BodyElement => Some(Self::Scroll(_RenderObject {
                children: vec![],
                style: vec![],
                rectangle: Rectangle::new(0.0, 0.0, 0.0, 0.0),
                window_size: WindowSize::new(window_width, window_height),
            })),
            HTMLElements::DivElement | HTMLElements::ParagraphElement | HTMLElements::H1Element => {
                Some(Self::Block(_RenderObject {
                    children: vec![],
                    style: vec![],
                    rectangle: Rectangle::new(0.0, 0.0, 0.0, 0.0),
                    window_size: WindowSize::new(window_width, window_height),
                }))
            }
            HTMLElements::AnchorElement | HTMLElements::SpanElement => {
                Some(Self::Inline(_RenderObject {
                    children: vec![],
                    style: vec![],
                    rectangle: Rectangle::new(0.0, 0.0, 0.0, 0.0),
                    window_size: WindowSize::new(window_width, window_height),
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
