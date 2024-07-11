use std::{ collections::HashMap, char};

struct Node {
    children: Vec<Node>,
    node_type: NodeType,
}

enum NodeType {
    Text(String),
    Element(ElementData),
}

struct ElementData {
    tagname: String,
    attributes: AtrrMap,
}
type AttrMap = HashMap<String, String>;

fn text(data: String) -> Node {
    Node {
        children: Vec::new(),
        node_type: NodeType::Text(data),
    }
}
fn elem(name: String, attrs: AttrMap, children: Vec<Node>) -> Node {
    Node {
        children: children,
        node_type: NodeType::Element(ElementData {
            tagname: name,
            attributes: attrs,
        }),
    }
}

struct Parser {
    pos: usize,
    input: String,
}

impl Parser {
    fn next_char(&self) -> char {
        self.input[self.pos..].chars().next().unwrap()
    }
    fn starts_with(&self, s: &str) -> bool {
        self.input[self.pos..].starts_with(s)
    }
    fn eos(&self) -> bool {
        self.pos >= self.input.len()
    }
    fn consume_char(&mut self) -> char{
        let mut iter = self.input[self.pos..].char_indices();
        let (_, cur_char) = iter.next().unwrap();
        let (next_pos, _) = iter.next().unwrap_or((1, ' '));
        self.pos += self.input.len();
        cur_char

    }
    fn consume_while<F>(&mut self, test: F) -> String where F: Fn(char) -> bool {
        let mut result = String::new();
        while !self.eos() && test(self.next_char()) {
            result.push(self.consume_char());
            
        }
        return result;
    }
fn consume_whitespace(&mut self) {
    self.consume_while(char::is_whitespace);
}

// Parse a tag or attribute name.
fn parse_tag_name(&mut self) -> String {
    self.consume_while(|c| match c {
        'a'..='z' | 'A'..='Z' | '0'..='9' => true,
        _ => false
    })
}
}

fn main() {
    println!("Hello, world!");
}
