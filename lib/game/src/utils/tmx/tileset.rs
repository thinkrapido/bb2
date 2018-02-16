
use ::noisy_float::*;
use ::noisy_float::prelude::*;

use super::Node;
use super::property::*;

use std::collections::HashMap;
use std::rc::Rc;

#[derive(Debug, PartialEq, Eq)]
pub struct TmxTileset {
    pub name: Rc<String>,
    pub columns: usize,
    pub rows: usize,
    pub tilewidth: usize,
    pub tileheight: usize,
    pub tiles: HashMap<usize, Tile>,
    pub image_file_name: String,
}

impl TmxTileset {

    pub fn tile(&self, id: usize) -> Option<&Tile> {
        self.tiles.get(&id)
    }

    pub fn property(&self, id: usize, name: &str) -> Option<&PropertyEnum> {
        if let Some(tile) = self.tile(id) {
            if let Some(property) = tile.properties.get(&Rc::new(name.to_string())) {
                Some(&property.value);
            }
        }

        None
    }

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
                                tile.properties = Property::properties_from_node(node);
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

#[derive(Debug, PartialEq, Eq)]
pub struct Tile {
    pub id: usize,
    pub properties: HashMap<Rc<String>, Property>,
}

impl Tile {
    pub fn new(id: usize) -> Tile{
        Tile {
            id,
            properties: HashMap::new(),
        }
    }
}

