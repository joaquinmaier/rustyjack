/*
Copyright (c) 2023 Joaquin Maier

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all
copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
SOFTWARE.
*/

use crate::hand::Hand;
use crate::card::components::SumType;
use crate::Deck;
use crate::wallet::Wallet;
use crate::utils::input::wants_to_insure;
use crate::notifications::{ Notification, NotificationType, NotificationBuffer };
use std::rc::Rc;
use std::cell::RefCell;
use std::io::{ self, Write };

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

pub fn insurance_round( hands: &mut Vec<Hand>, wallet: &mut Wallet, notifications: &mut NotificationBuffer ) -> bool {
    // Step 1: Present option and collect insurances
    let mut i = 1;
    let mut input = String::new();

    while i < hands.len()
    {
        let mut done = false;

        while !done
        {
            crate::clr!();

            for j in 0..hands.len() {
                hands[ j ].print();
            }

            wallet.print_info();

            notifications.print_all();

            colour::green_ln!( "\nINSURE BET? (y/n)" );

            print!( "\nHand {}: ", i );

            io::stdout().flush().unwrap();
            input.clear();
            io::stdin().read_line( &mut input ).unwrap();

            let result = wants_to_insure( &input );

            if result.is_none()     { continue; }

            if result.unwrap() {
                match wallet.pay_insurance() {
                    Ok( () )    => {
                        hands[ i ].insured = true;
                        done = true;

                    },
                    Err( e )    => {
                        if let Some(_) = e.as_ref().downcast_ref::<crate::errors::NonExistentBetError>() {
                            panic!( "Bet does not exist" );

                        } else if let Some(_) = e.as_ref().downcast_ref::<crate::errors::InvalidStateError>() {
                            panic!( "Hand is illegal state" );

                        } else if let Some(_) = e.as_ref().downcast_ref::<crate::errors::NotEnoughMoneyError>() {
                            notifications.add( Notification::new( NotificationType::INFO, String::from( "Not enough money to insure" ) ) );

                        }
                    }
                }

            } else {
                done = true;
            }
        }
        i += 1;
    }

    // Step 2: Check if it is blackjack, give back the bets of those who insured, and take the ones
    // of those who didn't.
    if hands[0].is_blackjack() {
        for j in 1..hands.len() {
            if hands[j].insured {
                wallet.return_insurance().unwrap();

            } else {
                wallet.take_bet().unwrap();

            }
        }

        return false;
    }

    // Step 3: If it isn't blackjack, take the insurances (which because of us discounting the
    // money immediately is done already) and keep going.
    true
}

