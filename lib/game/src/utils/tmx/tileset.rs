extern crate noisy_float;
use self::noisy_float::prelude::*;

use super::Node;

use std::collections::HashMap;
use std::rc::Rc;

#[derive(Debug, PartialEq, Eq)]
pub struct TmxTileset {
    pub name: Rc<String>,
    pub columns: usize,
    pub rows: usize,
    pub tilewidth: usize,
    pub tileheight: usize,
    pub tiles: HashMap<Rc<usize>, Tile>,
    pub image_file_name: String,
}

impl<'a> From<&'a Node> for TmxTileset {
    fn from(node: &'a Node) -> TmxTileset {

        let mut name = String::new();
        let mut image_file_name = String::new();
        let mut columns = 0;
        let mut tilecount = 0;
        let mut tilewidth = 0;
        let mut tileheight = 0;
        let mut firstgid = 0;

        for (ref key, ref value) in &node.attributes {
            match key.as_ref() {
                "name" => {
                    name.clear();
                    name.push_str(value.clone());
                }
                "firstgid" => {
                    firstgid = value.parse::<usize>().unwrap();
                }
                "columns" => {
                    columns = value.parse::<usize>().unwrap();
                }
                "tilecount" => {
                    tilecount = value.parse::<usize>().unwrap();
                }
                "tilewidth" => {
                    tilewidth = value.parse::<usize>().unwrap();
                }
                "tileheight" => {
                    tileheight = value.parse::<usize>().unwrap();
                }
                _ => {}
            };
        }

        let mut tiles = HashMap::new();

        for ref node in node.children.iter() {
            match node.name.as_ref() {
                "image" => {
//                    let mut width = 0;
//                    let mut height = 0;
                    for (ref k, ref v) in &node.attributes {
                        match k.as_ref() {
                            "source" => {
                                image_file_name = v.clone().to_string();
                            }
/*
                             "width" => {
                                width = v.parse::<usize>().unwrap();
                            }
                            "height" => {
                                height = v.parse::<usize>().unwrap();
                            }
 */
                            _ => {}
                        };
                    }
                }
                "tile" => {
                    for (ref k, ref v) in &node.attributes {
                        match k.as_ref() {
                            "id" => {
                                let mut tile = Tile::new(firstgid + v.parse::<usize>().unwrap()); 
                                tile.properties = get_properties(node);
                                tiles.insert(tile.id.clone(), tile);
                            }
                            _ => {}
                        };
                    }
                }
                _ => {}
            }
        }

        TmxTileset {
            name: Rc::new(name),
            columns,
            rows: tilecount / columns,
            tiles,
            tilewidth,
            tileheight,
            image_file_name,
        }
    }
}

fn get_properties(node: &Node) -> HashMap<Rc<String>, Property> {

    let mut properties = HashMap::new();

    for node in node.children.iter() {
        match node.name.as_ref() {
            "properties" => {
                for node in node.children.iter() {
                    match node.name.as_ref() {
                        "property" => {
                            let mut name = String::new();
                            let mut type_ = String::new();
                            let mut value = String::new();

                            for (ref k, ref v) in &node.attributes {
                                match k.as_ref() {
                                    "name" => {
                                        name.push_str(v.clone());
                                    }
                                    "type" => {
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

#[derive(Debug, PartialEq, Eq)]
pub struct Tile {
    pub id: Rc<usize>,
    pub properties: HashMap<Rc<String>, Property>,
}


impl Tile {
    pub fn new(id: usize) -> Tile{
        Tile {
            id: Rc::new(id),
            properties: HashMap::new(),
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct Property {
    pub name: Rc<String>,
    pub value: PropertyEnum,
}

#[derive(Debug, PartialEq, Eq)]
pub enum PropertyEnum {
    Float(R32),
}

