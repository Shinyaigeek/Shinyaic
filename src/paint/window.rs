// TODO

use iced::{
    button, scrollable, slider, text_input, Button, Checkbox, Color, Column, Container, Element,
    HorizontalAlignment, Image, Length, Radio, Row, Sandbox, Scrollable, Settings, Slider, Space,
    Text, TextInput,
};

// TODO
struct Window {
    scroll: scrollable::State,
    debug: bool,
    render_tree: RenderTree,
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

fn paint(render_tree: RenderTree) {
    let mut settings = Settings::default();
    settings.window.size = (300, 300);
    Window::run(settings);
}

// struct Color {
//     pub r: u8,
//     pub g: u8,
//     pub b: u8,
//     pub a: u8,
// }

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