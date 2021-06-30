use crate::css::cssom::cssom::CSSOM;
use crate::html::parser::parser::Parser;

mod css;
mod html;

fn main() {
    let mut parser = Parser {
        pos: 0,
        input: "<html><head></head><body><p>hoge</p><p>asdf</p></body></html>".to_string(),
    };

    let dom = parser.parse();
    println!("------");
    println!("{:?}", dom);
}
