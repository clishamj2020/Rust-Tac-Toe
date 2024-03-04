use crate::constants::{BOARD_HORIZONTAL_LINE, BUFFER_STRING};
use crate::tile::Tile;

// ... rest of the code ...
pub struct Board {
    pub tiles: Vec<Tile>,
}

impl Board {
    pub fn new() -> Board {
        Self::populate_board()
    }

    pub fn populate_board() -> Board {
        let mut tiles = Vec::new();
        for i in 0..9 {
            tiles.push(Tile::new((i + 1).to_string()));
        }
        Board { tiles }
    }

    pub fn render_board(&mut self) {
        print!("{}", BUFFER_STRING);

        for i in 0..9 {
            if (i + 1) % 3 != 0 {
                print!(" {} |", self.tiles[i].tile_value);
            } else {
                println!(" {}", self.tiles[i].tile_value);
                if i != 8 {
                    println!("{}", BOARD_HORIZONTAL_LINE);
                }
            }
        }
        print!("{}", BUFFER_STRING);
    }

    pub fn change_tile_value(&mut self, tile_number: usize, val: String) -> () {
        self.tiles[tile_number - 1].tile_value = val;
    }

    pub fn is_win(&mut self) -> i32 {
        let mut x_vals: Vec<i32> = Vec::new();
        let mut o_vals: Vec<i32> = Vec::new();
        // Must use vec! macro when declaring vecc
        let winning_combos: Vec<Vec<i32>> = vec![
            vec![1, 2, 3],
            vec![4, 5, 6],
            vec![7, 8, 9],
            vec![1, 4, 7],
            vec![2, 5, 8],
            vec![3, 6, 9],
            vec![1, 5, 9],
            vec![3, 5, 7],
        ];
        for i in 0..9 {
            if self.tiles[i].tile_value == String::from("X") {
                x_vals.push(i as i32 + 1);
            }
            if self.tiles[i].tile_value == String::from("O") {
                o_vals.push(i as i32 + 1);
            }
        }
        // Using || destructures the object so &i becomes i32 instead of &i32
        for combo in &winning_combos {
            if combo.iter().all(|&i| x_vals.contains(&i)) {
                return 1;
            }
            if combo.iter().all(|&i| o_vals.contains(&i)) {
                return 1;
            }
            if x_vals.len() + o_vals.len() == 9 {
                return -1;
            }
        }
        0
    }

    pub fn render_exit_message(&mut self) -> () {
        print!("{}", BUFFER_STRING);
        println!("All Done! Thanks for playing :)");
        print!("{}", BUFFER_STRING);
    }
}
