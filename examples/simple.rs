extern crate rand;
use rand::{thread_rng, Rng};

extern crate quadtree;
use quadtree::{Spacial, Quadtree, BoundingBox};


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
    let mut n = 10_000;
    //for _ in 0..2 {
    println!("inserting {} items", n);
    for p in pts.iter().take(n) {
        qt.insert(p);
    }
    println!("counting subtrees");
    println!("{} items, {} trees", n, qt.tot_trees());
    //qt.clear();
    n *= 10;
    //}

    //std::thread::sleep_ms(10_000);
    let sb = BoundingBox {
        x: 0.0,
        y: 0.0,
        w: 5.0,
        h: 5.0,
    };
    let found = qt.find(&sb);
    println!("{:#?}, {}", found, found.len());
    //println!("{:#?}", qt);
}

#[derive(Debug)]
struct Point {
    x: f64,
    y: f64,
}
impl Spacial for Point {
    fn x(&self) -> f64 {
        self.x
    }
    fn y(&self) -> f64 {
        self.y
    }
}
