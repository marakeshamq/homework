#[derive(Debug)]
struct Point {
    x: i32,
    y: i32,
}

#[derive(Debug)]
struct Rectangle {
    p1: Point,
    p2: Point,
}

impl Rectangle {
    fn left(&self) -> i32 {
        self.p1.x.min(self.p2.x)
    }
    fn right(&self) -> i32 {
        self.p1.x.max(self.p2.x)
    }
    fn top(&self) -> i32 {
        self.p1.y.max(self.p2.y)
    }
    fn bottom(&self) -> i32 {
        self.p1.y.min(self.p2.y)
    }

    fn area(&self) -> i32 {
        let width = self.right() - self.left();
        let height = self.top() - self.bottom();
        if width > 0 && height > 0 {
            width * height
        } else {
            0
        }
    }

    fn intersection(&self, other: &Rectangle) -> Option<Rectangle> {
        let left = self.left().max(other.left());
        let right = self.right().min(other.right());
        let top = self.top().min(other.top());
        let bottom = self.bottom().max(other.bottom());

        if right > left && top > bottom {
            Some(Rectangle {
                p1: Point { x: left, y: top },
                p2: Point { x: right, y: bottom },
            })
        } else {
            None
        }
    }
}

fn total_area(rects: &[Rectangle]) -> i32 {
    let mut total = 0;
    for (i, rect) in rects.iter().enumerate() {
        total += rect.area();
        for j in 0..i {
            if let Some(intersect) = rect.intersection(&rects[j]) {
                total -= intersect.area();
            }
        }
    }
    total
}

fn test_data() -> Vec<Rectangle> {
    vec![
        Rectangle {
            p1: Point { x: 2, y: 9 },
            p2: Point { x: 5, y: 3 },
        },
        Rectangle {
            p1: Point { x: 1, y: 8 },
            p2: Point { x: 11, y: 6 },
        },
        Rectangle {
            p1: Point { x: 9, y: 10 },
            p2: Point { x: 13, y: 2 },
        },
    ]
}

fn main() {
    let rects = test_data();
    let area = total_area(&rects);
    println!("Загальна площа з урахуванням перетинів: {}", area);
}
