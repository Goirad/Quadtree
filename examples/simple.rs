extern crate rand;
use rand::{thread_rng, Rng};

extern crate quadtree;
use quadtree::{Boundable, BoundingBox, Quadtree};

fn main() {
    println!("Hello, world!");
    let bb = BoundingBox {
        x: 0.0,
        y: 0.0,
        w: 50.0,
        h: 50.0,
    };
    let mut rng = thread_rng();

    println!("generating points");
    let mut pts = vec![];
    let mut qt: Quadtree<Point> = Quadtree::new(bb, 10);
    for _ in 0..10_000 {
        pts.push(Point {
            x: rng.gen::<f64>() * 100.0 - 50.0,
            y: rng.gen::<f64>() * 100.0 - 50.0,
        });
    }
    println!("done creating");

    let n = 10_000;
    println!("inserting {} items", n);

    for p in pts.iter().take(n) {
        qt.insert(p);
    }

    println!("counting subtrees");
    println!("{} items, {} trees", n, qt.total_trees());

    let sb = BoundingBox {
        x: 0.0,
        y: 0.0,
        w: 25.0,
        h: 25.0,
    };
    let found = qt.find(&sb);
    println!("found {}", found.len());
}

#[derive(Debug)]
struct Point {
    x: f64,
    y: f64,
}
impl Boundable for Point {
    fn bounds(&self) -> BoundingBox {
        BoundingBox {
            x: self.x,
            y: self.y,
            w: 0.0,
            h: 0.0,
        }
    }
}
