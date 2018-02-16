
extern crate noisy_float;

use self::noisy_float::prelude::*;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Area {
    pub x: R32,
    pub y: R32,
    pub width: R32,
    pub height: R32,
    pub left: R32,
    pub right: R32,
    pub top: R32,
    pub bottom: R32,
}

pub struct Point(R32, R32);

impl Area {

    pub fn new(x: R32, y: R32, width: R32, height: R32) -> Area {
        let left = x;
        let top = y;
        let right = x + width;
        let bottom = y + height;
        Area {
            x, y, height, width, left, right, top, bottom,
        }
    }

    pub fn area(&self) -> R32 {
        return self.width * self.height;
    }

    pub fn intersect(&self, other: &Area) -> Option<Area> {
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

            Some(Area::new(x, y, width, height))
        } else {
            None
        }
    }

    pub fn collision(&self, other: &Area) -> bool {
        match self.intersect(other) {
            Some(area) => area.area() > r32(0.0),
            None => false,
        }
    }

    pub fn contains(&self, point: &Point) -> bool {
        return point.0 >= self.left && point.0 <= self.right && point.1 >= self.top && point.1 <= self.bottom
    }

    pub fn enlarge(&self, value: R32) -> Area {
        let x = self.x - value;
        let y = self.y - value;
        let width = self.width + r32(2.0) * value;
        let height = self.height + r32(2.0) * value;
        Area::new(x, y, width, height)
    }

    pub fn shrink(&self, value: R32) -> Area {
        self.enlarge(-value)
    }

    pub fn scale(&self, value: R32) -> Area {
        let width = self.width * value;
        let height = self.height * value;
        let x = self.x - width / r32(2.0);
        let y = self.y - height / r32(2.0);
        Area::new(x, y, width, height)
    }

}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn area_new() {
        let area = Area::new(r32(4.0), r32(5.0), r32(3.0), r32(8.0));
        assert_eq!(area.x, r32(4.0));
        assert_eq!(area.y, r32(5.0));
        assert_eq!(area.width, r32(3.0));
        assert_eq!(area.height, r32(8.0));
        assert_eq!(area.top, r32(5.0));
        assert_eq!(area.bottom, r32(13.0));
        assert_eq!(area.left, r32(4.0));
        assert_eq!(area.right, r32(7.0));
    }

    #[test]
    fn area_area() {
        let area = Area::new(r32(4.0), r32(5.0), r32(3.0), r32(8.0));
        assert_eq!(area.area(), r32(24.0));
    }

    #[test]
    fn area_contains_point() {
        let area = Area::new(r32(4.0), r32(5.0), r32(3.0), r32(8.0));
        
        let p = Point(r32(1.0), r32(1.0));
        assert!(!area.contains(&p));
        
        let p = Point(r32(5.0), r32(1.0));
        assert!(!area.contains(&p));
        
        let p = Point(r32(11.0), r32(1.0));
        assert!(!area.contains(&p));
        
        let p = Point(r32(1.0), r32(6.0));
        assert!(!area.contains(&p));
        
        let p = Point(r32(5.0), r32(6.0));
        assert!(area.contains(&p));
        
        let p = Point(r32(11.0), r32(6.0));
        assert!(!area.contains(&p));
        
        let p = Point(r32(1.0), r32(14.0));
        assert!(!area.contains(&p));
        
        let p = Point(r32(5.0), r32(14.0));
        assert!(!area.contains(&p));
        
        let p = Point(r32(11.0), r32(14.0));
        assert!(!area.contains(&p));
    }

    #[test]
    fn area_collision() {
        let area = Area::new(r32(4.0), r32(5.0), r32(3.0), r32(8.0));
        let other = Area::new(r32(14.0), r32(15.0), r32(3.0), r32(8.0));
        assert_eq!(area.collision(&other), false);

        let other = Area::new(r32(14.0), r32(15.0), r32(3.0), r32(8.0));
        assert_eq!(area.collision(&other), false);

        let other = Area::new(r32(3.0), r32(4.0), r32(3.0), r32(8.0));
        assert_eq!(area.collision(&other), true);
    }

    #[test]
    fn area_enlarge() {
        let area = Area::new(r32(4.0), r32(5.0), r32(3.0), r32(8.0));
        let enlarged = area.enlarge(r32(0.5));
        let expected = Area::new(r32(3.5), r32(4.5), r32(4.0), r32(9.0));
        assert_eq!(enlarged, expected);
    }

    #[test]
    fn area_shrink() {
        let area = Area::new(r32(4.0), r32(5.0), r32(3.0), r32(8.0));
        let shrinked = area.shrink(r32(0.5));
        let expected = Area::new(r32(4.5), r32(5.5), r32(2.0), r32(7.0));
        assert_eq!(shrinked, expected);
    }


    #[test]
    fn area_scale() {
        let area = Area::new(r32(4.0), r32(5.0), r32(4.0), r32(8.0));
        let scaled = area.scale(r32(0.5));
        let expected = Area::new(r32(3.0), r32(3.0), r32(2.0), r32(4.0));
        assert_eq!(scaled, expected);

        let scaled = area.scale(r32(2.0));
        let expected = Area::new(r32(0.0), -r32(3.0), r32(8.0), r32(16.0));
        assert_eq!(scaled, expected);
    }
}