# Quadtree
---
This is a very simple rust library to create and use static quadtrees. These quadtrees can store only references, and do not yet support having elements removed.

#### Features and Limitations

1. Support for arbitrary split thresholds
2. Only support 2D bounding boxes
3. Boundable trait allows easy integration with your own data types


#### Planned Features (May or may not ever get implemented)

1. Support removing entries
2. Support checking if entries are present in tree
3. Investigate stack data field, for faster insertion and clearing (might require macros because it requires compile time changes). The idea is that the user of the library would experiment with different thresholds, and then once they find a value they like, they can macro in a custom size Quadtree struct with all the existing methods automatically working on it. This might require having a Quadtree trait, but I'll cross that bridge if I ever get there.