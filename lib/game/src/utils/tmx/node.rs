
extern crate xml;

use std::io::{Read};
use self::xml::reader::{XmlEvent, EventReader};
use std::collections::HashMap;

#[derive(Debug)]
pub struct Node {
    pub name: String,
    pub attributes: HashMap<String, String>,
    pub children: Vec<Node>,
    pub content: String,
}

pub fn read_node_elements<R: Read>(current: &mut Node, reader: &mut EventReader<R>) {
    loop {
        let e = reader.next();
        match e {
            Ok(XmlEvent::EndDocument) | Err(_) => break,
            Ok(XmlEvent::StartElement{ name, attributes, .. }) => {
                let mut map = HashMap::new();
                for ref attr in attributes.iter() {
                    map.insert(attr.name.local_name.clone(), attr.value.clone());
                }
                let mut node = Node{
                    name: name.local_name.clone(),
                    attributes: map,
                    children: Vec::new(),
                    content: "".to_string(),
                };
                read_node_elements(&mut node, reader);
                current.children.push(node);
            }
            Ok(XmlEvent::Characters(ref data)) => {
                current.content.push_str(data);
            }
            Ok(XmlEvent::EndElement{..}) => break,
            _ => {}
        }
    }
}

