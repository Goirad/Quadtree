extern crate rand;
use rand::{thread_rng, Rng};
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



mod quadtree {
    #[derive(Debug, Copy, Clone)]

    struct BoundingBox {
        x: f64, //center coords
        y: f64,
        w: f64, //half width and height
        h: f64,
    }
    impl BoundingBox {
        fn contains<T: Spacial>(&self, item: &T) -> bool {
            !(item.x() < self.x - self.w || item.x() > self.x + self.w || item.y() > self.y + self.h
                || item.y() < self.y - self.h)
        }
        fn contains_completely(&self, other: &BoundingBox) -> bool {
            (other.x - other.w > self.x - self.w && other.x + other.w < self.x + self.w
                && other.y - other.h > self.y - self.h && other.y + other.h < self.y + self.h)
        }

        fn intersects(&self, other: &BoundingBox) -> bool {
            !(self.x + self.w < other.x - other.w || self.x - self.w > other.x + other.w
                || self.y + self.h < other.y - other.h || self.y - self.h > other.y + other.h)
        }
        fn tl(&self) -> BoundingBox {
            BoundingBox {
                x: self.x - self.w / 2.0,
                y: self.y + self.h / 2.0,
                w: self.w / 2.0,
                h: self.h / 2.0,
            }
        }
        fn tr(&self) -> BoundingBox {
            BoundingBox {
                x: self.x + self.w / 2.0,
                y: self.y + self.h / 2.0,
                w: self.w / 2.0,
                h: self.h / 2.0,
            }
        }
        fn bl(&self) -> BoundingBox {
            BoundingBox {
                x: self.x - self.w / 2.0,
                y: self.y - self.h / 2.0,
                w: self.w / 2.0,
                h: self.h / 2.0,
            }
        }
        fn br(&self) -> BoundingBox {
            BoundingBox {
                x: self.x + self.w / 2.0,
                y: self.y - self.h / 2.0,
                w: self.w / 2.0,
                h: self.h / 2.0,
            }
        }
    }

    #[derive(Debug)]
    struct Quadtree<'a, T: 'a> {
        split_threshold: usize,
        bb: BoundingBox,
        data: Vec<&'a T>,
        //st: Vec<Box<Quadtree<'a, T>>>,
        st: Vec<Quadtree<'a, T>>,
    }
    trait Spacial {
        fn x(&self) -> f64;
        fn y(&self) -> f64;
    }

    impl<'a, T: Spacial> Quadtree<'a, T> {
        pub fn new(bb: BoundingBox, threshold: usize) -> Quadtree<'a, T> {
            Quadtree {
                split_threshold: threshold,
                st: vec![],
                bb,
                data: vec![],
            }
        }
        fn contains(&self, item: &T) -> bool {
            self.bb.contains(item)
        }
        fn split(&mut self) {
            if (self.st.len() == 0) {
                let mut st = vec![];
                st.push(Quadtree::new(self.bb.tl(), self.split_threshold));
                st.push(Quadtree::new(self.bb.tr(), self.split_threshold));
                st.push(Quadtree::new(self.bb.bl(), self.split_threshold));
                st.push(Quadtree::new(self.bb.br(), self.split_threshold));

                self.st = st;
            }
        }
        pub fn insert(&mut self, item: &'a T) -> bool {
            if !self.contains(item) {
                return false;
            }
            if self.data.len() < self.split_threshold {
                self.data.push(item);
                return true;
            } else {
                self.split();

                for t in self.st.iter_mut() {
                    if t.insert(item) {
                        return true;
                    }
                }
                false
            }
        }
        //consider converting this to an iterative approach
        //maybe first go through and do a df traversal, with the obvious stop condition
        //and then check each tree that matches

        //also maybe consider a new check to see if the sb completely contains the
        //trees bounding box, in that case it can be iterated through rather than recursed through
        pub fn find(&self, sb: &BoundingBox) -> Vec<&T> {
            let mut result = vec![];

            if self.bb.intersects(&sb) {
                for d in self.data.iter() {
                    if sb.contains(*d) {
                        result.push(*d);
                    }
                }
                for st in self.st.iter() {
                    //don't bother recursing if that subtree is empty
                    //the idea being that it is cheaper to check its length
                    //here than to enter a new function call
                    if st.data.len() > 0 {
                        result.append(&mut st.find(sb));
                    }
                }
            }

            result
        }
        fn tot_trees(&self) -> usize {
            let mut total = 0;

            total += self.st.len();
            for st in self.st.iter() {
                total += st.tot_trees();
            }
            total
        }
        fn clear(&mut self) {
            self.data.clear();
            self.st.clear();
        }
    }

}
