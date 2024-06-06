use crate::tiles::tile::Tile;

#[derive(Debug, Copy, Clone)]
pub struct TileTreeNode {
    pub tile: Option<Tile>,
    pub depth: i32,
    pub left: Option<TileTreeNode>,
    pub right: Option<TileTreeNode>,
    pub parent: Option<TileTreeNode>,
}

impl TileTreeNode {
    pub fn new(tile: Option<Tile>, depth: i32, left: Option<TileTreeNode>, right: Option<TileTreeNode>, parent: Option<TileTreeNode>) -> TileTreeNode {
        return TileTreeNode {
            tile,
            depth,
            left,
            right,
            parent
        }
    }

    pub fn new_leaf(tile: Tile, depth: i32, parent: Option<TileTreeNode>) -> TileTreeNode {
        return Self::new(Some(tile), depth, None, None, parent)
    }

    pub fn is_leaf(self) -> bool {
        return self.right.is_none() && self.left.is_none()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::geometry::rectangle::Rect;
    use crate::tiles::tile::Tile;
    use crate::tree::node::TileTreeNode;

    #[test]
    fn leaf_node_test() {
        let tile = Tile::new(Rect::new(0, 0, 2000, 2000));
        let leaf = TileTreeNode::new_leaf(tile, 1, None);

        assert_eq!(true, leaf.is_leaf(), "No children implies leaf node")
    }
}
