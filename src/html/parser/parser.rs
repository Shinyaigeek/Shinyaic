use crate::html::dom::dom::DOMNode;
use crate::html::dom::elements::elements::{
    HTMLElements, ANCHOR_ELEMENT, BODY_ELEMENT, HEAD_ELEMENT, HTML_ELEMENT, PARAGRAPH_ELEMENT,
};
use std::collections::HashMap;
use std::vec::Vec;

pub struct Parser {
    pub pos: usize,
    pub input: String,
}

impl Parser {
    pub fn parse(&mut self) -> DOMNode {
        let program = self.parse_node();
        program
    }

    fn eat_opening_tag(&mut self) -> (HTMLElements, HashMap<String, String>) {
        // `<`li id="id1" `>`</li> -> li
        let init_pos = self.input.as_bytes()[self.pos];
        if init_pos != b'<' {
            panic!("pick_tag_name was invoked not in < but in {:?}", init_pos)
        }

        self.eat();

        let mut tag: Vec<u8> = vec![];

        let mut attributes: HashMap<String, String> = HashMap::new();

        loop {
            let next_token = self.eat();

            if next_token == b'>' {
                break;
            }

            if next_token == b' ' {
                attributes = self.eat_element_attributes();
            }

            tag.push(next_token);
        }

        let tag = String::from_utf8(tag).unwrap();
        let tag: &str = &tag;

        let tag = match tag {
            HTML_ELEMENT => HTMLElements::HTML_ELEMENT,
            HEAD_ELEMENT => HTMLElements::HEAD_ELEMENT,
            BODY_ELEMENT => HTMLElements::BODY_ELEMENT,
            PARAGRAPH_ELEMENT => HTMLElements::PARAGRAPH_ELEMENT,
            ANCHOR_ELEMENT => HTMLElements::ANCHOR_ELEMENT,
            _ => panic!("there is no element, {:?}", tag),
        };

        (tag, attributes)
    }

    fn eat_closing_tag(&mut self) {
        let init_pos = self.eat();
        if init_pos != b'<' {
            panic!(
                "eat_closing_tag was invoked not in < but in {:?}",
                init_pos as char
            )
        }

        let should_slash = self.eat();
        if should_slash != b'/' {
            panic!(
                "eat_closing_tag was invoked not in / but in {:?}",
                should_slash as char
            )
        }

        loop {
            let token = self.eat();

            if token == b'>' {
                break;
            }
        }
    }

    fn eat_element_attributes(&mut self) -> HashMap<String, String> {
        // <li ` `id="asdf"`>`
        let init_pos = self.input.as_bytes()[self.pos];
        if init_pos != b' ' {
            panic!("pick_tag_name was invoked not in ` ` but in {:?}", init_pos)
        }

        let mut attributes: HashMap<String, String> = HashMap::new();
        let mut key = String::from("");
        let mut value = String::from("");
        let mut is_eating_key = true;

        loop {
            let next_token = self.eat();
            if !is_eating_key {
                if next_token == b'"' {
                    attributes.insert(key.clone(), value.clone());
                    is_eating_key = true;
                } else {
                    value.push(next_token as char);
                }
            } else {
                if next_token == b'>' {
                    break;
                }
                if next_token == b'"' {
                    is_eating_key = false;
                } else if next_token == b' ' || next_token == b'=' {
                    // do nothing
                } else {
                    value.push(next_token as char);
                }
            }
        }

        attributes
    }

    fn eat(&mut self) -> u8 {
        self.pos += 1;
        self.input.as_bytes()[self.pos - 1]
    }

    fn peek(&self) -> u8 {
        self.input.as_bytes()[self.pos]
    }

    fn peek_start_with(&self, value: String) -> bool {
        let mut token = String::from("");
        self.input[self.pos..].starts_with(&value)
    }

    fn go_to_next_left_tag(&mut self) {}

    fn parse_text(&mut self) -> DOMNode {
        let mut text = String::from("");

        while !self.peek_start_with("<".to_string()) {
            let c = self.eat();
            text.push(c as char);
        }

        DOMNode::text(text)
    }

    fn consume_comment(&mut self) {}

    fn parse_node(&mut self) -> DOMNode {
        // `<`li></LI`>`
        if self.input.as_bytes()[self.pos] != b'<' {
            // text node
            return self.parse_text();
        }
        let (target_tag_name, attributes) = self.eat_opening_tag();
        let node = DOMNode::elem(target_tag_name, attributes, self.parse_nodes());

        self.eat_closing_tag();
        node
    }

    fn parse_nodes(&mut self) -> Vec<DOMNode> {
        // input: <head></head><body><div>hello</div></body>    <ul>`<`li>1</li><li>1</li><li>1</li></ul>
        let mut nodes: Vec<DOMNode> = vec![];

        // 終了条件: eof or </
        while !self.peek_start_with("</".to_string()) && !(self.input.len() <= self.pos) {
            nodes.push(self.parse_node());
        }

        nodes
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parser_works() {
        let mut parser = Parser {
            pos: 0,
            input: "<html><head></head><body><p>hoge</p><p>asdf</p></body></html>".to_string(),
        };

        let dom = parser.parse();

        let expected_dom = DOMNode::elem(
            HTMLElements::HTML_ELEMENT,
            HashMap::new(),
            vec![
                DOMNode::elem(HTMLElements::HEAD_ELEMENT, HashMap::new(), vec![]),
                DOMNode::elem(
                    HTMLElements::BODY_ELEMENT,
                    HashMap::new(),
                    vec![
                        DOMNode::elem(
                            HTMLElements::PARAGRAPH_ELEMENT,
                            HashMap::new(),
                            vec![DOMNode::text(String::from("hoge"))],
                        ),
                        DOMNode::elem(
                            HTMLElements::PARAGRAPH_ELEMENT,
                            HashMap::new(),
                            vec![DOMNode::text(String::from("asdf"))],
                        ),
                    ],
                ),
            ],
        );

        assert_eq!(dom, expected_dom);
    }
}
