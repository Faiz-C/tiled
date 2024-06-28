use tiled::geometry::Rect;
use tiled::tile::Tile;
use tiled::tree::{TileTree, TileNode};

#[test]
fn leaf_node_test() {
    let tile = Tile::new(1, Rect::new(0, 0, 2000, 2000));
    let leaf = TileNode::new_leaf(tile, 1, None);

    assert_eq!(true, leaf.is_leaf(), "Children found on leaf node")
}

#[test]
fn insert_once() {
    let mut tree = TileTree::new(Rect::from_dimensions(3840, 2160));
    tree.insert(1);
    let visit = tree.visit();

    assert_eq!(1, visit.len(), "Tree has more than 1 element with a single insert");
    assert_eq!(1, visit[0].app_process_id, "Single element's id doesn't match the inserted id");
    assert_eq!(3840, visit[0].boundary.width(), "Single element's width isn't correct");
    assert_eq!(2160, visit[0].boundary.height(), "Single element's height isn't correct");
}

#[test]
fn insert_many() {
    let mut tree = TileTree::new(Rect::from_dimensions(3840, 2160));

    tree.insert(1);
    tree.insert(2);

    let visit = tree.visit();

    assert_eq!(2, visit.len(), "First visit doesn't have exactly 2 elements");

    for (id, tile) in visit.iter().enumerate() {
        assert_eq!(id + 1, tile.app_process_id as usize, "Expected id {:?}", id);
        assert_eq!(1920, tile.boundary.width(), "Expected 1920 for the child's width");
        assert_eq!(2160, tile.boundary.height(), "Expected 2160 for the child's height");
    }

    tree.insert(3);
    tree.insert(4);

    let visit = tree.visit();

    // We should now have something that looks like
    // |----------------|
    // |  1    |   2    |
    // |       |        |
    // |----------------|
    // |  4    |   3    |
    // |       |        |
    // |----------------|

    assert_eq!(4, visit.len(), "Second visit doesn't have exactly 4 elements");

    for (id, tile) in visit.iter().enumerate() {
        assert_eq!(id + 1, tile.app_process_id as usize, "Expected id {:?}", id);
        assert_eq!(1920, tile.boundary.width(), "Expected 1920 for the child's width");
        assert_eq!(1080, tile.boundary.height(), "Expected 1080 for the child's height");
    }
}
