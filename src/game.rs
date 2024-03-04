use crate::board::Board;
use crate::constants::{BUFFER_STRING, ERROR_STRING};
use crate::player::Player;
use crate::scoreboard::Scoreboard;
// ... rest of the code ...
use std::io::{self, Write};

pub fn tic_tac_toe() {
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

pub fn get_player_name(prompt: &str) -> String {
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

pub fn get_yes_or_no_input(prompt: &str) -> String {
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

pub fn get_tile_number(player_name: &str) -> i32 {
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
