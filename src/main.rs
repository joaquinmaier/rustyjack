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

use colour::*;
use std::io::{ self, Write };
use std::rc::Rc;
use std::cell::RefCell;
use terminal_size;

pub mod card;
pub mod deck;
pub mod hand;
pub mod errors;
pub mod utils;
pub mod ui;
pub mod wallet;
pub mod notifications;
pub mod level_handler;

use crate::deck::Deck;
use crate::hand::Hand;
use crate::utils::input::*;
use crate::utils::game_logic::*;
use crate::ui::TerminalResolution;
use crate::wallet::Wallet;
use crate::notifications::{ NotificationBuffer, Notification, NotificationType };
use crate::level_handler::{ LevelHandler, LevelProgressionSystem };

const PLAYERS: u32 = 2;
const INITIAL_BET: i32 = 10;
const BET_LEVEL_MULTIPLIER: f64 = 2.0;
const UPGRADE_SECURITY_MARGIN: f64 = 3.0;

fn main() {
    // ? Step 1: Init
    let terminal_size           = TerminalResolution::new( terminal_size::terminal_size().unwrap().1.0, terminal_size::terminal_size().unwrap().0.0 );

    let deck                    = Rc::new( RefCell::new( Deck::new() ) );                   // Deck of cards
    let mut level_handler       = LevelHandler::new( INITIAL_BET, LevelProgressionSystem::Linear( BET_LEVEL_MULTIPLIER ) );
    let mut input_buffer        = String::new();                                            // For receiving input from the user (reusable)
    let mut hands: Vec<Hand>    = Vec::new();

    let mut first_hand          = true;
    let mut player_wallet       = Wallet::new( level_handler.get_bet() as i32 * 2 );

    let mut notifications       = NotificationBuffer::new();

    // ? Step 2: Gameplay loop
    // Step 2.1: Initialize hands
    while player_wallet.can_pay( level_handler.get_bet() as f64 ) {
        player_wallet.bet( level_handler.get_bet() as f64 ).unwrap();

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
        let mut can_play = true;
        if hands[0].should_present_insurance() {
            can_play = insurance_round( &mut hands, &mut player_wallet, &mut notifications );
        }

        // Ranges don't update with a push() to the Vec, so we have to use while
        let mut i = 1;
        while i < hands.len() && can_play {
            let mut done = false;

            while !done {
                clr!();

                for j in 0..hands.len() {
                    hands[ j ].print();
                }

                player_wallet.print_info();

                notifications.print_all();

                if first_hand {
                    dark_grey_ln!( "\nHINT: Don't know how to play? Hit H + Intro to boot up the help menu" );
                }

                print!("\nHand {}: ", i);
                // Make sure the characters are displayed
                io::stdout().flush().unwrap();

                input_buffer.clear();

                io::stdin().read_line( &mut input_buffer ).unwrap();

                match handle_input( &input_buffer ) {
                    1   => { std::process::exit( 0 ); },
                    2   => {
                        // Stand (lock the hand) and move on to the next person.
                        hands[i].stand();
                        done = true;
                    },
                    3   => {
                        // Hit the hand, and if it's gone overboard, move on to the next person
                        hands[i].hit( Rc::clone( &deck ) );

                        if !hands[i].is_valid().unwrap()    { done = true; }

                    },
                    4   => {
                        // Split the hand
                        if hands[i].can_split() && player_wallet.bet( level_handler.get_bet() as f64 ).is_ok() {
                            match hands[i].split( Rc::downgrade( &deck ) ) {
                                Ok( new_hand )  => {
                                    hands.push( new_hand );

                                },
                                Err(_)          => ()           // Errors cannot happen since we checked beforehand if the player could
                            }

                        } else {
                            match hands[i].split( Rc::downgrade( &deck ) ) {
                                Err( e )        => {
                                    notifications.add( Notification::new( NotificationType::ERROR, String::from( e.reason.unwrap_or( "[Unexplained error]" ) ) ) );
                                },
                                Ok( _ )         => { panic!( "hand.split() returned Ok() when Err() was expected" ); }
                            }
                        }
                    },
                    5   => {
                        // Double the hand
                        if hands[i].cards.len() > 2 {
                            notifications.add( Notification::new( NotificationType::INFO, String::from( "You cannot double after hitting" ) ) );
                            continue;
                        }

                        let wallet_result = player_wallet.double_bet();

                        if wallet_result.is_err() {
                            if let Some(_) = wallet_result.unwrap_err().downcast_ref::<errors::NotEnoughMoneyError>() {
                                notifications.add( Notification::new( NotificationType::INFO, String::from( "You do not have enough money to double" ) ) );

                            }
                            else {
                                // Don't have to check, is the only other error that can appear
                                panic!( "Attempted to double a non-existent bet" );

                            }
                        }
                        else {
                            hands[i].double( Rc::downgrade( &deck ) ).unwrap();
                        }

                        if !hands[i].is_valid().unwrap()    { done = true; }

                    },
                    6   => {
                        // Present help message
                        ui::help_message( &terminal_size );
                        first_hand = false;
                    },
                    _   => ()
                }
            }

            first_hand = false;
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

        green_ln!( "\n$$ Total Money: {} $$", player_wallet.money );
        println!("\nPress ENTER to continue...");
        io::stdout().flush().unwrap();
        io::stdin().read_line( &mut input_buffer ).unwrap();

        notifications.add( Notification::new( NotificationType::INFO, format!( "Next bets: {}, Must have: {}", level_handler.peek_next_level_bet(), UPGRADE_SECURITY_MARGIN * level_handler.peek_next_level_bet() ) ) );

        // Step 4: Check if the player should pass to the next level and present a message if so.
        let mut upgraded = false;
        while player_wallet.can_pay( UPGRADE_SECURITY_MARGIN * level_handler.peek_next_level_bet() ) {
            println!( "Checking upgrades" );
            if !upgraded {
                upgraded = true;
            }

            level_handler.increase_level();
        }

        if upgraded {
            ui::upgrade_message( &terminal_size, level_handler.level, level_handler.get_bet() );
        }
    }

    red_ln!( "\nGAME OVER\n" );
}

