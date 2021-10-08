use crate::html::dom::dom::DOMNode;
use crate::html::dom::elements::elements::HTMLElements;
use std::collections::HashMap;
use std::vec::Vec;

pub struct Parser {
    pub pos: usize,
    pub input: String,
}

impl Parser {
    pub fn parse(&mut self) -> DOMNode {
        if self.peek_start_with("<!DOCTYPE") || self.peek_start_with("<!doctype") {
            self.parse_doctype();
            self.eat_whitespace();
        }
        let program = self.parse_node();

        program
    }

    fn eat_opening_tag(&mut self) -> (HTMLElements, HashMap<String, String>) {
        // `<`li id="id1" `>`</li> -> li
        let init_pos = self.input.chars().nth(self.pos).unwrap();
        if init_pos != '<' {
            panic!("pick_tag_name was invoked not in < but in {:?}", init_pos)
        }

        self.eat();

        let mut tag: Vec<char> = vec![];

        let mut attributes: HashMap<String, String> = HashMap::new();

        loop {
            let next_token = self.eat();

            if next_token == '>' {
                break;
            }

            if next_token == ' ' {
                attributes = self.eat_element_attributes();
                break;
            }

            tag.push(next_token);
        }

        let tag = tag.into_iter().collect::<String>();
        let raw_tag: &str = &tag;

        let tag = HTMLElements::init_from_str_tag(raw_tag);

        let tag = match tag {
            Some(tag) => tag,
            None => panic!("Unknown tag: {}", raw_tag),
        };

        (tag, attributes)
    }

    fn eat_closing_tag(&mut self) {
        let init_pos = self.eat();
        if init_pos != '<' {
            panic!(
                "eat_closing_tag was invoked not in < but in {:?}",
                init_pos as char
            )
        }

        let should_slash = self.eat();
        if should_slash != '/' {
            panic!(
                "eat_closing_tag was invoked not in / but in {:?}",
                should_slash as char
            )
        }

        loop {
            let token = self.eat();

            if token == '>' {
                break;
            }
        }
    }

    fn eat_element_attributes(&mut self) -> HashMap<String, String> {
        // <li ` `id="asdf"`>`

        let mut attributes: HashMap<String, String> = HashMap::new();
        let mut key = String::from("");
        let mut value = String::from("");
        let mut is_eating_key = true;

        loop {
            let next_token = self.eat();
            if !is_eating_key {
                if next_token == '"' {
                    attributes.insert(key.to_string(), value.to_string());
                    key = String::from("");
                    value = String::from("");
                    is_eating_key = true;
                } else {
                    value.push(next_token as char);
                }
            } else {
                if next_token == '>' {
                    break;
                }
                if next_token == '"' {
                    is_eating_key = false;
                } else if next_token == ' ' || next_token == '=' {
                    // do nothing
                } else {
                    key.push(next_token as char);
                }
            }
        }

        attributes
    }

    fn eat(&mut self) -> char {
        self.pos += 1;
        self.input.chars().nth(self.pos - 1).unwrap_or(0 as char)
    }

    pub fn eat_whitespace(&mut self) {
        loop {
            if self.pos >= self.input.len() {
                break;
            }

            let next_ch = self.peek();

            // EOF or breakline or whitespace or tab
            if !(next_ch == 0 as char || next_ch == '\n' || next_ch == ' ' || next_ch == '\t') {
                break;
            }

            self.eat();
        }
    }

    fn peek(&self) -> char {
        if self.pos >= self.input.chars().count() {
            return 0 as char;
        }
        self.input.chars().nth(self.pos).unwrap()
    }

    fn peek_start_with<S: Into<String>>(&self, value: S) -> bool {
        // TODO
        // self.input[self.pos..].starts_with(value.into().as_str())

        let mut input = self.input.chars();
        let value = value.into();

        let mut value = value.chars();

        if value.next().unwrap() != input.nth(self.pos).unwrap_or(0 as char) {
            return false;
        }

        for v in value {
            let i = input.next().unwrap();

            if i != v {
                return false;
            }
            if input.clone().count() <= 0 {
                return false;
            }
        }

        true
    }

    fn parse_text(&mut self) -> DOMNode {
        let mut text = String::from("");

        while !self.peek_start_with("<") {
            let c = self.eat();
            text.push(c as char);
        }

        DOMNode::text(text)
    }

    // TODO 暫定的にpeekをdoctypeの次まで進めるだけ
    fn parse_doctype(&mut self) {
        if self.eat() != '<' {
            panic!("parse_doctype was invoked not in < but in {:?}", self.eat());
        }

        loop {
            let ch = self.eat();

            if ch == '>' {
                break;
            }
        }
    }

    fn parse_node(&mut self) -> DOMNode {
        self.eat_whitespace();
        // `<`li></LI`>`
        if self.input.chars().nth(self.pos).unwrap() != '<' {
            // text node
            return self.parse_text();
        }
        let (target_tag_name, attributes) = self.eat_opening_tag();

        let node = if !target_tag_name.need_closing_tag() {
            DOMNode::elem(target_tag_name, attributes, vec![])
        } else {
            let node = DOMNode::elem(target_tag_name, attributes, self.parse_nodes());
            self.eat_closing_tag();
            node
        };

        self.eat_whitespace();
        node
    }

    fn parse_nodes(&mut self) -> Vec<DOMNode> {
        self.eat_whitespace();
        // input: <head></head><body><div>hello</div></body>    <ul>`<`li>1</li><li>1</li><li>1</li></ul>
        let mut nodes: Vec<DOMNode> = vec![];

        // 終了条件: eof or </
        while !self.peek_start_with("</") && !(self.input.len() <= self.pos) {
            nodes.push(self.parse_node());
        }
        self.eat_whitespace();

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
            HTMLElements::HtmlElement,
            HashMap::new(),
            vec![
                DOMNode::elem(HTMLElements::HeadElement, HashMap::new(), vec![]),
                DOMNode::elem(
                    HTMLElements::BodyElement,
                    HashMap::new(),
                    vec![
                        DOMNode::elem(
                            HTMLElements::ParagraphElement,
                            HashMap::new(),
                            vec![DOMNode::text(String::from("hoge"))],
                        ),
                        DOMNode::elem(
                            HTMLElements::ParagraphElement,
                            HashMap::new(),
                            vec![DOMNode::text(String::from("asdf"))],
                        ),
                    ],
                ),
            ],
        );

        assert_eq!(dom, expected_dom);
    }

    #[test]
    fn parse_html_with_doctype() {
        let mut parser = Parser {
            pos: 0,
            input: "<!DOCTYPE html><html><head></head><body><p>hoge</p><p>asdf</p></body></html>"
                .to_string(),
        };

        let dom = parser.parse();

        let expected_dom = DOMNode::elem(
            HTMLElements::HtmlElement,
            HashMap::new(),
            vec![
                DOMNode::elem(HTMLElements::HeadElement, HashMap::new(), vec![]),
                DOMNode::elem(
                    HTMLElements::BodyElement,
                    HashMap::new(),
                    vec![
                        DOMNode::elem(
                            HTMLElements::ParagraphElement,
                            HashMap::new(),
                            vec![DOMNode::text(String::from("hoge"))],
                        ),
                        DOMNode::elem(
                            HTMLElements::ParagraphElement,
                            HashMap::new(),
                            vec![DOMNode::text(String::from("asdf"))],
                        ),
                    ],
                ),
            ],
        );

        assert_eq!(dom, expected_dom);
    }

    #[test]
    fn parse_html_with_breakline() {
        let mut parser = Parser {
            pos: 0,
            input: "<html>
<head>
</head>
<body>
    <p>hoge</p>
    <p>asdf</p>
</body>
</html>"
                .to_string(),
        };

        let dom = parser.parse();

        let expected_dom = DOMNode::elem(
            HTMLElements::HtmlElement,
            HashMap::new(),
            vec![
                DOMNode::elem(HTMLElements::HeadElement, HashMap::new(), vec![]),
                DOMNode::elem(
                    HTMLElements::BodyElement,
                    HashMap::new(),
                    vec![
                        DOMNode::elem(
                            HTMLElements::ParagraphElement,
                            HashMap::new(),
                            vec![DOMNode::text(String::from("hoge"))],
                        ),
                        DOMNode::elem(
                            HTMLElements::ParagraphElement,
                            HashMap::new(),
                            vec![DOMNode::text(String::from("asdf"))],
                        ),
                    ],
                ),
            ],
        );

        assert_eq!(dom, expected_dom);
    }

    #[test]
    fn parser_works_with_attributes() {
        let mut parser = Parser {
            pos: 0,
            input: "<html><head></head><body><p id=\"fuga\">hoge</p><p>asdf</p></body></html>"
                .to_string(),
        };

        let dom = parser.parse();

        let mut id: HashMap<String, String> = HashMap::new();
        id.insert("id".to_string(), "fuga".to_string());

        let expected_dom = DOMNode::elem(
            HTMLElements::HtmlElement,
            HashMap::new(),
            vec![
                DOMNode::elem(HTMLElements::HeadElement, HashMap::new(), vec![]),
                DOMNode::elem(
                    HTMLElements::BodyElement,
                    HashMap::new(),
                    vec![
                        DOMNode::elem(
                            HTMLElements::ParagraphElement,
                            id,
                            vec![DOMNode::text(String::from("hoge"))],
                        ),
                        DOMNode::elem(
                            HTMLElements::ParagraphElement,
                            HashMap::new(),
                            vec![DOMNode::text(String::from("asdf"))],
                        ),
                    ],
                ),
            ],
        );

        assert_eq!(dom, expected_dom);
    }

    #[test]
    fn html_parser_works_with_children() {
        let mut parser = Parser {
            pos: 0,
            input:
                "<html><head></head><body><div><p>hoge</p><p>fuga</p><p>bar</p></div></body></html>"
                    .to_string(),
        };

        let dom = parser.parse();
        assert_eq!(dom.children[1].children[0].children.len(), 3);
    }
}
