
#[derive(Debug, Clone, PartialEq)]
pub struct Area {
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

impl Area {

    pub fn new(x: f32, y: f32, width: f32, height: f32) -> Area {
        let left = x;
        let top = y;
        let right = x + width;
        let bottom = y + height;
        Area {
            x, y, height, width, left, right, top, bottom,
        }
    }

    pub fn area(&self) -> f32 {
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
            Some(area) => area.area() > 0.0,
            None => false,
        }
    }

    pub fn contains(&self, point: &Point) -> bool {
        return point.0 >= self.left && point.0 <= self.right && point.1 >= self.top && point.1 <= self.bottom
    }

    pub fn enlarge(&self, value: f32) -> Area {
        let x = self.x - value;
        let y = self.y - value;
        let width = self.width + 2f32 * value;
        let height = self.height + 2f32 * value;
        Area::new(x, y, width, height)
    }

    pub fn shrink(&self, value: f32) -> Area {
        self.enlarge(-value)
    }

    pub fn scale(&self, value: f32) -> Area {
        let width = self.width * value;
        let height = self.height * value;
        let x = self.x - width / 2.0;
        let y = self.y - height / 2.0;
        Area::new(x, y, width, height)
    }

}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn area_new() {
        let area = Area::new(4.0, 5.0, 3.0, 8.0);
        assert_eq!(area.x, 4.0);
        assert_eq!(area.y, 5.0);
        assert_eq!(area.width, 3.0);
        assert_eq!(area.height, 8.0);
        assert_eq!(area.top, 5.0);
        assert_eq!(area.bottom, 13.0);
        assert_eq!(area.left, 4.0);
        assert_eq!(area.right, 7.0);
    }

    #[test]
    fn area_area() {
        let area = Area::new(4.0, 5.0, 3.0, 8.0);
        assert_eq!(area.area(), 24.0);
    }

    #[test]
    fn area_contains_point() {
        let area = Area::new(4.0, 5.0, 3.0, 8.0);
        
        let p = Point(1.0, 1.0);
        assert!(!area.contains(&p));
        
        let p = Point(5.0, 1.0);
        assert!(!area.contains(&p));
        
        let p = Point(11.0, 1.0);
        assert!(!area.contains(&p));
        
        let p = Point(1.0, 6.0);
        assert!(!area.contains(&p));
        
        let p = Point(5.0, 6.0);
        assert!(area.contains(&p));
        
        let p = Point(11.0, 6.0);
        assert!(!area.contains(&p));
        
        let p = Point(1.0, 14.0);
        assert!(!area.contains(&p));
        
        let p = Point(5.0, 14.0);
        assert!(!area.contains(&p));
        
        let p = Point(11.0, 14.0);
        assert!(!area.contains(&p));
    }

    #[test]
    fn area_collision() {
        let area = Area::new(4.0, 5.0, 3.0, 8.0);
        let other = Area::new(14.0, 15.0, 3.0, 8.0);
        assert_eq!(area.collision(&other), false);

        let other = Area::new(14.0, 15.0, 3.0, 8.0);
        assert_eq!(area.collision(&other), false);

        let other = Area::new(3.0, 4.0, 3.0, 8.0);
        assert_eq!(area.collision(&other), true);
    }

    #[test]
    fn area_enlarge() {
        let area = Area::new(4.0, 5.0, 3.0, 8.0);
        let enlarged = area.enlarge(0.5);
        let expected = Area::new(3.5, 4.5, 4.0, 9.0);
        assert_eq!(enlarged, expected);
    }

    #[test]
    fn area_shrink() {
        let area = Area::new(4.0, 5.0, 3.0, 8.0);
        let shrinked = area.shrink(0.5);
        let expected = Area::new(4.5, 5.5, 2.0, 7.0);
        assert_eq!(shrinked, expected);
    }


    #[test]
    fn area_scale() {
        let area = Area::new(4.0, 5.0, 4.0, 8.0);
        let scaled = area.scale(0.5);
        let expected = Area::new(3.0, 3.0, 2.0, 4.0);
        assert_eq!(scaled, expected);

        let scaled = area.scale(2.0);
        let expected = Area::new(0.0, -3.0, 8.0, 16.0);
        assert_eq!(scaled, expected);
    }
}