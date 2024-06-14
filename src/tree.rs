use std::cell::RefCell;
use std::rc::{Rc, Weak};
use crate::tile::Tile;

// Because a TileTreeNode is a recursive type:
// 1) We need to wrap the children and parents in either a Box or a Rc/Weak so Rust is able to
//    properly allocate memory for us as those pointers have a fixed size.
// 2) Rc is chosen here so that we can have multiple ownership
// 3) RefCell is required as the contents within a Rc are *immutable*, by wrapping the inner content
//    with a RefCell we allow for interior mutability using _borrow_mut_
// 4) Weak is used for the parent to avoid needless ownership of the parent

#[derive(Debug, Clone)]
pub struct TileTreeNode {
    pub tile: Option<Tile>,
    pub depth: i32,
    pub left: Option<Box<RefCell<TileTreeNode>>>,
    pub right: Option<Box<RefCell<TileTreeNode>>>,
    pub parent: Option<Weak<RefCell<TileTreeNode>>>,
}

impl TileTreeNode {
    pub fn new(
        tile: Option<Tile>,
        depth: i32,
        left: Option<Box<RefCell<TileTreeNode>>>,
        right: Option<Box<RefCell<TileTreeNode>>>,
        parent: Option<Weak<RefCell<TileTreeNode>>>
    ) -> TileTreeNode {
        return TileTreeNode {
            tile,
            depth,
            left,
            right,
            parent
        }
    }

    pub fn new_leaf(tile: Tile, depth: i32, parent: Option<Weak<RefCell<TileTreeNode>>>) -> TileTreeNode {
        return Self::new(Some(tile), depth, None, None, parent)
    }

    // &self -> borrows the reference instead of moving it
    // which allows for continued usage of the object after we call this method
    pub fn is_leaf(&self) -> bool {
        return self.right.is_none() && self.left.is_none()
    }
}


pub struct TileTree {
    root: TileTreeNode
}

impl TileTree {

    pub fn new(tile: Tile) -> TileTree {
        return TileTree {
            root: TileTreeNode::new_leaf(tile, 0, None)
        }
    }

    pub fn insert(&self, tile: Tile) {

        fn insert(node: &TileTreeNode, tile: Tile) {
            // How inserts work:
            // 1. Find the first node, scanning right to left, that has no children
            // 2. Expand this node to have two children
            //    2.1 This node becomes a parent
            //    2.2 Two children get added, left child has parent's original value, right child has the new value

            if node.is_leaf() {
                let parent = Rc::new(RefCell::new(node.clone()));
                let child_depth = node.depth + 1;

                let left_child = Box::new(RefCell::new(TileTreeNode::new_leaf(
                    node.tile.unwrap(),
                    child_depth,
                    Some(Rc::downgrade(&parent))
                )));

                let right_child = Box::new(RefCell::new(TileTreeNode::new_leaf(
                    tile,
                    child_depth,
                    Some(Rc::downgrade(&parent))
                )));

                let mut parent = parent.borrow_mut();

                parent.tile = None;
                parent.left = Some(left_child);
                parent.right = Some(right_child);

                return
            }

            let node = node.clone();

            match node.right {
                None => {}
                Some(child) => insert(&child.borrow(), tile)
            }

            match node.left {
                None => {}
                Some(child) => insert(&child.borrow(), tile)
            }

        }

        insert(&self.root, tile)
    }

    pub fn visit(&self) -> Vec<Tile> {
        fn visit(node: &TileTreeNode) -> Vec<Tile> {
            let mut tiles = Vec::new();

            let node = node.clone();

            match node.left {
                None => {}
                Some(child) => {
                    let mut child_tiles = visit(&child.borrow());
                    tiles.append(&mut child_tiles);
                }
            }

            tiles.push(node.tile.unwrap());

            match node.right {
                None => {}
                Some(child) => {
                    let mut child_tiles = visit(&child.borrow());
                    tiles.append(&mut child_tiles);
                }
            }

            return tiles;
        }

        return visit(&self.root)
    }
}
