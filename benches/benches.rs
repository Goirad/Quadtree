#![feature(test)]
extern crate test;
extern crate rand;
extern crate quadtree;

use quadtree::{Boundable, BoundingBox, Quadtree};

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

#[cfg(test)]
mod benches {
    use rand::{Rng, SeedableRng, StdRng};

    use super::*;
    use test::{black_box, Bencher};

    #[bench]
    fn insert_1_000(b: &mut Bencher) {
        let bb = BoundingBox {
            x: 0.0,
            y: 0.0,
            w: 50.0,
            h: 50.0,
        };
        let mut rng = StdRng::from_seed(&[24, 25, 12]);
        let mut pts = vec![];
        let mut qt: Quadtree<Point> = Quadtree::new(bb, 10);
        for _ in 0..1_000 {
            pts.push(Point {
                x: rng.gen::<f64>() * 100.0 - 50.0,
                y: rng.gen::<f64>() * 100.0 - 50.0,
            });
        }

        b.iter(|| {
            black_box({
                for p in pts.iter() {
                    qt.insert(p);
                }
                qt.clear();
            });
        });
    }
    #[bench]
    fn insert_10_000(b: &mut Bencher) {
        let bb = BoundingBox {
            x: 0.0,
            y: 0.0,
            w: 50.0,
            h: 50.0,
        };
        let mut rng = StdRng::from_seed(&[24, 25, 12]);
        let mut pts = vec![];
        let mut qt: Quadtree<Point> = Quadtree::new(bb, 10);
        for _ in 0..10_000 {
            pts.push(Point {
                x: rng.gen::<f64>() * 100.0 - 50.0,
                y: rng.gen::<f64>() * 100.0 - 50.0,
            });
        }

        b.iter(|| {
            black_box({
                for p in pts.iter() {
                    qt.insert(p);
                }
                qt.clear();
            });
        });
    }
    #[bench]
    fn insert_100_000(b: &mut Bencher) {
        let bb = BoundingBox {
            x: 0.0,
            y: 0.0,
            w: 50.0,
            h: 50.0,
        };
        let mut rng = StdRng::from_seed(&[24, 25, 12]);
        let mut pts = vec![];
        let mut qt: Quadtree<Point> = Quadtree::new(bb, 10);
        for _ in 0..100_000 {
            pts.push(Point {
                x: rng.gen::<f64>() * 100.0 - 50.0,
                y: rng.gen::<f64>() * 100.0 - 50.0,
            });
        }

        b.iter(|| {
            black_box({
                for p in pts.iter() {
                    qt.insert(p);
                }
                qt.clear();
            });
        });
    }
    #[bench]
    fn find_1_000(b: &mut Bencher) {
        let bb = BoundingBox {
            x: 0.0,
            y: 0.0,
            w: 50.0,
            h: 50.0,
        };
        let mut rng = StdRng::from_seed(&[24, 25, 12]);
        let mut pts = vec![];
        let mut qt: Quadtree<Point> = Quadtree::new(bb, 10);
        for _ in 0..1_000 {
            pts.push(Point {
                x: rng.gen::<f64>() * 100.0 - 50.0,
                y: rng.gen::<f64>() * 100.0 - 50.0,
            });
        }
        for p in pts.iter() {
            qt.insert(p);
        }

        b.iter(|| {
            black_box(qt.find(&BoundingBox {
                x: 0.0,
                y: 0.0,
                w: 25.0,
                h: 25.0,
            }));
        });
    }
    #[bench]
    fn find_10_000(b: &mut Bencher) {
        let bb = BoundingBox {
            x: 0.0,
            y: 0.0,
            w: 50.0,
            h: 50.0,
        };
        let mut rng = StdRng::from_seed(&[24, 25, 12]);
        let mut pts = vec![];
        let mut qt: Quadtree<Point> = Quadtree::new(bb, 10);
        for _ in 0..10_000 {
            pts.push(Point {
                x: rng.gen::<f64>() * 100.0 - 50.0,
                y: rng.gen::<f64>() * 100.0 - 50.0,
            });
        }
        for p in pts.iter() {
            qt.insert(p);
        }

        b.iter(|| {
            black_box(qt.find(&BoundingBox {
                x: 0.0,
                y: 0.0,
                w: 25.0,
                h: 25.0,
            }));
        });
    }

    #[bench]
    fn find_100_000(b: &mut Bencher) {
        let bb = BoundingBox {
            x: 0.0,
            y: 0.0,
            w: 50.0,
            h: 50.0,
        };
        let mut rng = StdRng::from_seed(&[24, 25, 12]);
        let mut pts = vec![];
        let mut qt: Quadtree<Point> = Quadtree::new(bb, 10);
        for _ in 0..100_000 {
            pts.push(Point {
                x: rng.gen::<f64>() * 100.0 - 50.0,
                y: rng.gen::<f64>() * 100.0 - 50.0,
            });
        }
        for p in pts.iter() {
            qt.insert(p);
        }

        b.iter(|| {
            black_box(qt.find(&BoundingBox {
                x: 0.0,
                y: 0.0,
                w: 25.0,
                h: 25.0,
            }));
        });
    }
}
