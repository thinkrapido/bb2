
use ::noisy_float::prelude::*;

use std::collections::HashMap;
use std::rc::Rc;

use ::utils::tmx::property::*;
use ::world::Area;

use super::node::Node;

#[derive(Debug, PartialEq, Eq)]
pub struct TmxObjectGroup {
    pub name: Rc<String>,
    pub objects: HashMap<usize, Object>,
}

#[derive(Debug, PartialEq, Eq)]
pub struct Object {
    pub id: usize,
    pub name: String,
    pub area: Area,
    pub properties: HashMap<Rc<String>, Property>,
}

impl<'a> From<&'a Node> for TmxObjectGroup {
    fn from(node: &'a Node) -> TmxObjectGroup {

        let mut name = String::new();
        let mut objects = HashMap::new();

        for (ref key, ref value) in &node.attributes {
            match key.as_ref() {
                "name" => {
                    name.clear();
                    name.push_str(value.clone());
                }
                _ => {}
            }
        }

        for ref node in node.children.iter() {
            match node.name.as_ref() {
                "object" => {
                    let o = Object::from(*node);
                    objects.insert(o.id, o);
                }
                _ => {}
            }
        }

        TmxObjectGroup {
            name: Rc::new(name),
            objects,
        }
    }
}

impl<'a> From<&'a Node> for Object {
    fn from(node: &'a Node) -> Object {
        let mut id = 0;
        let mut name = String::new();
        let mut x = r32(0.0);
        let mut y = r32(0.0);
        let mut width = r32(0.0);
        let mut height = r32(0.0);

        for (ref key, ref value) in &node.attributes {
            match key.as_ref() {
                "id" => {
                    id = value.parse::<usize>().unwrap();
                }
                "name" => {
                    name.clear();
                    name.push_str(value.clone());
                }
                "x" => {
                    x = r32(value.parse::<f32>().unwrap());
                }
                "y" => {
                    y = r32(value.parse::<f32>().unwrap());
                }
                "width" => {
                    width = r32(value.parse::<f32>().unwrap());
                }
                "height" => {
                    height = r32(value.parse::<f32>().unwrap());
                }
                _ => {}
            }
        }

        let properties = Property::properties_from_node(node);

        Object {
            id,
            name,
            area: Area::new(x, y, width, height),
            properties,
        }
    }
}