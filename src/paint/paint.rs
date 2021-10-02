// TODO

use crate::css::parser::parser::Parser as CSSParser;
use crate::html::parser::parser::Parser;
use crate::paint::styling_handler::handle_background::handle_background;
use crate::paint::window_canvas::{create_block, create_text};
use crate::paint::wrapper::Wrapper;
use crate::render_tree::render_object::RenderObject;
use crate::render_tree::render_tree::RenderTree;
use iced::{
    scrollable, text_input, Color, Container, Element, Length, Sandbox, Scrollable, Settings,
    TextInput,
};
use konnnyaku_client::Client;

pub fn paint() {
    let mut settings = Settings::default();
    settings.window.size = (900, 700);
    Window::run(settings).unwrap();
}

// TODO
pub struct Window {
    scroll: scrollable::State,
    pub debug: bool,
    render_tree: Vec<RenderObject>,
    url_search_bar_text_value: text_input::State,
    url_searchbar_text: String,
}

fn prepare() -> RenderTree {

    let mut parser = Parser {
        pos: 0,
        input: "<!doctype html>
        <html>
        <head>
        <style>
        h1 {
            font-size: 24px;
            padding:4px;
        }

        body {
            background: #ffffef;
        }

        #box {
            background: #fff;
            padding: 12px;
            margin: 36px auto;
        }

        </style>
        </head>
        <body>
            <div id=\"box\">
            <h1>Hello, Shinyaic Browser!</h1>
            <p>
                shinyaic is a browser made by @Shinyaigeek
            </p>
            </div>
        </body>
        </html>".to_string()
    };

    let dom = parser.parse();
    println!("------");
    println!("{:?}", dom);

    let mut parser = CSSParser {
        pos: 0,
        input: "h1 {
            display: block;
            font-size: 2em;
            margin-top: 0.67em;
            margin-bottom: 0.67em;
            margin-left: 0;
            margin-right: 0;
            font-weight: bold;
        }
        body {
            min-height: 100vh;
        }"
        .to_string(),
    };

    let cssom = parser.parse();
    println!("------");
    println!("{:?}", cssom);

    let mut render_tree = RenderTree::new(dom, cssom);
    render_tree.constructor();
    println!("------");
    println!("{:#?}", render_tree);

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
            url_search_bar_text_value: text_input::State::new(),
            url_searchbar_text: "".to_string(),
        }
    }

    fn title(&self) -> String {
        String::from("shinyaic")
    }

    fn update(&mut self, event: Message) {
        match event {
            Message::UrlSearchBarTextInputChanged(text) => {
                self.url_searchbar_text = text;
            }
            Message::UrlSearchBarSubmit(url) => {
                println!("{}", url);
                let response = Client::get(url);
                let body = response.body;
                let mut parser = Parser {
                    pos: 0,
                    input: body,
                };
                let dom = parser.parse();
                println!("------");
                println!("{:?}", dom);
                // TODO link の css をちゃんと読む
                let mut parser = CSSParser {
                    pos: 0,
                    input:
                        "#id1 { width: 120; height: 90; background: rgba(255, 0, 0, 1); margin: 0; }
                    #id2 { width: 120; height: 90; background: rgba(0, 255, 0, 1); margin: 0; }
                    #id3 { width: 120; height: 90; background: rgba(0, 0, 255, 1); margin: 0; }"
                            .to_string(),
                };
                let cssom = parser.parse();
                println!("------");
                println!("{:?}", cssom);
                let mut render_tree = RenderTree::new(dom, cssom);
                render_tree.constructor();

                self.render_tree = render_tree.prepare_iterator();
            }
        }
    }

    fn view(&mut self) -> Element<Message> {
        let Window {
            scroll,
            url_searchbar_text,
            url_search_bar_text_value,
            ..
        } = self;

        let mut wrapper = Wrapper::new(900.0, 700.0);

        let url_search_bar = TextInput::new(
            url_search_bar_text_value,
            "url",
            url_searchbar_text,
            Message::UrlSearchBarTextInputChanged,
        )
        .padding(10)
        .on_submit(Message::UrlSearchBarSubmit(url_searchbar_text.to_string()));

        for item in &self.render_tree {
            match item {
                RenderObject::Text(text) => {
                    wrapper.items.push(create_text(
                        text.text.to_string(),
                        Color::new(0.0, 0.0, 0.0, 1.0),
                        text.rectangle.clone(),
                        text.font.clone(),
                    ));
                }
                RenderObject::ViewPort(rendering_object)
                | RenderObject::Block(rendering_object)
                | RenderObject::Inline(rendering_object)
                | RenderObject::Scroll(rendering_object) => {
                    let mut background_color = Color::new(1.0, 1.0, 1.0, 1.0);
                    for style in &rendering_object.style {
                        let bg = handle_background(style);
                        if bg.is_some() {
                            background_color = bg.unwrap();
                        }
                    }
                    wrapper.items.push(create_block(
                        background_color,
                        rendering_object.rectangle.clone(),
                    ));
                }
            };
        }

        let scrollable = Scrollable::new(scroll).push(url_search_bar).push(wrapper);

        Container::new(scrollable)
            .height(Length::Fill)
            .center_y()
            .into()
    }
}

#[derive(Debug, Clone)]
pub enum Message {
    UrlSearchBarTextInputChanged(String),
    UrlSearchBarSubmit(String),
}
