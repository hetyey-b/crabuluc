// Module Puluc
// Includes the logic of the game

use rand::Rng;

pub struct PulucRoll {
    result: u8,
    rolls: [u8;4]
}

impl PulucRoll {
    pub fn from_rolls(rolls: [u8;4])->Self {
        let mut result: u8 = 0;
        for roll in rolls  {
            if roll != 0 && roll != 1 {
                result += 0;
            } else {
                result += roll;
            }
        } 

        if result == 0 {
            result = 5;
        }
        Self {
            rolls,
            result
        }
    }

    pub fn new()->Self {
        let mut rolls: [u8;4] = [0,0,0,0];

        let mut rng = rand::rng();
        for n in 0..4 {
            let curr_roll = rng.random_range(0..2);
            rolls[n] = curr_roll;
        }

        PulucRoll::from_rolls(rolls)
    }

    pub fn to_string(&self) -> String {
        format!("Roll: {}, ({},{},{},{})",self.result, self.rolls[0],self.rolls[1],self.rolls[2],self.rolls[3])
    }
} 

#[derive(Clone, Copy)]
pub enum GameColor {
    Black,
    White
}

#[derive(Clone, Copy)]
pub struct GamePiece  {
    color: GameColor,
    tile_ind: u8,
    moving_backward: bool,
    black_under: u8,
    white_under: u8
}

#[derive(Clone, Copy)]
pub struct GameBoard {
    tiles: [Option<GamePiece> ; 11],
    white_in_base: u8,
    black_in_base: u8,
    removed_white: u8,
    removed_black: u8,
    current_player: GameColor
}

impl GameBoard {
    pub fn new()->Self {
        Self {
            tiles:[None, None, None, None, None, None, None, None, None, None, None],
            white_in_base: 6,
            black_in_base: 6,
            removed_white: 0,
            removed_black: 0,
            current_player: GameColor::White
        }
    }

    pub fn to_string(&self) -> String {
        let mut str: String = "".to_string();

        for n in 0..7 {
            if n <= self.black_in_base {
                str.push('B');
            } else {
                str.push('_');
            }
        }
        
        str.push('\n');
        for tile in self.tiles {
            match tile {
                Some(game_piece) => {
                    match game_piece.color {
                        GameColor::Black => {
                            if game_piece.moving_backward {
                                str.push('^');
                            } else {
                                str.push('v');
                            }
                            str.push('B');
                        },
                        GameColor::White => {
                            if game_piece.moving_backward  {
                                str.push('v');
                            } else {
                                str.push('^');
                            }
                            str.push('W');
                        }
                    }
                    if game_piece.white_under > 0 || game_piece.black_under > 0 {
                        str.push_str(format!("(under: {}w, {}b)", game_piece.white_under, game_piece.black_under).as_str());
                    }
                }
                None => str.push('_')
            }
            str.push('\n');
        }

        for n in 0..7 {
            if n <= self.white_in_base {
                str.push('W');
            } else {
                str.push('_');
            }
        }
        str.push('\n');

        str
    }
}

pub struct PulucMove {
    player: GameColor,
    // None means base
    from: Option<u8>,
    to: Option<u8>
}

pub fn generate_legal_moves(board: GameBoard, player: GameColor, roll: u8) -> Result<Vec<PulucMove>,String> {
    let mut moves: Vec<PulucMove> = Vec::new();

    if roll > 5 {
        return Err("Roll cannot be bigger than 5!".to_string());
    }

    Ok(moves)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_puluc_roll_creation() {
        let mut roll = PulucRoll::from_rolls([0,0,0,0]);
        assert_eq!(roll.result,5);
        roll = PulucRoll::from_rolls([1,0,0,0]);
        assert_eq!(roll.result,1);
        roll = PulucRoll::from_rolls([1,1,0,0]);
        assert_eq!(roll.result,2);
        roll = PulucRoll::from_rolls([1,1,1,0]);
        assert_eq!(roll.result,3);
        roll = PulucRoll::from_rolls([1,1,1,1]);
        assert_eq!(roll.result,4);
    }
}
