// TODO

use crate::paint::border::Border;
use crate::paint::styling_handler::handle_background::handle_background;
use crate::paint::util::load_default_css::load_default_css;
use crate::paint::window_canvas::{create_block, create_text};
use crate::render_tree::render_object::RenderObject;
use crate::render_tree::render_tree::RenderTree;
use crate::wasm_bindgen::JsCast;
use shinyaic_core::css::parser::parser::Parser as CSSParser;
use shinyaic_core::html::dom::dom::DOMNode;
use shinyaic_core::html::dom::dom::NodeType;
use shinyaic_core::html::dom::elements::elements::HTMLElements;
use shinyaic_core::html::parser::parser::Parser;
use wasm_bindgen::prelude::*;
use web_sys::CanvasRenderingContext2d;

#[wasm_bindgen]
extern "C" {
    // Use `js_namespace` here to bind `console.log(..)` instead of just
    // `log(..)`
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

fn prepare() -> RenderTree {
    let mut parser = Parser::new(
        r#"<!doctype html>
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
        <div id="box">
        <h1>Hello, Shinyaic Browser!</h1>
        <p>
            shinyaic is a browser made by @Shinyaigeek
        </p>
        </div>
    </body>
    </html>"#
            .to_string(),
    );

    let dom = parser.parse();

    log(&format!("{:?}", dom));
    println!("------");
    println!("{:?}", dom);

    let default_css = load_default_css();

    let mut parser = CSSParser {
        pos: 0,
        input: default_css,
    };

    let cssom = parser.parse();
    log(&format!("{:?}", cssom));
    println!("------");
    println!("{:?}", cssom);

    let mut render_tree = RenderTree::new(dom, cssom);
    render_tree.constructor();
    println!("------");
    println!("{:#?}", render_tree);
    log(&format!("{:?}", render_tree));

    render_tree
}

pub fn paint() {
    let render_tree = prepare();

    let rendering_objects = render_tree.prepare_iterator();

    let document = web_sys::window().unwrap().document().unwrap();
    let canvas = document.get_element_by_id("canvas").unwrap();
    let canvas: web_sys::HtmlCanvasElement = canvas
        .dyn_into::<web_sys::HtmlCanvasElement>()
        .map_err(|_| ())
        .unwrap();

    let context = canvas
        .get_context("2d")
        .unwrap()
        .unwrap()
        .dyn_into::<web_sys::CanvasRenderingContext2d>()
        .unwrap();
    for item in rendering_objects {
        match item {
            RenderObject::Text(text) => {
                create_text(
                    text.text.to_string(),
                    "black".to_string(),
                    text.rectangle.clone(),
                    text.font.clone(),
                    &context,
                );
            }
            RenderObject::ViewPort(rendering_object)
            | RenderObject::Block(rendering_object)
            | RenderObject::Inline(rendering_object)
            | RenderObject::Scroll(rendering_object) => {
                let mut background_color = "white".to_string();
                let mut border = Border::new(None, None, None, None);
                for style in &rendering_object.style {
                    let bg = handle_background(style);
                    if bg.is_some() {
                        background_color = bg.unwrap();
                    }

                    // TODO
                    if style.declarations.get("border").is_some() {
                        border.apply_shorthand(style.declarations.get("border").unwrap());
                    }

                    // TODO
                    if style.declarations.get("border-radius").is_some() {
                        border.apply_radius(style.declarations.get("border-radius").unwrap());
                    }
                }
                create_block(
                    background_color.to_string(),
                    border,
                    rendering_object.rectangle.clone(),
                    &context,
                );
            }
        };
    }
}

fn get_external_css(dom: &DOMNode) -> String {
    let mut css_source = String::from("");

    // match &dom.node_type {
    //     NodeType::DomNode(dom_node) => match dom_node.tag_name {
    //         HTMLElements::LinkElement => {
    //             if dom_node.attributes.get("type").unwrap_or(&"".to_string()) == "text/css" {
    //                 let external_css_href = dom_node.attributes.get("href").unwrap();
    //                 let external_css_source = Client::get(external_css_href.to_string());
    //                 let external_css_source = external_css_source.body;
    //                 css_source.push_str(&external_css_source)
    //             }
    //         }
    //         _ => {
    //             for child in dom.children.clone() {
    //                 css_source.push_str(&get_external_css(&child));
    //             }
    //         }
    //     },
    //     NodeType::TextNode(_) => {
    //         return css_source;
    //     }
    // };

    css_source
}

#[derive(Debug, Clone)]
pub enum Message {
    UrlSearchBarTextInputChanged(String),
    UrlSearchBarSubmit(String),
}
