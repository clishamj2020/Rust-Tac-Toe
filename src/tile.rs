pub struct Tile {
    pub tile_value: String,
}

impl Tile {
    pub fn new(value: String) -> Tile {
        Tile { tile_value: value }
    }
}
