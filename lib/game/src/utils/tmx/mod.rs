extern crate noisy_float;

extern crate xml;

mod layer;
mod tileset;
mod node;

use std::fs::File;
use std::io::{Read};
use std::path::Path;
use std::collections::HashMap;
use std::rc::Rc;
use self::xml::reader::{ParserConfig};

use self::layer::*;
use self::tileset::*;
use self::node::*;

#[derive(Debug, PartialEq, Eq)]
pub struct TmxContent {
    pub entries: HashMap<Rc<String>, TmxEntry>,
}

impl TmxContent {

    pub fn from_file(file_name: &str) -> TmxContent {

        let path = Path::new(file_name);
        let mut f = File::open(path).expect("tmx file could not be opend");
        let mut buffer = Vec::<u8>::new();

        f.read_to_end(&mut buffer).expect("tmx file could not be read");

        let mut reader = parser_config().create_reader(&buffer[..]);

        let mut root = Node {
            name: "root".to_string(),
            attributes: HashMap::new(),
            children: Vec::new(),
            content: "".to_string(),
        };

        read_node_elements(&mut root, &mut reader);

        create_tmx_content(& root)
    }

}

fn parser_config() -> ParserConfig {
    ParserConfig::new()
        .ignore_comments(true)
        .trim_whitespace(true)
        .coalesce_characters(true)
        .whitespace_to_characters(true)
}

fn create_tmx_content(root: &Node) -> TmxContent {

    let mut entries = HashMap::new();

    for node in root.children[0].children.iter() {
        match node.name.as_ref() {
            "layer" => {
                let layer = TmxLayer::from(node);
                entries.insert(layer.name.clone(), TmxEntry::Layer(layer));
            }
            "tileset" => {
                let tileset = TmxTileset::from(node);
                entries.insert(tileset.name.clone(), TmxEntry::Tileset(tileset));
            }
            _ => {}
        }
    }

    TmxContent {
        entries,
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum TmxEntry {
    Layer(TmxLayer),
    Tileset(TmxTileset),
}

#[cfg(test)]
mod test {

    use super::noisy_float::*;
    use super::*;

    #[test]
    fn read_tmx_file() {
        let file_name = "../../assets/maps/topworld.tmx";

        let tmx_content = TmxContent::from_file(file_name);

        handle_tmx_entry(tmx_content.entries.get(&"Background_Layer".to_string()).unwrap());
        handle_tmx_entry(tmx_content.entries.get(&"Floor".to_string()).unwrap());
    }

    fn handle_tmx_entry(entry: &TmxEntry) {
        match entry {
            &TmxEntry::Layer(ref layer) => {
                let data = layer.grid[0][0];
                assert_eq!(data, 1605);
            }
            &TmxEntry::Tileset(ref tileset) => {
                //let PropertyEnum::Float(got) = tileset.tiles.get(&1161).unwrap().properties.get(&"Penalty".to_string()).unwrap().value;
                let got = tileset.property(1161, "Penalty");
                let should_be = prelude::r32(1.2);

                match got {
                    Some(&PropertyEnum::Float(f)) => assert_eq!(f, should_be),
                    _ => {}
                };
            }
        }
    }

}