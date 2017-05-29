use std::mem;

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}

impl Point {
    pub fn magnitude(&self) -> f32 {
        ((self.x * self.x + self.y * self.y) as f32).sqrt()
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Dimensions {
    pub width: i32,
    pub height: i32,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Rect {
    pub x: i32,
    pub y: i32,
    pub w: i32,
    pub h: i32,
}

impl Rect {
    pub fn from_points(mut a: Point, mut b: Point) -> Rect {
        if b.x < a.x {
            mem::swap(&mut a.x, &mut b.x);
        }
        if b.y < a.y {
            mem::swap(&mut a.y, &mut b.y);
        }
        Rect {
            x: a.x,
            y: a.y,
            w: b.x - a.x,
            h: b.y - a.y,
        }
    }

    pub fn area(&self) -> i32 {
        self.w * self.h
    }

    pub fn contains(&self, point: Point) -> bool {
        self.x <= point.x && self.y <= point.y && point.x < self.x + self.w &&
        point.y < self.y + self.h
    }
}
