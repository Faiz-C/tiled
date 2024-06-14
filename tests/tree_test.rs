use tiled::geometry::Rect;
use tiled::tile::Tile;
use tiled::tree::{TileTree, TileTreeNode};

#[test]
fn leaf_node_test() {
    let tile = Tile::new(Rect::new(0, 0, 2000, 2000));
    let leaf = TileTreeNode::new_leaf(tile, 1, None);

    assert_eq!(true, leaf.is_leaf(), "Children found on leaf node")
}

#[test]
fn insert_once() {
    let tile = Tile::new(Rect::new(0, 0, 2000, 2000));
    let tree = TileTree::new(tile);
    let visit = tree.visit();

    assert_eq!(1, visit.len(), "Tree has more than 1 element with a single insert");
    assert_eq!(tile, visit[0], "Tree's single element doesn't match the inserted tile")
}
