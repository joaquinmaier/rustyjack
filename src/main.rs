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
    // Would love a for-each but it conflicts with the printing of the hands
    for i in 1..hands.len() {
        let mut done = false;

        while !done {
            clr!();

            for j in 0..hands.len() {
                hands[ j as usize ].print();
            }

            print!("\n: ");
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
                _   => ()
            }
        }
    }

    hands[0].reveal( Rc::downgrade( &deck ) );

    // Step 3: Present croupier hand and show win/lose message
    clr!();

    for j in 0..hands.len() {
        hands[ j as usize ].print();
    }

    let winner = determine_winner( &hands );
    if winner == 1 {
        green_ln!( "\nYOU WIN!" );

    } else if winner == 0 {
        red_ln!( "\nYOU LOSE" );

    } else {
        // * THIS IS A BAND-AID. BLAH-BLAH-BLAH.
        cyan_ln!( "\nPUSH ;)" );

    }
}
