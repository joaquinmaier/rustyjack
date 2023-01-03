use crate::hand::Hand;
use crate::card::components::SumType;
use crate::Deck;
use std::rc::Rc;
use std::cell::RefCell;

pub enum GameResultType
{
    WIN,
    WINBJ,
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
                        if hand.is_blackjack()  { results.push( GameResult::new( index, GameResultType::WINBJ ) ); continue; }

                        if c_h > 21 {
                            results.push( GameResult::new( index, GameResultType::WIN ) );
                            continue;

                        }

                        if n > c_h              { results.push( GameResult::new( index, GameResultType::WIN ) ); }
                        else if n == c_h        { results.push( GameResult::new( index, GameResultType::PUSH ) ); }
                        else                    { results.push( GameResult::new( index, GameResultType::LOSE ) ); }

                    },
                    SumType::MultipleValue( c_h1, c_h2 ) => {
                        if hand.is_blackjack()  { results.push( GameResult::new( index, GameResultType::WINBJ ) ); continue; }

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
                        if hand.is_blackjack()  { results.push( GameResult::new( index, GameResultType::WINBJ ) ); continue; }

                        if c_h > 21 {
                            results.push( GameResult::new( index, GameResultType::WIN ) );
                            continue;

                        }

                        if n1 > c_h || n2 > c_h { results.push( GameResult::new( index, GameResultType::WIN ) ); }
                        else if n2 == c_h       { results.push( GameResult::new( index, GameResultType::PUSH ) ); }
                        else                    { results.push( GameResult::new( index, GameResultType::LOSE ) ); }
                    },
                    SumType::MultipleValue( c_h1, c_h2 ) => {
                        if hand.is_blackjack()  { results.push( GameResult::new( index, GameResultType::WINBJ ) ); continue; }

                        if c_h1 > 21 {
                            results.push( GameResult::new( index, GameResultType::WIN ) );
                            continue;

                        }

                        if ( n1 > c_h1 && n1 > c_h2 ) || ( n2 > c_h1 && n2 > c_h2 ) { results.push( GameResult::new( index, GameResultType::WIN ) ); }
                        else if n2 == c_h2      { results.push( GameResult::new( index, GameResultType::PUSH ) ); }
                        else                    { results.push( GameResult::new( index, GameResultType::LOSE ) ); }
                    }
                }
            },
            _ => { results.push( GameResult::new( index, GameResultType::LOSE ) ); }
        }
    }

    results
}

// This function only exists because calling borrow_mut() in main MIGHT cause problems
pub fn shuffle_deck( deck: Rc<RefCell<Deck>> ) {
    let mut deck_mut = deck.borrow_mut();

    deck_mut.shuffle();
}
