
extern crate xml;

mod layer;
mod node;

use std::fs::File;
use std::io::{BufRead, BufReader, Read};
use std::path::Path;
use std::collections::HashMap;

use self::xml::reader::{ParserConfig};

use self::layer::*;
use self::node::*;

#[derive(Debug, PartialEq, Eq)]
pub struct TmxContent {
    pub entries: HashMap<String, TmxEntry>,
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

        Node::read_elements(&mut root, &mut reader);

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
                let layer = TmxLayer::new(&node);
                entries.insert(layer.name.clone(), TmxEntry::Layer(layer));
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
}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn read_tmx_file() {
        let file_name = "../../assets/maps/topworld.tmx";

        let tmx_content = TmxContent::from_file(file_name);

        let &TmxEntry::Layer(ref layer) = tmx_content.entries.get("Background_Layer").unwrap();
        let data = layer.grid[0][0];

        assert_eq!(data, 1605);
    }

}