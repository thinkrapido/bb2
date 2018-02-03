
#[derive(Debug, Clone, PartialEq)]
pub struct Tile {
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
    pub left: f32,
    pub right: f32,
    pub top: f32,
    pub bottom: f32,
}

pub struct Point(f32, f32);

impl Tile {

    pub fn new(x: f32, y: f32, width: f32, height: f32) -> Tile {
        let left = x;
        let top = y;
        let right = x + width;
        let bottom = y + height;
        Tile {
            x, y, height, width, left, right, top, bottom,
        }
    }

    pub fn area(&self) -> f32 {
        return self.width * self.height;
    }

    pub fn intersect(&self, other: &Tile) -> Option<Tile> {
        println!("{:?}\n{:?}\n", self, other);

        if  self.contains(&Point(other.top, other.left)) || 
            self.contains(&Point(other.top, other.right)) || 
            self.contains(&Point(other.bottom, other.left)) || 
            self.contains(&Point(other.bottom, other.right))
        {
            let left = self.left.max(other.left);
            let top = self.top.max(other.top);
            let right = self.right.min(other.right);
            let bottom = self.bottom.min(other.bottom);
            let x = left;
            let y = top;
            let width = right - left;
            let height = bottom - top;

            Some(Tile::new(x, y, width, height))
        } else {
            None
        }
    }

    pub fn collision(&self, other: &Tile) -> bool {
        match self.intersect(other) {
            Some(tile) => tile.area() > 0.0,
            None => false,
        }
    }

    pub fn contains(&self, point: &Point) -> bool {
        return point.0 >= self.left && point.0 <= self.right && point.1 >= self.top && point.1 <= self.bottom
    }

    pub fn enlarge(&self, value: f32) -> Tile {
        let x = self.x - value;
        let y = self.y - value;
        let width = self.width + 2f32 * value;
        let height = self.height + 2f32 * value;
        Tile::new(x, y, width, height)
    }

    pub fn shrink(&self, value: f32) -> Tile {
        self.enlarge(-value)
    }

    pub fn scale(&self, value: f32) -> Tile {
        let width = self.width * value;
        let height = self.height * value;
        let x = self.x - width / 2.0;
        let y = self.y - height / 2.0;
        Tile::new(x, y, width, height)
    }

}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn tile_new() {
        let tile = Tile::new(4.0, 5.0, 3.0, 8.0);
        assert_eq!(tile.x, 4.0);
        assert_eq!(tile.y, 5.0);
        assert_eq!(tile.width, 3.0);
        assert_eq!(tile.height, 8.0);
        assert_eq!(tile.top, 5.0);
        assert_eq!(tile.bottom, 13.0);
        assert_eq!(tile.left, 4.0);
        assert_eq!(tile.right, 7.0);
    }

    #[test]
    fn tile_area() {
        let tile = Tile::new(4.0, 5.0, 3.0, 8.0);
        assert_eq!(tile.area(), 24.0);
    }

    #[test]
    fn tile_contains_point() {
        let tile = Tile::new(4.0, 5.0, 3.0, 8.0);
        
        let p = Point(1.0, 1.0);
        assert!(!tile.contains(&p));
        
        let p = Point(5.0, 1.0);
        assert!(!tile.contains(&p));
        
        let p = Point(11.0, 1.0);
        assert!(!tile.contains(&p));
        
        let p = Point(1.0, 6.0);
        assert!(!tile.contains(&p));
        
        let p = Point(5.0, 6.0);
        assert!(tile.contains(&p));
        
        let p = Point(11.0, 6.0);
        assert!(!tile.contains(&p));
        
        let p = Point(1.0, 14.0);
        assert!(!tile.contains(&p));
        
        let p = Point(5.0, 14.0);
        assert!(!tile.contains(&p));
        
        let p = Point(11.0, 14.0);
        assert!(!tile.contains(&p));
    }

    #[test]
    fn tile_collision() {
        let tile = Tile::new(4.0, 5.0, 3.0, 8.0);
        let other = Tile::new(14.0, 15.0, 3.0, 8.0);
        assert_eq!(tile.collision(&other), false);

        let other = Tile::new(14.0, 15.0, 3.0, 8.0);
        assert_eq!(tile.collision(&other), false);

        let other = Tile::new(3.0, 4.0, 3.0, 8.0);
        assert_eq!(tile.collision(&other), true);
    }

    #[test]
    fn tile_enlarge() {
        let tile = Tile::new(4.0, 5.0, 3.0, 8.0);
        let enlarged = tile.enlarge(0.5);
        let expected = Tile::new(3.5, 4.5, 4.0, 9.0);
        assert_eq!(enlarged, expected);
    }

    #[test]
    fn tile_shrink() {
        let tile = Tile::new(4.0, 5.0, 3.0, 8.0);
        let shrinked = tile.shrink(0.5);
        let expected = Tile::new(4.5, 5.5, 2.0, 7.0);
        assert_eq!(shrinked, expected);
    }


    #[test]
    fn tile_scale() {
        let tile = Tile::new(4.0, 5.0, 4.0, 8.0);
        let scaled = tile.scale(0.5);
        let expected = Tile::new(3.0, 3.0, 2.0, 4.0);
        assert_eq!(scaled, expected);

        let scaled = tile.scale(2.0);
        let expected = Tile::new(0.0, -3.0, 8.0, 16.0);
        assert_eq!(scaled, expected);
    }
}