
use ::noisy_float::prelude::*;

use std::collections::HashMap;
use std::rc::Rc;

use super::node::Node;

#[derive(Debug, PartialEq, Eq)]
pub struct Property {
    pub name: Rc<String>,
    pub value: PropertyEnum,
}

#[derive(Debug, PartialEq, Eq)]
pub enum PropertyEnum {
    Float(R32),
    String(String),
}

impl Property {
    pub fn properties_from_node(node: &Node) -> HashMap<Rc<String>, Property> {

        let mut properties = HashMap::new();

        for node in node.children.iter() {
            match node.name.as_ref() {
                "properties" => {
                    for node in node.children.iter() {
                        match node.name.as_ref() {
                            "property" => {
                                let mut name = String::new();
                                let mut type_ = String::from("string");
                                let mut value = String::new();

                                for (ref k, ref v) in &node.attributes {
                                    match k.as_ref() {
                                        "name" => {
                                            name.push_str(v.clone());
                                        }
                                        "type" => {
                                            type_.clear();
                                            type_.push_str(v.clone());
                                        }
                                        "value" => {
                                            value.push_str(v.clone());
                                        }
                                        _ => {}
                                    }
                                }
                                let property = Property {
                                    name: Rc::new(name),
                                    value: match type_.as_ref() {
                                        "string" => PropertyEnum::String(value),
                                        "float" => PropertyEnum::Float(r32(value.parse::<f32>().unwrap())),
                                        _ => panic!("no property type match"),
                                    }
                                };
                                properties.insert(property.name.clone(), property);
                            }
                            _ => {}
                        }
                    }
                }
                _ => {}
            }
        }

        properties
    }
}