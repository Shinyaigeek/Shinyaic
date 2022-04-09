use crate::paint::font::PaintFont;
use crate::layout_tree::rectangle::Rectangle;
use crate::layout_tree::window_size::WindowSize;
use shinyaic_core::css::cssom::cssom::StylingRule;
use shinyaic_core::html::dom::dom::{DOMNode, ElementType, NodeType};
use shinyaic_core::html::dom::elements::elements::HTMLElements;

#[derive(Debug, PartialEq, Clone)]
pub struct _LayoutObject {
    pub children: Vec<LayoutObject>,
    pub style: Vec<StylingRule>,
    pub rectangle: Rectangle,
    pub window_size: WindowSize,
    pub img_href: Option<String>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct TextLayoutObject {
    pub text: String,
    pub rectangle: Rectangle,
    pub font: PaintFont,
}

#[derive(Debug, PartialEq, Clone)]
pub enum LayoutObject {
    ViewPort(_LayoutObject),
    Scroll(_LayoutObject),
    Block(_LayoutObject),
    Inline(_LayoutObject),
    Text(TextLayoutObject),
}

impl LayoutObject {
    pub fn new() -> Self {
        Self::ViewPort(_LayoutObject {
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
                    let margin = self.fix_unit_to_px(margin.to_string());

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
                        self.fix_unit_to_px(margin_vertical.to_string())
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
                        self.fix_unit_to_px(margin_horizontal.to_string())
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

                let raw_width = if raw_width == "max-content" {
                    // TODO
                    Some(parent_width.clone())
                } else if raw_width.ends_with("%") {
                    let raw_width = raw_width.strip_suffix("%").unwrap();
                    Some(parent_width.clone() * raw_width.parse::<f32>().unwrap() / 100.0)
                } else {
                    self.fix_unit_to_px(raw_width.to_string())
                };

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
                let padding = padding.split(" ").collect::<Vec<&str>>();

                // TODO
                let padding = if padding.len() == 1 {
                    self.fix_unit_to_px(padding[0].to_string())
                } else if padding.len() == 2 {
                    Some(self.fix_unit_to_px(padding[1].to_string()).unwrap() * 2.0)
                } else {
                    Some(
                        self.fix_unit_to_px(padding[1].to_string()).unwrap()
                            + self.fix_unit_to_px(padding[3].to_string()).unwrap(),
                    )
                };

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
                let _height = style.declarations.get(&"height".to_string()).unwrap();
                let _height = self.fix_unit_to_px(_height.to_string());
                match _height {
                    Some(_height) => {
                        height = Some(_height);
                    }
                    None => {
                        panic!("TODO");
                    }
                }
            }

            if style.declarations.get(&"min-height".to_string()).is_some() {
                let min_heigt = style.declarations.get(&"min-height".to_string()).unwrap();

                let min_heigt = self.fix_unit_to_px(min_heigt.to_string());

                match min_heigt {
                    Some(_min_heigt) => {
                        if _min_heigt > height.unwrap_or(0.0) {
                            height = Some(_min_heigt);
                        }
                    }
                    None => {
                        panic!("TODO");
                    }
                }
            }

            if style.declarations.get(&"padding".to_string()).is_some() {
                let padding = style.declarations.get(&"padding".to_string()).unwrap();
                let padding = padding.split(" ").collect::<Vec<&str>>();

                // TODO
                let padding = if padding.len() == 1 {
                    Some(self.fix_unit_to_px(padding[0].to_string()).unwrap() * 2.0)
                } else if padding.len() == 2 {
                    Some(self.fix_unit_to_px(padding[0].to_string()).unwrap() * 2.0)
                } else {
                    Some(
                        self.fix_unit_to_px(padding[0].to_string()).unwrap()
                            + self.fix_unit_to_px(padding[2].to_string()).unwrap(),
                    )
                };

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

    fn get_window_size(&self) -> WindowSize {
        match self {
            Self::Text(_) => {
                // TODO
                panic!("TODO");
            }
            Self::Block(rendering_object)
            | Self::Inline(rendering_object)
            | Self::Scroll(rendering_object)
            | Self::ViewPort(rendering_object) => rendering_object.window_size.clone(),
        }
    }

    pub fn fix_unit_to_px(&self, value: String) -> Option<f32> {
        let window_size = self.get_window_size();
        let value = if value.starts_with(".") {
            let mut v = String::from("0");
            v.push_str(&value);
            v
        } else {
            value
        };
        if value.ends_with("px") {
            let str_value = value.strip_suffix("px").unwrap();
            return Some(str_value.parse::<f32>().unwrap());
        }

        if value.ends_with("em") {
            // TODO
            let str_value = value.strip_suffix("em").unwrap();
            return Some(str_value.parse::<f32>().unwrap() * 18.0);
        }

        if value.ends_with("vh") {
            // TODO
            let str_value = value.strip_suffix("vh").unwrap();
            let vh = str_value.parse::<f32>().unwrap();
            return Some(vh * window_size.height);
        }

        if value.ends_with("vw") {
            // TODO
            let str_value = value.strip_suffix("vw").unwrap();
            let vw = str_value.parse::<f32>().unwrap();
            return Some(vw * window_size.width);
        }

        println!("value: {:?}", value);

        Some(value.parse::<f32>().unwrap())
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

        Self::Text(TextLayoutObject {
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
            HTMLElements::BodyElement => Some(Self::Scroll(_LayoutObject {
                children: vec![],
                style: vec![],
                rectangle: Rectangle::new(0.0, 0.0, 0.0, 0.0),
                window_size: WindowSize::new(window_width, window_height),
            })),
            HTMLElements::DivElement | HTMLElements::ParagraphElement | HTMLElements::H1Element => {
                Some(Self::Block(_LayoutObject {
                    children: vec![],
                    style: vec![],
                    rectangle: Rectangle::new(0.0, 0.0, 0.0, 0.0),
                    window_size: WindowSize::new(window_width, window_height),
                }))
            }
            HTMLElements::AnchorElement | HTMLElements::SpanElement | HTMLElements::ImgElement => {
                Some(Self::Inline(_LayoutObject {
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
            || tag == &HTMLElements::ImgElement
    }

    pub fn can_init_text(dom_node: &DOMNode) -> bool {
        match &dom_node.node_type {
            NodeType::TextNode(_) => true,
            NodeType::DomNode(_) => false,
        }
    }

    pub fn push_child(&mut self, child: LayoutObject) {
        match self {
            Self::Text(_) => {
                panic!("LayoutObject::push_shild should not be called with text")
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
                panic!("LayoutObject::replace_style should not be called with text")
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
        let rendering_object = LayoutObject::Block(_LayoutObject {
            children: vec![],
            style: vec![],
            rectangle: Rectangle::new(0.0, 0.0, 0.0, 0.0),
            window_size: WindowSize::new(0.0, 0.0),
        });

        assert_eq!(
            rendering_object.fix_unit_to_px("10px".to_string()),
            Some(10.0)
        );
    }

    #[test]
    fn test_fix_unit_to_px_without_px() {
        let rendering_object = LayoutObject::Block(_LayoutObject {
            children: vec![],
            style: vec![],
            rectangle: Rectangle::new(0.0, 0.0, 0.0, 0.0),
            window_size: WindowSize::new(0.0, 0.0),
        });
        assert_eq!(
            rendering_object.fix_unit_to_px("10".to_string()),
            Some(10.0)
        );
    }
}
