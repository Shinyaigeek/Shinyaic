use crate::css::cssom::cssom::{StylingRule, CSSOM};
use crate::css::cssom::declarations::Declarations;
use crate::css::cssom::selector::{Selector, SelectorChildren, SelectorElm};
use std::collections::HashMap;

pub struct Parser {
    pub pos: usize,
    pub input: String,
}

impl Parser {
    pub fn parse(&mut self) -> CSSOM {
        let mut cssom = Vec::<StylingRule>::new();
        self.goto_next_token();

        loop {
            if self.peek_start_with("@") {
                // self.parse_media_query(&mut cssom);
                // TODO
                break;
            } else {
                self.parse_style(&mut cssom);
            };

            self.goto_next_token();

            if self.is_eof() {
                break;
            }
        }

        cssom
    }

    fn eat(&mut self) -> char {
        self.pos += 1;
        self.input.chars().nth(self.pos - 1).unwrap_or(0 as char)
    }

    fn peek(&self) -> char {
        if self.pos >= self.input.len() {
            return 0 as char;
        }

        self.input.chars().nth(self.pos).unwrap()
    }

    fn parse_style(&mut self, cssom: &mut Vec<StylingRule>) {
        let selector = self.parse_selector();
        //  eat {
        self.eat();

        let declarations = self.parse_declarations();

        cssom.push(StylingRule {
            selector,
            declarations,
        });
    }

    fn parse_media_query(&mut self, cssom: &mut Vec<StylingRule>) {
        let selector = self.parse_selector();
        //  eat {
        self.eat();

        let declarations = self.parse_declarations();

        cssom.push(StylingRule {
            selector,
            declarations,
        });
    }

    fn get_cur_char(&self) -> char {
        self.input.chars().nth(self.pos - 1).unwrap_or(0 as char)
    }

    fn is_eof(&self) -> bool {
        self.pos + 1 >= self.input.len()
    }

    //  "."class "{" width: 80px }
    fn parse_selector(&mut self) -> Vec<Selector> {
        let mut selectors = Vec::<Selector>::new();
        let mut selector_children = Vec::<SelectorChildren>::new();

        let mut selector_elm = self.parse_selector_elm();

        loop {
            if self.peek() == '{' {
                selectors.push(Selector {
                    elm: selector_elm.clone(),
                    children: selector_children,
                    pseudo_elements: None,
                });
                break;
            }
            self.goto_next_token();
            // * " "と" ,"を分けるためにこうするしかなかった...

            let separation_char = match self.peek() {
                ',' => ',',
                '>' => '>',
                '+' => '+',
                '~' => '~',
                '{' => '{',
                _ => ' ',
            };

            if separation_char == '{' {
                selectors.push(Selector {
                    elm: selector_elm.clone(),
                    children: vec![],
                    pseudo_elements: None,
                });
                break;
            } else if separation_char == ',' {
                self.eat();
                selectors.push(Selector {
                    elm: selector_elm.clone(),
                    children: selector_children,
                    pseudo_elements: None,
                });
                selector_elm = self.parse_selector_elm();
                selector_children = Vec::<SelectorChildren>::new();
            } else if separation_char == ' ' {
                selector_children.push(SelectorChildren::DescendantCombinator(
                    self.parse_selector(),
                ));
            } else if separation_char == '>' {
                self.eat();
                selector_children.push(SelectorChildren::ChildCombinator(self.parse_selector()));
            } else if separation_char == '+' {
                self.eat();
                selector_children.push(SelectorChildren::AdjacentSiblingCombinator(
                    self.parse_selector(),
                ));
            } else if separation_char == '~' {
                self.eat();
                selector_children.push(SelectorChildren::GeneralSiblingCombinator(
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

            if peeked_char == ','
                || peeked_char == ' '
                || peeked_char == '>'
                || peeked_char == '{'
                || peeked_char == '~'
                || peeked_char == '+'
            {
                break;
            }
            elm.push(self.eat() as char);
        }

        match first_char_selector_elm {
            '#' => SelectorElm::Id(elm),
            '.' => SelectorElm::Class(elm),
            '*' => SelectorElm::Asterisk("*".to_string()),
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

            if self.is_eof() || (peeked != ' ' && peeked != '\n' && peeked != '\t') {
                break;
            }

            self.eat();
        }
    }

    //  .class "{" width: 80px "}"
    fn parse_declarations(&mut self) -> Declarations {
        let should_be_left_embrace = self.get_cur_char();

        if should_be_left_embrace != '{' {
            panic!("[CSSOM] parse css failed! because in parse_declaration, should_ne_left_brace should be {{ but got {:?}", should_be_left_embrace);
        }

        let mut declarations: Declarations = HashMap::new();

        loop {
            self.goto_next_token();
            let cur_char = self.peek();

            if cur_char == '}' {
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

            if cur_char == ':' {
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

            if cur_char == ';' || cur_char == '}' {
                break;
            } else {
                value.push(cur_char as char);
            }
        }

        value
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
                    pseudo_elements: None
                }],
                declarations
            }]
        );
    }

    #[test]
    fn parser_works_with_pseudo_elements() {
        let css = "a:link { color: red; }";
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
                    elm: SelectorElm::TagName("a".to_string()),
                    children: vec![],

                    pseudo_elements: None
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
                    children: vec![SelectorChildren::ChildCombinator(vec![Selector {
                        elm: SelectorElm::TagName("div".to_string()),
                        children: vec![SelectorChildren::AdjacentSiblingCombinator(vec![
                            Selector {
                                elm: SelectorElm::TagName("p".to_string()),
                                children: vec![SelectorChildren::GeneralSiblingCombinator(vec![
                                    Selector {
                                        elm: SelectorElm::TagName("a".to_string()),
                                        children: vec![SelectorChildren::DescendantCombinator(
                                            vec![Selector {
                                                elm: SelectorElm::TagName("div".to_string()),
                                                children: vec![],
                                                pseudo_elements: None
                                            }]
                                        )],
                                        pseudo_elements: None
                                    }
                                ])],
                                pseudo_elements: None
                            }
                        ])],
                        pseudo_elements: None
                    }])],
                    pseudo_elements: None
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

                        pseudo_elements: None
                    },
                    Selector {
                        elm: SelectorElm::Class("class2".to_string()),
                        children: vec![],

                        pseudo_elements: None
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
                    pseudo_elements: None
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
                        pseudo_elements: None
                    }],
                    declarations: declarations_body.clone(),
                },
                StylingRule {
                    selector: vec![Selector {
                        elm: SelectorElm::TagName("p".to_string()),
                        children: vec![],
                        pseudo_elements: None
                    }],
                    declarations: declarations_p.clone(),
                }
            ]
        );
    }
}
