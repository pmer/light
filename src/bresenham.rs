use geometry::*;

// Octants of a circle if said circle were drawn starting at 3 PM and moving counter-clockwise.
#[derive(Copy, Clone, Debug)]
pub enum Octant {
    One, // 0 to π/4
    Two, // π/4 to π/2
    Three, // etc.
    Four,
    Five,
    Six,
    Seven,
    Eight,
}

// Bresenham’s line algorithm
pub fn bresenham(a: Point, b: Point) -> Vec<Point> {
    let mut line = vec![];
    let dx = b.x - a.x;
    let dy = b.y - a.y;

    // If the line is vertical.
    if dx == 0 {
        let x = a.x;
        let top = if a.y < b.y { a.y } else { b.y };
        let bot = if a.y < b.y { b.y } else { a.y };
        for y in top..bot {
            line.push(Point { x, y });
        }
        return line;
    }

    let octant = determine_octant(Point { x: dx, y: dy });
    let begin = to_first_octant(octant, a);
    let end = to_first_octant(octant, b);
    let m = (end.y - begin.y) as f32 / (end.x - begin.x) as f32;

    let mut y = begin.y;
    let mut err = 0.5f32;

    for x in begin.x..end.x {
        line.push(from_first_octant(octant, Point { x, y }));
        err += m;
        if err > 1.0f32 {
            err -= 1.0f32;
            y += 1;
        }
    }

    line
}

fn determine_octant(p: Point) -> Octant {
    let Point { x, y } = p;

    // Incorrect when x==y for Five/Six and Seven/Eight, but good enough for our purposes.
    if x >= 0 {
        if y >= 0 {
            if x > y { Octant::One } else { Octant::Two }
        } else {
            if x > -y { Octant::Eight } else { Octant::Seven }
        }
    } else {
        if y >= 0 {
            if -x > y { Octant::Four } else { Octant::Three }
        } else {
            if -x > -y { Octant::Five } else { Octant::Six }
        }
    }
}

fn to_first_octant(octant: Octant, p: Point) -> Point {
    let Point { x, y } = p;
    match octant {
        Octant::One => p,
        Octant::Two => Point { x: y, y: x },
        Octant::Three => Point { x: y, y: -x },
        Octant::Four => Point { x: -x, y: y },
        Octant::Five => Point { x: -x, y: -y },
        Octant::Six => Point { x: -y, y: -x },
        Octant::Seven => Point { x: -y, y: x },
        Octant::Eight => Point { x: x, y: -y },
    }
}

fn from_first_octant(octant: Octant, p: Point) -> Point {
    let Point { x, y } = p;
    match octant {
        Octant::One => p,
        Octant::Two => Point { x: y, y: x },
        Octant::Three => Point { x: -y, y: x },
        Octant::Four => Point { x: -x, y: y },
        Octant::Five => Point { x: -x, y: -y },
        Octant::Six => Point { x: -y, y: -x },
        Octant::Seven => Point { x: y, y: -x },
        Octant::Eight => Point { x: x, y: -y },
    }
}
