use crate::css::cssom::cssom::CSSOM;
use crate::css::parser::parser::Parser as CSSParser;
use crate::html::parser::parser::Parser;

mod css;
mod html;
mod render_tree;

fn main() {
    let mut parser = Parser {
        pos: 0,
        input: "<html><head></head><body><p>hoge</p><p>asdf</p></body></html>".to_string(),
    };

    let dom = parser.parse();
    println!("------");
    println!("{:?}", dom);

    let mut parser = CSSParser {
        pos: 0,
        input: ".css > #child { width: 80px; }".to_string(),
    };

    let cssom = parser.parse();
    println!("------");
    println!("{:?}", cssom);

    let mut parser = CSSParser {
        pos: 0,
        input: ".css #child { width: 80px; }".to_string(),
    };

    let cssom = parser.parse();
    println!("------");
    println!("{:?}", cssom);

    let mut parser = CSSParser {
        pos: 0,
        input: ".css { width: 80px; }".to_string(),
    };

    let cssom = parser.parse();
    println!("------");
    println!("{:?}", cssom);
}
