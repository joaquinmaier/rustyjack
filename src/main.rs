use colour::*;
use std::io::{ self, Write };
use std::rc::Rc;
use std::cell::RefCell;

pub mod card;
pub mod deck;
pub mod hand;
pub mod errors;
pub mod utils;
use crate::deck::Deck;
use crate::hand::Hand;
use crate::utils::input::*;
use crate::utils::game_logic::*;

const PLAYERS: u32 = 2;

// TODO: Fix the determine_winner() function so it isn't a house of cards.

macro_rules! clr {
    () => {
        print!("\x1B[2J\x1B[1;1H");
    }
}

fn main() {
    // ? Step 1: Init
    let deck                    = Rc::new( RefCell::new( Deck::new() ) );   // Deck of cards
    let mut input_buffer        = String::new();                            // For receiving input from the user (reusable)
    let mut hands: Vec<Hand>    = Vec::new();

    // ? Step 2: Gameplay loop
    // Step 2.1: Initialize hands
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
            // Make sure the character is displayed
            io::stdout().flush().unwrap();

            input_buffer.clear();

            // TODO: This error doesn't have to be fatal? Look into that.
            io::stdin().read_line( &mut input_buffer ).unwrap();

            match handle_input( &input_buffer ) {
                1   => { done = true; },        // TODO: This will be a bug if multiple players are involved
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
                green_ln!( "Hand {} WINS", winner.player_id );
            },
            GameResultType::PUSH => {
                cyan_ln!( "Hand {} PUSH", winner.player_id );
            },
            GameResultType::LOSE => {
                red_ln!( "Hand {} LOSES", winner.player_id );
            }
        }
    }
}
