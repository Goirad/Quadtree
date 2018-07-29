#[derive(Debug, Copy, Clone)]
pub struct BoundingBox {
    /// center x coord
    pub x: f64,
    /// center y coord
    pub y: f64,

    /// half width
    pub w: f64,
    ///half height
    pub h: f64,
}

impl BoundingBox {
    pub fn contains(&self, item: &BoundingBox) -> bool {
        !(item.x < self.x - self.w
            || item.x > self.x + self.w
            || item.y > self.y + self.h
            || item.y < self.y - self.h)
    }
    pub fn contains_completely(&self, other: &BoundingBox) -> bool {
        (other.x - other.w > self.x - self.w
            && other.x + other.w < self.x + self.w
            && other.y - other.h > self.y - self.h
            && other.y + other.h < self.y + self.h)
    }

    pub fn intersects(&self, other: &BoundingBox) -> bool {
        !(self.x + self.w < other.x - other.w
            || self.x - self.w > other.x + other.w
            || self.y + self.h < other.y - other.h
            || self.y - self.h > other.y + other.h)
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
pub struct Quadtree<'a, T: 'a> {
    split_threshold: usize,
    bb: BoundingBox,
    data: Vec<&'a T>,
    st: Vec<Quadtree<'a, T>>,
}

pub trait Boundable {
    fn bounds(&self) -> BoundingBox;
}

impl<'a, T: Boundable> Quadtree<'a, T> {

    /// Creates a new quadtree with the given split threshold, ie each tree contains threshold
    /// items before splitting
    pub fn new(bb: BoundingBox, threshold: usize) -> Quadtree<'a, T> {
        Quadtree {
            split_threshold: threshold,
            st: vec![],
            bb,
            data: vec![],
        }
    }

    /// Checks whether the item is within the trees bounds, NOT if the item is actually in the tree
    pub fn contains(&self, item: &T) -> bool {
        self.bb.contains(&item.bounds())
    }

    fn split(&mut self) {
        if self.st.len() == 0 {
            let mut st = vec![];
            st.push(Quadtree::new(self.bb.tl(), self.split_threshold));
            st.push(Quadtree::new(self.bb.tr(), self.split_threshold));
            st.push(Quadtree::new(self.bb.bl(), self.split_threshold));
            st.push(Quadtree::new(self.bb.br(), self.split_threshold));

            self.st = st;
        }
    }

    /// Recursively inserts item into the quadtree
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


    /// Recursively finds all elements whose bounding boxes overlap with the search box
    pub fn find(&self, search_box: &BoundingBox) -> Vec<&T> {
        let mut result = vec![];
        self.find_rec(search_box, &mut result);
        result
    }

    fn find_rec(&self, search_box: &BoundingBox, out: &mut Vec<&'a T>) {
        if self.bb.intersects(&search_box) {
            for d in self.data.iter() {
                if search_box.contains(&d.bounds()) {
                    out.push(*d);
                }
            }
            for st in self.st.iter() {
                //don't bother recursing if that subtree is empty
                //the idea being that it is cheaper to check its length
                //here than to enter a new function call
                if st.data.len() > 0 {
                    st.find_rec(search_box, out);
                }
            }
        }
    }

    /// Returns the number of trees contained in this subtree, NOT including itself
    pub fn total_trees(&self) -> usize {
        let mut total = 0;

        total += self.st.len();
        for st in self.st.iter() {
            total += st.total_trees();
        }
        total
    }

    /// Clear the subtree, removes all data and subtrees
    pub fn clear(&mut self) {
        self.data.clear();
        self.st.clear();
    }
}
