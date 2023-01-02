use colour::*;
use std::io::{ self, Write };
use std::rc::Rc;
use std::cell::RefCell;

extern crate termsize;

pub mod card;
pub mod deck;
pub mod hand;
pub mod errors;
pub mod utils;
pub mod ui;
pub mod wallet;
use crate::deck::Deck;
use crate::hand::Hand;
use crate::utils::input::*;
use crate::utils::game_logic::*;
use crate::ui::*;
use crate::wallet::Wallet;

const PLAYERS: u32 = 2;

// TODO: Splits create problems because it is trying to give rewards multiple times, because it doesn't put more money in.
// TODO: Doubling and pushing may create money out of thin air?

fn main() {
    // ? Step 1: Init
    let terminal_size           = termsize::get().map( |size| { TerminalResolution::new( size.rows, size.cols ); } ).unwrap();

    let deck                    = Rc::new( RefCell::new( Deck::new() ) );   // Deck of cards
    let mut input_buffer        = String::new();                            // For receiving input from the user (reusable)
    let mut hands: Vec<Hand>    = Vec::new();

    let mut playing             = true;
    let mut player_wallet       = Wallet::new( 10 );

    // ? Step 2: Gameplay loop
    // Step 2.1: Initialize hands
    while playing && player_wallet.money > 0. {
        player_wallet.bet( 10 as f64 ).unwrap();

        shuffle_deck( Rc::clone( &deck ) );
        hands.clear();

        for i in 0..PLAYERS {
            if i == 0 {
                hands.push( Hand::new( Rc::clone( &deck ), true ) );

            } else {
                hands.push( Hand::new( Rc::clone( &deck ), false ) );

            }

            hands[i as usize].calc_sum();
        }

        // Step 2.2: Receive player input and act accordingly
        // Ranges don't update with a push() to the Vec, so we have to use while
        let mut i = 1;
        while i < hands.len() {
            let mut done = false;

            while !done {
                clr!();

                for j in 0..hands.len() {
                    hands[ j as usize ].print();
                }

                print!("\nHand {}: ", i);
                // Make sure the characters are displayed
                io::stdout().flush().unwrap();

                input_buffer.clear();

                // TODO: This error doesn't have to be fatal? Look into that.
                io::stdin().read_line( &mut input_buffer ).unwrap();

                match handle_input( &input_buffer ) {
                    1   => { done = true; playing = false; },
                    2   => {
                        // Stand (lock the hand) and move on to the next person.
                        hands[i].stand();
                        done = true;
                    },
                    3   => {
                        // Hit the hand, and if it's gone overboard, move on to the next person
                        hands[i].hit( Rc::clone( &deck ) );

                        if !hands[i].is_valid().unwrap() { done = true; }

                    },
                    4   => {
                        // Split the hand
                        match hands[i].split( Rc::downgrade( &deck ) ) {
                            Ok( new_hand ) => {
                                hands.push( new_hand );

                            },
                            Err(_) => ()            // We already know what the error is, and it is non-fatal, so ignore it
                        }
                    },
                    5   => {
                        // Double the hand
                        let wallet_result = player_wallet.double_bet();

                        if wallet_result.is_err() {
                            if let Some(_) = wallet_result.unwrap_err().downcast_ref::<errors::NotEnoughMoneyError>() {
                                dark_red_ln!( "You do not have enough money to double" );

                            }
                            else {
                                // Don't have to check, is the only other error that can appear
                                panic!( "Attempted to double a non-existent bet" );

                            }
                        }
                        else {
                            hands[i].double( Rc::downgrade( &deck ) ).unwrap();
                        }

                    },
                    _   => ()
                }
            }

            i += 1;
        }

        hands[0].reveal( Rc::downgrade( &deck ) );

        // Step 3: Present croupier hand and show win/lose message
        clr!();

        for j in 0..hands.len() {
            hands[ j as usize ].print();
        }

        let winners = determine_winners( &hands );

        if winners.len() == 0 {
            panic!( "No winners/losers were determined!" );
        }

        for winner in winners.iter() {
            match winner.result {
                GameResultType::WIN => {
                    green_ln!( "\nHand {} WINS", winner.player_id );
                    player_wallet.give_win_reward().unwrap();
                },
                GameResultType::WINBJ => {
                    yellow_ln!( "\nHand {} WINS WITH BLACKJACK", winner.player_id );
                    player_wallet.give_win_reward_bj().unwrap();
                },
                GameResultType::PUSH => {
                    cyan_ln!( "\nHand {} PUSH", winner.player_id );
                    player_wallet.push_bet().unwrap();
                },
                GameResultType::LOSE => {
                    red_ln!( "\nHand {} LOSES", winner.player_id );
                    player_wallet.take_bet().unwrap();
                }
            }
        }

        green_ln!( "$$ Total Money: {} $$", player_wallet.money );
        println!("\nPress any key to continue...");
        io::stdout().flush().unwrap();
        io::stdin().read_line( &mut input_buffer ).unwrap();

    }

    if playing {
        red_ln!( "\nGAME OVER" );
    }

}