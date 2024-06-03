use crate::tiles::tile::Tile;

trait TilingStrategy {
    // Whenever a new application is created we want to expand our tiles
    fn tile(tiles: Vec<Tile>) -> Vec<Tile>;
}
