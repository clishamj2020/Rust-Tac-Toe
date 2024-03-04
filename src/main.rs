use std::io::{self, Write};
// '&' means reference to the value, ' means lifetime of the value (not the reference), static means the lifetime of the program (the entire time the program is running)
const BUFFER_STRING: &'static str = "\n<=====================================================>\n\n";
const ERROR_STRING: &'static str = "\nXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXX\n\n";
// Have to have this when trying to print out an object
#[derive(Debug)]
struct Tile {
    tile_value: String,
}

impl Tile {
    fn new(value: String) -> Tile {
        Tile { tile_value: value }
    }
}

struct Player {
    turn: bool,
    player_name: String,
}

impl Player {
    fn new(turn_status: bool, name: String) -> Player {
        Player {
            turn: turn_status,
            player_name: name,
        }
    }
}

// Must use lifetime annotations to ensure that the references in the struct are valid for the entire lifetime of the struct
struct Scoreboard {
    player1_name: String,
    player2_name: String,
    player1_score: i32,
    player2_score: i32,
}

impl Scoreboard {
    fn new(player1_name: String, player2_name: String) -> Scoreboard {
        Scoreboard {
            player1_name,
            player2_name,
            player1_score: 0,
            player2_score: 0,
        }
    }

    fn update_score(&mut self, player1_score: i32, player2_score: i32) {
        self.player1_score = player1_score;
        self.player2_score = player2_score;
    }

    fn render_scoreboard(&self) {
        println!("{}", BUFFER_STRING);
        println!("{:<15} | {:<15}", self.player1_name, self.player2_name);
        println!("{:<15} | {:<15}", self.player1_score, self.player2_score);
    }
}

struct Board {
    tiles: Vec<Tile>,
}

impl Board {
    fn new() -> Board {
        Self::populate_board()
    }

    fn populate_board() -> Board {
        let mut tiles = Vec::new();
        for i in 0..9 {
            tiles.push(Tile::new((i + 1).to_string()));
        }
        Board { tiles }
    }

    fn render_board(&mut self) {
        const BOARD_HORIZONTAL_LINE: &'static str = "-----------";
        println!("{}", BUFFER_STRING);

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
        println!("{}", BUFFER_STRING);
    }

    fn change_tile_value(&mut self, tile_number: usize, val: String) -> () {
        self.tiles[tile_number - 1].tile_value = val;
    }

    fn is_win(&mut self) -> i32 {
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

    fn render_exit_message(&mut self) -> () {
        println!("{}", BUFFER_STRING);
        println!("All Done! Thanks for playing :)");
        println!("{}", BUFFER_STRING);
    }
}

fn tic_tac_toe() {
    println!("Welcome to Tic-Tac-Toe!");
    println!("Player 1 will be X's and Player 2 will be O's.");

    let player_one_name = get_player_name("Player 1");
    let player_two_name = get_player_name("Player 2");

    let mut player_x = Player::new(true, player_one_name);
    let mut player_o = Player::new(false, player_two_name);

    let mut new_game = false;
    let mut exit_flag = false;
    let mut board = Board::new();
    let mut scoreboard =
        Scoreboard::new(player_x.player_name.clone(), player_o.player_name.clone());

    loop {
        if new_game {
            println!("Welcome to Tic-Tac-Toe!");
            println!("Player 1 will be X's and Player 2 will be O's.");
            let change_players = get_yes_or_no_input("Would you like to change players? (Y/n)");

            if change_players == "Y" {
                scoreboard.update_score(0, 0);
                player_x.player_name = get_player_name("Player 1");
                player_o.player_name = get_player_name("Player 2");
                scoreboard.player1_name = player_x.player_name.clone();
                scoreboard.player2_name = player_o.player_name.clone();
                player_x.turn = true;
                player_o.turn = false;
            } else if change_players == "N" {
                println!("{}", BUFFER_STRING);
                println!("No worries, let's begin!");
                let tmp_turn = player_x.turn;
                player_x.turn = player_o.turn;
                player_o.turn = tmp_turn;
            }
        }

        scoreboard.render_scoreboard();

        loop {
            let (tile_val, current_name) = if player_x.turn {
                (String::from("X"), player_x.player_name.clone())
            } else {
                (String::from("O"), player_o.player_name.clone())
            };

            board.render_board();

            let tile_num = get_tile_number(&current_name);

            if tile_num == 0 {
                exit_flag = true;
                board.render_exit_message();
                break;
            }

            if board.tiles[tile_num as usize - 1].tile_value == "X"
                || board.tiles[tile_num as usize - 1].tile_value == "O"
            {
                println!("{}", ERROR_STRING);
                println!(
                    "{}, the selected tile is already occupied. Please try again.",
                    current_name
                );
                println!("{}", ERROR_STRING);
            } else {
                board.change_tile_value(tile_num as usize, tile_val.clone());
                let who_won = board.is_win();

                if who_won == 1 {
                    println!("{}", BUFFER_STRING);
                    println!("{}, won the game! Great job :)", current_name);
                    if current_name == player_x.player_name {
                        scoreboard.player1_score += 1;
                    } else if current_name == player_o.player_name {
                        scoreboard.player2_score += 1;
                    }
                    scoreboard.render_scoreboard();
                    board.render_board();
                    board = Board::new();
                    break;
                } else if who_won == -1 {
                    println!("{}", BUFFER_STRING);
                    println!(
                        "The game ended in a tie. Nice try {} & {}!",
                        player_x.player_name, player_o.player_name
                    );
                    board.render_board();
                    board = Board::new();
                    break;
                }

                player_x.turn = !player_x.turn;
                player_o.turn = !player_o.turn;
            }
        }

        if exit_flag {
            break;
        }

        let play_again = get_yes_or_no_input("Would you like to play again? (Y/n)");

        if play_again == "N" {
            break;
        } else if play_again == "Y" {
            println!("{}", BUFFER_STRING);
            println!("New Game!!!");
            new_game = true;
        }
    }
}

fn get_player_name(prompt: &str) -> String {
    loop {
        print!("Enter {}'s name: ", prompt);
        io::stdout().flush().unwrap();
        let mut name = String::new();
        io::stdin()
            .read_line(&mut name)
            .expect("Failed to read line");
        let trimmed_name = name.trim();
        if !trimmed_name.is_empty() && trimmed_name.chars().all(|c| c.is_alphabetic()) {
            return trimmed_name.to_string();
        }
        println!(
            "Name must be non-empty and contain only alphabetic characters, please try again."
        );
    }
}

fn get_yes_or_no_input(prompt: &str) -> String {
    loop {
        print!("{}", prompt);
        io::stdout().flush().unwrap();
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");
        let trimmed_input = input.trim().to_uppercase();
        if trimmed_input == "Y" || trimmed_input == "N" {
            return trimmed_input;
        }
        println!("Please enter either 'Y' or 'N'.");
    }
}

fn get_tile_number(player_name: &str) -> i32 {
    loop {
        print!(
            "{}, please enter the tile you'd like to acquire (1-9, or 0 to exit): ",
            player_name
        );
        io::stdout().flush().unwrap();
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");
        let trimmed_input = input.trim();
        match trimmed_input.parse::<i32>() {
            Ok(num) if (0..=9).contains(&num) => return num,
            _ => println!("Invalid input. Please enter a number between 0 and 9."),
        }
    }
}

fn main() {
    tic_tac_toe();
}
