use crate::tiles::tile::Tile;
use crate::tree::node::TileTreeNode;

pub struct TileTree {
    root: TileTreeNode
}

impl TileTree {

    pub fn new(tile: Tile) -> TileTree {
        return TileTree {
            root: TileTreeNode::new_leaf(tile, 1, None)
        }
    }

    pub fn insert(self, tile: Tile) {
        Self::insert_tile(&self.root, tile)
    }

    fn insert_tile(node: &TileTreeNode, tile: Tile) {
        // How inserts work:
        // 1. Find the first node, scanning right to left, that has no children
        // 2. Expand this node to have two children
        //    2.1 This node becomes a parent
        //    2.2 Two children get added, left child has parent's original value, right child has the new value

        // Base case is that this node has no children
        if node.is_leaf() {
            let mut node = node.clone();

            let left_child = Box::new(TileTreeNode::new_leaf(node.tile.unwrap(), node.depth + 1, Some(node.clone())));
            let right_child = Box::new(TileTreeNode::new_leaf(tile, node.depth + 1, Some(node)));

            node.tile = None;
            node.left = Some(left_child);
            node.right = Some(right_child);
        }

        match node.right {
            None => {}
            Some(child) => Self::insert_tile(child, tile)
        }

        match node.left {
            None => {}
            Some(child) => Self::insert_tile(child.into_inner(), tile)
        }

    }

}
