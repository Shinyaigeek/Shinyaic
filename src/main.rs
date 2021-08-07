use crate::css::cssom::cssom::CSSOM;
use crate::css::parser::parser::Parser as CSSParser;
use crate::html::parser::parser::Parser;
use crate::paint::paint::paint;
use crate::render_tree::rectangle::Rectangle;
use crate::render_tree::render_object::RenderObject;
use crate::render_tree::render_tree::RenderTree;

mod css;
mod html;
mod paint;
mod render_tree;

fn main() {
    // let mut parser = Parser {
    //     pos: 0,
    //     input: "<html><head></head><body><p>hoge</p><p>asdf</p></body></html>".to_string(),
    // };

    // let dom = parser.parse();
    // println!("------");
    // println!("{:?}", dom);

    // let mut parser = CSSParser {
    //     pos: 0,
    //     input: "body p { width: 80; height: 90; }".to_string(),
    // };

    // let cssom = parser.parse();
    // println!("------");
    // println!("{:?}", cssom);

    // let mut render_tree = RenderTree::new(dom, cssom);
    // render_tree.constructor();

    // println!("{:?}", render_tree.tree);

    paint();
}
