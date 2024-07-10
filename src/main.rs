use std::collections::HashMap;

struct Node {
    children: Vec<Node>,
    node_type: NodeType,
}

enum NodeType {
    Text(String),
    Element(ElementData)
}

struct ElementData {
    tagname: String,
    attributes: AtrrMap,
    
}
type AttrMap = HashMap<String, String>;

fn text(data: String) -> Node {
    Node {children: Vec::new(), node_type: NodeType::Text(data)}
}
fn elem(name: String, attrs: AttrMap, children: Vec<Node>) -> Node{
    Node {
        children: children,
        node_type: NodeType::Element(ElementData { tagname: name, attributes: attrs })
    }
}


fn main() {
    println!("Hello, world!");
}
