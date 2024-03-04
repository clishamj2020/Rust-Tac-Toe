pub struct Player {
    pub turn: bool,
    pub player_name: String,
}

impl Player {
    pub fn new(turn_status: bool, name: String) -> Player {
        Player {
            turn: turn_status,
            player_name: name,
        }
    }
}
