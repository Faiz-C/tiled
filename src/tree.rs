use std::cell::RefCell;
use std::rc::{Rc, Weak};
use crate::geometry::Rect;
use crate::tile::Tile;

// Because a TileTreeNode is a recursive type:
// 1) We need to wrap the children and parents in either a Box or a Rc/Weak so Rust is able to
//    properly allocate memory for us as those pointers have a fixed size.
// 2) Rc is chosen here so that we can have multiple ownership
// 3) RefCell is required as the contents within a Rc are *immutable*, by wrapping the inner content
//    with a RefCell we allow for interior mutability using _borrow_mut_
// 4) Weak is used for the parent to avoid needless ownership of the parent

#[derive(Debug, Clone)]
pub struct TileNode {
    pub tile: Option<Tile>,
    pub depth: i32,
    pub left: Option<Box<RefCell<TileNode>>>,
    pub right: Option<Box<RefCell<TileNode>>>,
    pub parent: Option<Weak<RefCell<TileNode>>>,
}

impl TileNode {
    pub fn new(
        tile: Option<Tile>,
        depth: i32,
        left: Option<Box<RefCell<TileNode>>>,
        right: Option<Box<RefCell<TileNode>>>,
        parent: Option<Weak<RefCell<TileNode>>>
    ) -> TileNode {
        return TileNode {
            tile,
            depth,
            left,
            right,
            parent
        }
    }

    pub fn new_leaf(tile: Tile, depth: i32, parent: Option<Weak<RefCell<TileNode>>>) -> TileNode {
        return Self::new(Some(tile), depth, None, None, parent)
    }

    // &self -> borrows the reference instead of moving it
    // which allows for continued usage of the object after we call this method
    pub fn is_leaf(&self) -> bool {
        return self.right.is_none() && self.left.is_none()
    }
}


pub struct TileTree {
    screen_size: Rect,
    root: Option<TileNode>
}

impl TileTree {

    pub fn new(screen_size: Rect) -> TileTree {
        return TileTree {
            screen_size,
            root: None
        }
    }

    pub fn insert(&mut self, app_process_id: i32) {

        fn insert(node: &mut TileNode, app_process_id: i32) {
            // How inserts work:
            // 1. Find the first node, scanning right to left, that has no children
            // 2. Expand this node to have two children
            //    2.1 This node becomes a parent
            //    2.2 Two children get added, left child has parent's original value, right child has the new value

            match &node.right {
                None => {}
                Some(child) => insert(&mut child.borrow_mut(), app_process_id)
            }

            match &node.left {
                None => {}
                Some(child) => insert(&mut child.borrow_mut(), app_process_id)
            }

            let child_depth = node.depth + 1;
            let node_tile = node.tile.unwrap();

            let parent_ref = Rc::new(RefCell::new(node.clone()));
            let parent = node;

            let (left_child_rect, right_child_rect) = if child_depth % 2 != 0 {
                node_tile.boundary.split_vertically()
            } else {
                node_tile.boundary.split_horizontally()
            };

            let left_child = Box::new(RefCell::new(TileNode::new_leaf(
                Tile::new(node_tile.app_process_id, left_child_rect),
                child_depth,
                Some(Rc::downgrade(&parent_ref))
            )));

            let right_child = Box::new(RefCell::new(TileNode::new_leaf(
                Tile::new(app_process_id, right_child_rect),
                child_depth,
                Some(Rc::downgrade(&parent_ref))
            )));

            parent.tile = None;
            parent.left = Some(left_child);
            parent.right = Some(right_child);
        }

        match &mut self.root {
            None => {
                let tile = Tile::new(app_process_id, self.screen_size);
                self.root = Some(TileNode::new_leaf(tile, 0, None))
            }
            Some(node) => {
                insert(node, app_process_id)
            }
        }

    }

    pub fn visit(&self) -> Vec<Tile> {
        fn visit(node: &TileNode) -> Vec<Tile> {
            let mut tiles = Vec::new();

            let node = node.clone();

            match node.left {
                None => {}
                Some(child) => {
                    let mut child_tiles = visit(&child.borrow());
                    tiles.append(&mut child_tiles);
                }
            }

            match node.tile {
                None => {}
                Some(tile) => tiles.push(tile)
            }

            match node.right {
                None => {}
                Some(child) => {
                    let mut child_tiles = visit(&child.borrow());
                    tiles.append(&mut child_tiles);
                }
            }

            return tiles;
        }

        match &self.root {
            None => Vec::new(),
            Some(node) => visit(node)
        }
    }
}
