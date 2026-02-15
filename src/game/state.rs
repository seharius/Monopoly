use rand::{rngs::StdRng, Rng, SeedableRng};

use crate::game::{
    board::{create_board, Tile, BOARD_SIZE},
    command::Command,
    player::{Player, PlayerId},
};

const START_MONEY: i32 = 10000;
const PASS_START_MONEY: i32 = 2500;

#[derive(Debug)]
pub struct GameState {
    pub players: Vec<Player>,
    pub board: Vec<Tile>,
    pub current_player: PlayerId,
    rng: StdRng,
}

impl GameState {
    pub fn new(player_count: usize) -> Self {
        let players = (0..player_count)
            .map(|id| Player::new(id, START_MONEY))
            .collect();

        Self {
            players,
            board: create_board(),
            current_player: 0,
            rng: StdRng::seed_from_u64(42),
        }
    }

    pub fn apply(&mut self, command: Command) -> Result<(), String> {
        match command {
            Command::RollDice => self.roll_dice(),
            Command::BuyProperty => self.buy_property(),
            Command::EndTurn => self.end_turn(),
        }
    }

    fn roll_dice(&mut self) -> Result<(), String> {
        let roll = self.rng.gen_range(2..=12);

        let player = &mut self.players[self.current_player];
        let new_pos = (player.position + roll) % BOARD_SIZE;

        if player.position + roll >= BOARD_SIZE {
            player.money += PASS_START_MONEY;
        }

        player.position = new_pos;

        println!("Gracz {} wyrzucił {}", player.id, roll);

        self.resolve_tile()
    }

    fn resolve_tile(&mut self) -> Result<(), String> {
        let player_id = self.current_player;
        let player_pos = self.players[player_id].position;

        match &mut self.board[player_pos] {
            Tile::Property(property) => {
                if let Some(owner) = property.owner {
                    if owner != player_id {
                        let rent = property.rent;
                        self.players[player_id].money -= rent;
                        self.players[owner].money += rent;
                    }
                }
            }
            Tile::Tax(amount) => {
                self.players[player_id].money -= *amount;
            }
            _ => {}
        }

        Ok(())
    }

   fn buy_property(&mut self) -> Result<(), String> {
    
    let player_id = self.current_player;
    let player_pos = self.players[player_id].position;

    println!("Pozycja gracza: {}", player_pos);
    println!("Tile: {:?}", self.board[player_pos]);

    match &mut self.board[player_pos] {
        Tile::Property(property) => {
            println!("Wchodzimy w Property");

            if property.owner.is_some() {
                return Err("Ma właściciela".into());
            }

            if self.players[player_id].money < property.price {
                return Err("Za mało pieniędzy".into());
            }

            println!("Przed zakupem: {}", self.players[player_id].money);

            self.players[player_id].money -= property.price;
            property.owner = Some(player_id);

            println!("Po zakupie: {}", self.players[player_id].money);

            Ok(())
        }
        _ => Err("To nie jest nieruchomość".into()),
    }
}

    fn end_turn(&mut self) -> Result<(), String> {
        self.current_player = (self.current_player + 1) % self.players.len();
        Ok(())
    }
}