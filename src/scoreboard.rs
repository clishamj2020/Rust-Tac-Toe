use crate::constants::BUFFER_STRING;

pub struct Scoreboard {
    pub player1_name: String,
    pub player2_name: String,
    pub player1_score: i32,
    pub player2_score: i32,
}

impl Scoreboard {
    pub fn new(player1_name: String, player2_name: String) -> Scoreboard {
        Scoreboard {
            player1_name,
            player2_name,
            player1_score: 0,
            player2_score: 0,
        }
    }

    pub fn update_score(&mut self, player1_score: i32, player2_score: i32) {
        self.player1_score = player1_score;
        self.player2_score = player2_score;
    }

    pub fn render_scoreboard(&self) {
        print!("{}", BUFFER_STRING);
        println!("{:<15} | {:<15}", self.player1_name, self.player2_name);
        println!("{:<15} | {:<15}", self.player1_score, self.player2_score);
    }
}
