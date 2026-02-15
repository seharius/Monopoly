use crate::game::player::PlayerId;

pub const BOARD_SIZE: usize = 44;

#[derive(Debug)]
pub enum Tile {
    Start,
    Property(Property),
    Tax(i32),
    Empty,
}

#[derive(Debug)]
pub struct Property {
    pub price: i32,
    pub rent: i32,
    pub owner: Option<PlayerId>,
}

impl Property {
    pub fn new(price: i32, rent: i32) -> Self {
        Self {
            price,
            rent,
            owner: None,
        }
    }
}

pub fn create_board() -> Vec<Tile> {
    let mut board = Vec::with_capacity(BOARD_SIZE);

    for _ in 0..BOARD_SIZE {
    board.push(Tile::Empty);
}

    board[0] = Tile::Start;
    board[1] = Tile::Property(Property::new(1000, 100));
    board[2] = Tile::Tax(500);
    board[3] = Tile::Property(Property::new(1500, 200));

    board
}