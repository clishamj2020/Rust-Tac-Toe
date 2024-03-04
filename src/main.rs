struct Tile {
    tile_display: String,
    tile_value: String,
}

impl Tile {
    fn new(value: String) -> Tile {
        Tile {
            tile_display: String::from("+"),
            tile_value: value,
        }
    }
}

fn main() {
    let tile = Tile::new(String::from("1"));
    println!("Tile Value: {}", tile.tile_value);
    println!("Tile Display: {}", tile.tile_display);
}