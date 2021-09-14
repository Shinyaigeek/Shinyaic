use crate::css::cssom::cssom::{StylingRule, CSSOM};
use crate::css::cssom::declarations::Declarations;
use crate::css::cssom::selector::{Selector, SelectorChildren, SelectorElm};
use std::collections::{HashMap, HashSet};

pub struct Parser {
    pub pos: usize,
    pub input: String,
}

impl Parser {
    pub fn parse(&mut self) -> CSSOM {
        let mut cssom = Vec::<StylingRule>::new();

        loop {
            let selector = self.parse_selector();
            //  eat {
            self.eat();

            let declarations = self.parse_declarations();

            cssom.push(StylingRule {
                selector,
                declarations,
            });

            self.goto_next_token();

            if self.is_eof() {
                break;
            }
        }

        cssom
    }

    fn eat(&mut self) -> u8 {
        self.pos += 1;
        self.input.as_bytes()[self.pos - 1]
    }

    fn peek(&self) -> u8 {
        self.input.as_bytes()[self.pos]
    }

    fn get_cur_char(&self) -> u8 {
        self.input.as_bytes()[self.pos - 1]
    }

    fn is_eof(&self) -> bool {
        self.pos + 1 >= self.input.as_bytes().len()
    }

    //  "."class "{" width: 80px }
    fn parse_selector(&mut self) -> Vec<Selector> {
        let mut selectors = Vec::<Selector>::new();
        let mut selector_children = Vec::<SelectorChildren>::new();

        let mut selector_elm = self.parse_selector_elm();

        loop {
            if self.peek() == b'{' {
                selectors.push(Selector {
                    elm: selector_elm.clone(),
                    children: selector_children,
                });
                break;
            }
            self.goto_next_token();
            // * " "と" ,"を分けるためにこうするしかなかった...

            let separation_char = match self.peek() {
                b',' => b',',
                b'>' => b'>',
                b'+' => b'+',
                b'~' => b'~',
                b'{' => b'{',
                _ => b' ',
            };

            if separation_char == b'{' {
                selectors.push(Selector {
                    elm: selector_elm.clone(),
                    children: vec![],
                });
                break;
            } else if separation_char == b',' {
                self.eat();
                selectors.push(Selector {
                    elm: selector_elm.clone(),
                    children: selector_children,
                });
                selector_elm = self.parse_selector_elm();
                selector_children = Vec::<SelectorChildren>::new();
            } else if separation_char == b' ' {
                selector_children.push(SelectorChildren::descendant_combinator(
                    self.parse_selector(),
                ));
            } else if separation_char == b'>' {
                self.eat();
                selector_children.push(SelectorChildren::child_combinator(self.parse_selector()));
            } else if separation_char == b'+' {
                self.eat();
                selector_children.push(SelectorChildren::adjacent_sibling_combinator(
                    self.parse_selector(),
                ));
            } else if separation_char == b'~' {
                self.eat();
                selector_children.push(SelectorChildren::general_sibling_combinator(
                    self.parse_selector(),
                ));
            } else {
                panic!(
                    "separation_char should not be {:?}",
                    separation_char as char
                );
            }
        }

        selectors
    }

    // "."clas"s" { width: 80px }
    fn parse_selector_elm(&mut self) -> SelectorElm {
        self.goto_next_token();
        let first_char_selector_elm = self.eat().clone();

        let mut elm = String::from("");

        loop {
            let peeked_char = self.peek();

            if peeked_char == b','
                || peeked_char == b' '
                || peeked_char == b'>'
                || peeked_char == b'{'
                || peeked_char == b'~'
                || peeked_char == b'+'
            {
                break;
            }
            elm.push(self.eat() as char);
        }

        match first_char_selector_elm {
            b'#' => SelectorElm::Id(elm),
            b'.' => SelectorElm::Class(elm),
            b'*' => SelectorElm::Asterisk("*".to_string()),
            _ => {
                let mut tag_name = String::from(first_char_selector_elm as char);
                tag_name.push_str(&elm);
                SelectorElm::TagName(tag_name)
            }
        }
    }

    // .clas"s"   " ", .class2
    fn goto_next_token(&mut self) {
        loop {
            let peeked = self.peek();

            if peeked != b' ' || self.is_eof() {
                break;
            }

            self.eat();
        }
    }

    //  .class "{" width: 80px "}"
    fn parse_declarations(&mut self) -> Declarations {
        let should_be_left_embrace = self.get_cur_char();

        if should_be_left_embrace != b'{' {
            panic!("[CSSOM] parse css failed! because in parse_declaration, should_ne_left_brace should be {{ but got {:?}", should_be_left_embrace);
        }

        let mut declarations: Declarations = HashMap::new();

        loop {
            self.goto_next_token();
            let cur_char = self.peek();

            if cur_char == b'}' {
                if !self.is_eof() {
                    self.eat();
                }
                break;
            }

            let (key, value) = self.parse_declaration();

            declarations.insert(key, value);
        }

        declarations
    }

    fn parse_declaration(&mut self) -> (String, String) {
        let key = self.parse_declaration_key();
        let value = self.parse_declaration_value();

        (key, value)
    }

    fn parse_declaration_key(&mut self) -> String {
        self.goto_next_token();
        let mut key = String::from("");

        loop {
            let cur_char = self.eat();

            if cur_char == b':' {
                break;
            } else {
                key.push(cur_char as char);
            }
        }

        key
    }

    fn parse_declaration_value(&mut self) -> String {
        self.goto_next_token();
        let mut value = String::from("");

        loop {
            let cur_char = self.eat();

            if cur_char == b';' {
                break;
            } else {
                value.push(cur_char as char);
            }
        }

        value
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_can_parse_a_simple_css() {
        let css = "body { color: red; }";
        let mut parser = Parser {
            pos: 0,
            input: String::from(css),
        };
        let result = parser.parse();
        let mut declarations = HashMap::new();
        declarations.insert("color".to_string(), "red".to_string());
        assert_eq!(
            result,
            vec![StylingRule {
                selector: vec![Selector {
                    elm: SelectorElm::TagName("body".to_string()),
                    children: vec![],
                }],
                declarations
            }]
        );
    }

    #[test]
    fn it_can_parse_selector_with_combinator() {
        let css = "body > div + p ~ a div { color: red; }";
        let mut parser = Parser {
            pos: 0,
            input: String::from(css),
        };
        let result = parser.parse();
        let mut declarations = HashMap::new();
        declarations.insert("color".to_string(), "red".to_string());
        assert_eq!(
            result,
            vec![StylingRule {
                selector: vec![Selector {
                    elm: SelectorElm::TagName("body".to_string()),
                    children: vec![SelectorChildren::child_combinator(vec![Selector {
                        elm: SelectorElm::TagName("div".to_string()),
                        children: vec![SelectorChildren::adjacent_sibling_combinator(vec![
                            Selector {
                                elm: SelectorElm::TagName("p".to_string()),
                                children: vec![SelectorChildren::general_sibling_combinator(vec![
                                    Selector {
                                        elm: SelectorElm::TagName("a".to_string()),
                                        children: vec![SelectorChildren::descendant_combinator(
                                            vec![Selector {
                                                elm: SelectorElm::TagName("div".to_string()),
                                                children: vec![],
                                            }]
                                        )]
                                    }
                                ])]
                            }
                        ])]
                    }])]
                }],
                declarations
            }]
        );
    }

    #[test]
    fn it_can_parse_multi_selector() {
        let css = ".class1, .class2 { color: red; }";
        let mut parser = Parser {
            pos: 0,
            input: String::from(css),
        };
        let result = parser.parse();
        let mut declarations = HashMap::new();
        declarations.insert("color".to_string(), "red".to_string());
        assert_eq!(
            result,
            vec![StylingRule {
                selector: vec![
                    Selector {
                        elm: SelectorElm::Class("class1".to_string()),
                        children: vec![],
                    },
                    Selector {
                        elm: SelectorElm::Class("class2".to_string()),
                        children: vec![],
                    }
                ],
                declarations: declarations.clone(),
            }]
        );
    }

    #[test]
    fn it_can_parse_multiple_declarations() {
        let css = "body { color: red; background: white; }";
        let mut parser = Parser {
            pos: 0,
            input: String::from(css),
        };
        let result = parser.parse();
        let mut declarations = HashMap::new();
        declarations.insert("color".to_string(), "red".to_string());
        declarations.insert("background".to_string(), "white".to_string());
        assert_eq!(
            result,
            vec![StylingRule {
                selector: vec![Selector {
                    elm: SelectorElm::TagName("body".to_string()),
                    children: vec![],
                }],
                declarations
            }]
        );
    }

    #[test]
    fn it_can_parse_multiple_styles() {
        let css = "body { color: red; background: white; } p { color: blue; }";
        let mut parser = Parser {
            pos: 0,
            input: String::from(css),
        };
        let result = parser.parse();
        let mut declarations_body = HashMap::new();
        declarations_body.insert("color".to_string(), "red".to_string());
        declarations_body.insert("background".to_string(), "white".to_string());

        let mut declarations_p = HashMap::new();
        declarations_p.insert("color".to_string(), "blue".to_string());

        assert_eq!(
            result,
            vec![
                StylingRule {
                    selector: vec![Selector {
                        elm: SelectorElm::TagName("body".to_string()),
                        children: vec![],
                    }],
                    declarations: declarations_body.clone(),
                },
                StylingRule {
                    selector: vec![Selector {
                        elm: SelectorElm::TagName("p".to_string()),
                        children: vec![],
                    }],
                    declarations: declarations_p.clone(),
                }
            ]
        );
    }
}
