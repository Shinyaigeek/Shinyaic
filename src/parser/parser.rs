use std::vec::Vec;
use std::collections::HashMap;

pub struct Parser {
    pub pos: usize,
    pub input: String,
}

impl Parser {
    pub fn parse(&mut self) -> DOMNode { // 渡されるもの: <head></head><body><div>hello</div></body>
         let program = self.parse_node();
         DOMNode::elem("html".to_string(), HashMap::new(), vec![program])
    }

    fn eat_opening_tag(&mut self) -> (String, HashMap<String, String>) { // `<`li id="id1" `>`</li> -> li
        let init_pos = self.input.as_bytes()[self.pos];
        if init_pos != b'<' {
            panic!("pick_tag_name was invoked not in < but in {:?}", init_pos)
        }

        self.eat();

        let mut tag : Vec<u8> = vec![];

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

        (String::from_utf8(tag).unwrap(), attributes)
    }

    fn eat_element_attributes(&mut self) -> HashMap<String, String> { // <li ` `id="asdf"`>`
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
                }else{
                    value.push(next_token as char);
                }
            }else{ 
                if next_token == b'>' {
                    break;
                }
                if next_token == b'"' {
                    is_eating_key = false;
                }else if next_token == b' ' || next_token == b'=' {
                    // do nothing
                }else{
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

    fn peek_start_with(&self ,value: String) -> bool {
        let mut token = String::from("");
        self.input[self.pos..].starts_with(&value)
    }

    fn go_to_next_left_tag(&mut self) {

    }

    fn parse_text(&mut self) -> DOMNode {
        let mut text = String::from("");

        println!("------------------------");
        println!("{:?}", self.input.as_bytes()[self.pos] as char);
        println!("{:?}", self.input.as_bytes()[self.pos + 1] as char);
        println!("{:?}", self.input.as_bytes()[self.pos + 2] as char);
        println!("{:?}", self.peek_start_with("<".to_string()));
        println!("------------------------");

        while !self.peek_start_with("<".to_string()) {
            let c = self.eat();
            text.push(c as char);
        }

        DOMNode::text(text)
    }

    fn consume_comment(&mut self) {

    }    

    fn parse_node(&mut self) -> DOMNode { // `<`li></LI`>`
        if self.input.as_bytes()[self.pos] != b'<' {
            // text node
            return self.parse_text()
        }
        let (target_tag_name, attributes) = self.eat_opening_tag();
        let node = DOMNode::elem(target_tag_name.clone(), attributes, self.parse_nodes());

        // eat_closing_tag();
        self.pos += 3 + target_tag_name.len();
        println!("aaa{:?}", self.pos);
        println!("aaa{:?}", node);
        node
    }

    fn parse_nodes(&mut self) -> Vec<DOMNode> { // input: <head></head><body><div>hello</div></body>    <ul>`<`li>1</li><li>1</li><li>1</li></ul>
        let mut nodes: Vec<DOMNode> = vec![];
        println!("hohohohoh");

        println!("{:?}, {:?}", self.peek_start_with("</".to_string()), self.pos);
        
        // 終了条件: eof or </
        while !self.peek_start_with("</".to_string()) && !(self.input.len() <= self.pos) {
            println!("node: {:?}", nodes);
            nodes.push(self.parse_node());
            println!("node: {:?}", nodes);
            if nodes.len() > 2 {
                // panic!("debugger")
            }
        }

        nodes

    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum NodeType {
    text_node(String),
    dom_node(ElementType)
}

#[derive(Debug, PartialEq, Clone)]
pub struct DOMNode {
    children: Vec<DOMNode>,
    node_type: NodeType,
}

impl DOMNode {
    pub fn text(value: String) -> Self {
        Self {
            children: vec![],
            node_type: NodeType::text_node(value)
        }
    }

    pub fn elem(tag_name: String, attributes: HashMap<String, String>, children: Vec<DOMNode>) -> Self {
        Self {
            children,
            node_type: NodeType::dom_node(ElementType {
                tag_name,
                attributes
            })
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct ElementType {
    // TODO できればenum
    tag_name: String,
    attributes: HashMap<String, String>
}

impl ElementType {
    pub fn get_attributes_value(&self ,key: String) -> String {
        if self.attributes.get(&key).is_none() {
            return self.attributes.get(&key).unwrap().to_string()
        }

        "".to_string()
    }
}