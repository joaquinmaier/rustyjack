use crate::hand::Hand;
use crate::card::components::SumType;

pub enum GameResultType
{
    WIN,
    PUSH,
    LOSE
}

pub struct GameResult
{
    pub player_id:  usize,
    pub result:     GameResultType
}

impl GameResult
{
    pub fn new( player_id: usize, result: GameResultType ) -> GameResult {
        GameResult { player_id, result }
    }
}

pub fn determine_winner( hands: &Vec<Hand> ) -> usize {
    let mut temp_winner: (usize, i32)  = ( 0, 0 );
    for (index, hand) in hands.iter().enumerate() {
        match hand.sum_value.clone().unwrap() {
            SumType::SingleValue( n ) => {
                if n <= 21 && n > temp_winner.1 {
                    temp_winner = ( index, n );

                } else if n == temp_winner.1 {
                    // * This is a band-aid. Will not work properly with more than a single player (pt 1)
                    return 255;
                }
            },
            SumType::MultipleValue( n1, n2 ) => {
                if n2 <= 21 && n2 > temp_winner.1 {
                    temp_winner = ( index, n2 );

                }
                else if n1 <= 21 && n1 > temp_winner.1 {
                    temp_winner = ( index, n1 );

                } else if n1 == temp_winner.1 || n2 == temp_winner.1 {
                    // * This is a band-aid. Will not work properly with more than a single player (pt 2)
                    return 255;

                }
            }
        }
    }

    return temp_winner.0;
}

pub fn determine_winners( hands: &Vec<Hand> ) -> Vec<GameResult> {
    let mut results: Vec<GameResult> = Vec::new();
    let croupier_hand = hands[ 0 ].sum_value.clone().unwrap();

    for ( index, hand ) in hands.iter().enumerate() {
        if index == 0 { continue; }

        match hand.sum_value.clone().unwrap() {
            SumType::SingleValue( n ) if n <= 21  =>
            {
                match croupier_hand {
                    SumType::SingleValue( c_h ) => {
                        if c_h > 21 {
                            results.push( GameResult::new( index, GameResultType::WIN ) );
                            continue;

                        }

                        if n > c_h          { results.push( GameResult::new( index, GameResultType::WIN ) ); }
                        else if n == c_h    { results.push( GameResult::new( index, GameResultType::PUSH ) ); }
                        else                { results.push( GameResult::new( index, GameResultType::LOSE ) ); }

                    },
                    SumType::MultipleValue( c_h1, c_h2 ) => {
                        if c_h1 > 21 {
                            results.push( GameResult::new( index, GameResultType::WIN ) );
                            continue;

                        }

                        if n > c_h2 && n > c_h1 { results.push( GameResult::new( index, GameResultType::WIN ) ); }
                        else if n == c_h2       { results.push( GameResult::new( index, GameResultType::PUSH ) ); }
                        else                    { results.push( GameResult::new( index, GameResultType::LOSE ) ); }
                    }
                }
            },
            SumType::MultipleValue( n1, n2 ) if n1 <= 21 && n2 <= 21  =>
            {
                match croupier_hand {
                    SumType::SingleValue( c_h ) => {
                        if c_h > 21 {
                            results.push( GameResult::new( index, GameResultType::WIN ) );
                            continue;

                        }

                        if n1 > c_h || n2 > c_h { results.push( GameResult::new( index, GameResultType::WIN ) ); }
                        else if n2 == c_h       { results.push( GameResult::new( index, GameResultType::PUSH ) ); }
                        else                    { results.push( GameResult::new( index, GameResultType::LOSE ) ); }
                    },
                    SumType::MultipleValue( c_h1, c_h2 ) => {
                        if c_h1 > 21 {
                            results.push( GameResult::new( index, GameResultType::WIN ) );
                            continue;

                        }

                        if ( n1 > c_h1 && n1 > c_h2 ) || ( n2 > c_h1 && n2 > c_h2 ) { results.push( GameResult::new( index, GameResultType::WIN ) ); }
                        else if n2 == c_h2  { results.push( GameResult::new( index, GameResultType::PUSH ) ); }
                        else                { results.push( GameResult::new( index, GameResultType::LOSE ) ); }
                    }
                }
            },
            _ => { results.push( GameResult::new( index, GameResultType::LOSE ) ); }
        }
    }

    results
}
