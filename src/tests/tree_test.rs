#[test]
fn leaf_node_test() {
    let tile = Tile::new(Rect::new(0, 0, 2000, 2000));
    let leaf = TileTreeNode::new_leaf(tile, 1, None);

    assert_eq!(true, leaf.is_leaf(), "No children implies leaf node")
}
