use crate::tiles::tile::Tile;

#[derive(Debug, Clone)]
pub struct TileTreeNode {
    pub tile: Option<Tile>,
    pub depth: i32,
    pub left: Option<Box<TileTreeNode>>,
    pub right: Option<Box<TileTreeNode>>,
    pub parent: Option<Box<TileTreeNode>>,
}

impl TileTreeNode {
    pub fn new(
        tile: Option<Tile>,
        depth: i32,
        left: Option<TileTreeNode>,
        right: Option<TileTreeNode>,
        parent: Option<TileTreeNode>
    ) -> TileTreeNode {
        let left = match left {
            None => None,
            Some(l) => Some(Box::new(l))
        };

        let right = match right {
            None => None,
            Some(r) => Some(Box::new(r))
        };

        let parent = match parent {
            None => None,
            Some(p) => Some(Box::new(p))
        };

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

    pub fn is_leaf(&self) -> bool {
        return self.right.is_none() && self.left.is_none()
    }
}
