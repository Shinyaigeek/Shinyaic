// TODO

use crate::css::cssom::cssom::CSSOM;
use crate::css::parser::parser::Parser as CSSParser;
use crate::html::parser::parser::Parser;
use crate::paint::window_canvas::{create_block, create_text};
use crate::paint::wrapper::Wrapper;
use crate::render_tree::rectangle::Rectangle;
use crate::render_tree::render_object::RenderObject;
use crate::render_tree::render_tree::RenderTree;
use iced::{
    button, scrollable, slider, text_input, Button, Checkbox, Color, Column, Container, Element,
    HorizontalAlignment, Image, Length, Radio, Row, Sandbox, Scrollable, Settings, Slider, Space,
    Text, TextInput,
};
use iced_native::Font;
pub fn paint() {
    let mut settings = Settings::default();
    settings.window.size = (300, 300);
    Window::run(settings);
}

// TODO
pub struct Window {
    scroll: scrollable::State,
    debug: bool,
    render_tree: Vec<RenderObject>,
}

fn prepare() -> RenderTree {
    let mut parser = Parser {
        pos: 0,
        input: "<html><head></head><body><p id=\"id1\">hello browser!</p><p id=\"id2\"></p><p id=\"id3\"></p></body></html>".to_string(),
    };

    let dom = parser.parse();
    println!("------");
    println!("{:?}", dom);

    let mut parser = CSSParser {
        pos: 0,
        input: "#id1 { width: 120; height: 90; background: rgba(255, 0, 0, 1); margin: 0; }
        #id2 { width: 120; height: 90; background: rgba(0, 255, 0, 1); margin: 0; }
        #id3 { width: 120; height: 90; background: rgba(0, 0, 255, 1); margin: 0; }"
            .to_string(),
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
        let rendering_objects = render_tree.prepare_iterator();
        Window {
            scroll: scrollable::State::new(),
            debug: false,
            render_tree: rendering_objects,
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

        let mut wrapper = Wrapper::new(300.0, 300.0);

        for item in &self.render_tree {
            println!("-----------------");
            println!("item: {:?}", item);
            match item {
                RenderObject::Text(text) => {
                    wrapper.items.push(create_text(
                        text.to_string(),
                        Color::new(0.0, 0.0, 0.0, 1.0),
                        Rectangle {
                            x: 0.0,
                            y: 0.0,
                            width: 300.0,
                            height: 300.0,
                        },
                        Font::default(),
                    ));
                }
                RenderObject::ViewPort(rendering_object)
                | RenderObject::Block(rendering_object)
                | RenderObject::Scroll(rendering_object) => {
                    let mut background_color = Color::new(1.0, 1.0, 1.0, 1.0);
                    for style in &rendering_object.style {
                        if style.declarations.get(&"background".to_string()).is_some() {
                            let mut raw_background_color = style
                                .declarations
                                .get(&"background".to_string())
                                .unwrap()
                                .clone();
                            raw_background_color.retain(|c| {
                                c == ','
                                    || c == '.'
                                    || c == '1'
                                    || c == '0'
                                    || c == '2'
                                    || c == '3'
                                    || c == '4'
                                    || c == '5'
                                    || c == '6'
                                    || c == '7'
                                    || c == '8'
                                    || c == '9'
                            });

                            let colors = &raw_background_color;
                            let colors: Vec<&str> = colors.split(",").collect();
                            background_color = Color::from_rgba8(
                                colors[0].parse::<u8>().unwrap(),
                                colors[1].parse::<u8>().unwrap(),
                                colors[2].parse::<u8>().unwrap(),
                                colors[3].parse::<f32>().unwrap(),
                            );
                        }
                    }
                    wrapper.items.push(create_block(
                        background_color,
                        rendering_object.rectangle.clone(),
                    ));
                }
            };
        }

        let scrollable = Scrollable::new(scroll).push(wrapper);

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
