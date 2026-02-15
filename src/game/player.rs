pub type PlayerId = usize;

#[derive(Debug)]
pub struct Player {
    pub id: PlayerId,
    pub position: usize,
    pub money: i32,
    pub is_bankrupt: bool,
}

impl Player {
    pub fn new(id: PlayerId, start_money: i32) -> Self {
        Self {
            id,
            position: 0,
            money: start_money,
            is_bankrupt: false,
        }
    }
}