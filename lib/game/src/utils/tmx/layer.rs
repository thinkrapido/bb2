
use super::Node;

use std::rc::Rc;

#[derive(Debug, PartialEq, Eq)]
pub struct TmxLayer {
    pub name: Rc<String>,
    pub width: usize,
    pub height: usize,
    pub grid: Grid,
}

impl<'a> From<&'a Node> for TmxLayer {
    fn from(node: &Node) -> TmxLayer {

        let mut width = 0;
        let mut height = 0;
        let mut name = String::new();

        for (ref key, ref value) in &node.attributes {
            match key.as_ref() {
                "width" => {
                    width = value.parse::<usize>().unwrap();
                }
                "height" => {
                    height = value.parse::<usize>().unwrap();
                }
                "name" => {
                    name.clear();
                    name.push_str(value.clone());
                }
                _ => {}
            };
        }

        assert!(width > 0);
        assert!(height > 0);

        let mut grid = Grid::new(width, height);

        for (idx, str) in node.children[0].content.split(",").enumerate() {
            let col = idx % grid.width;
            let row = idx / grid.width;
            grid[row][col] = str.trim().parse::<usize>().unwrap();
        }

        TmxLayer {
            name: Rc::new(name),
            width,
            height,
            grid,
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct Grid {
    width: usize,
    data: Vec<usize>,
}

impl Grid {
    fn new(width: usize, height: usize) -> Grid {
        Grid {
            width,
            data: vec![0; width * height],
        }
    }
}

impl ::std::ops::Index<usize> for Grid {
    type Output = [usize];
    fn index(&self, row: usize) -> &[usize] {
        let start = row * self.width;
        &self.data[start .. start + self.width]
    }
}

impl ::std::ops::IndexMut<usize> for Grid {
    fn index_mut(&mut self, row: usize) -> &mut [usize] {
        let start = row * self.width;
        &mut self.data[start .. start + self.width]
    }
}
