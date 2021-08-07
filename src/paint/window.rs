// TODO

use crate::css::cssom::cssom::CSSOM;
use crate::css::parser::parser::Parser as CSSParser;
use crate::html::parser::parser::Parser;
use crate::render_tree::rectangle::Rectangle;
use crate::render_tree::render_object::RenderObject;
use crate::render_tree::render_tree::RenderTree;
use iced::{
    button, scrollable, slider, text_input, Button, Checkbox, Color, Column, Container, Element,
    HorizontalAlignment, Image, Length, Radio, Row, Sandbox, Scrollable, Settings, Slider, Space,
    Text, TextInput,
};

// TODO
pub struct Window {
    scroll: scrollable::State,
    debug: bool,
    render_tree: RenderTree,
}

fn prepare() -> RenderTree {
    let mut parser = Parser {
        pos: 0,
        input: "<html><head></head><body><p>hoge</p><p>asdf</p></body></html>".to_string(),
    };

    let dom = parser.parse();
    println!("------");
    println!("{:?}", dom);

    let mut parser = CSSParser {
        pos: 0,
        input: "body p { width: 80; height: 90; }".to_string(),
    };

    let cssom = parser.parse();
    println!("------");
    println!("{:?}", cssom);

    let mut render_tree = RenderTree::new(dom, cssom);
    render_tree.constructor();

    render_tree
}

impl Sandbox for Window {
    type Message = Message;
    fn new() -> Window {
        let render_tree = prepare();
        Window {
            scroll: scrollable::State::new(),
            debug: false,
            render_tree,
        }
    }

    fn title(&self) -> String {
        String::from("shinyaic")
    }

    fn update(&mut self, event: Message) {
        match event {}
    }

    fn view(&mut self) -> Element<Message> {
        let Window { scroll, .. } = self;

        // let mut wrapper = Wrapper::new(300, 300);

        // for item in &self.render_tree {
        // wrapper.items.push(match item {
        //     DisplayCommand::SolidColor(color, rect) => {
        //         painter::create_block(color.clone(), rect.clone())
        //     }
        //     DisplayCommand::Text(text, color, rect, font) => painter::create_text(
        //         text.into(),
        //         color.clone(),
        //         rect.clone(),
        //         font.clone(),
        //         font_context,
        //     ),
        // });
        // }

        let scrollable = Scrollable::new(scroll);

        Container::new(scrollable)
            .height(Length::Fill)
            .center_y()
            .into()
    }
}

#[derive(Debug, Clone)]
pub enum Message {}

enum DisplayCommand {
    Text(String, Color, Rectangle),
    SolidColor(Color, Rectangle),
}

struct DisplayList {
    list: Vec<DisplayCommand>,
}

impl DisplayList {
    pub fn new() -> DisplayList {
        DisplayList { list: Vec::new() }
    }

    pub fn constructor(render_tree: RenderTree) {}

    fn traverse(&mut self, render_object: &RenderObject, parent_render_object: &RenderObject) {
        let parent_render_object = match parent_render_object {
            RenderObject::Text(_) => panic!("todo"),
            RenderObject::ViewPort(render_object)
            | RenderObject::Scroll(render_object)
            | RenderObject::Block(render_object) => render_object,
        };
        match render_object {
            RenderObject::Text(text) => {
                self.list.push(DisplayCommand::Text(
                    text.clone(),
                    Color {
                        r: 0.0,
                        g: 0.0,
                        b: 0.0,
                        a: 0.0,
                    },
                    parent_render_object.rectangle.clone(),
                ));
            }
            RenderObject::ViewPort(render_object)
            | RenderObject::Scroll(render_object)
            | RenderObject::Block(render_object) => {
                self.list.push(DisplayCommand::SolidColor(
                    Color {
                        r: 255.0,
                        g: 0.0,
                        b: 0.0,
                        a: 0.0,
                    },
                    parent_render_object.rectangle.clone(),
                ));
            }
        };
    }
}
