use crate::card::{ Card, components };
use crate::deck::Deck;
use colour::*;
use std::error::Error;
use std::rc::{ Rc, Weak };
use std::cell::RefCell;
use crate::errors::NotComputedError;

pub struct Hand
{
    pub cards: Vec<Card>,
    pub sum_value: Option<components::SumType>,
    pub locked: bool,
        auto_lock: bool,
    pub hidden: bool
}

impl Hand
{
    pub fn new( deck: Rc<RefCell<Deck>>, croupier: bool ) -> Hand {
        let mut deck_mut = deck.borrow_mut();

        let cards = [ deck_mut.take_card(), deck_mut.take_card() ];

        Hand { cards: Vec::from( cards ), sum_value: None, locked: false, auto_lock: false, hidden: croupier }
    }

    pub fn print( &self ) {
        if !self.hidden {
            for i in 0..self.cards.len() {
                self.cards[i].print();
                print!("\t");
            }

            // ! Im not convinced by the colors
            match self.sum_value.clone().unwrap() {
                components::SumType::SingleValue( n ) => {
                    if n <= 21 {
                        yellow!( "\t({})", n );
                    } else {
                        dark_red!( "\t({})", n );
                    }
                },
                components::SumType::MultipleValue( n1, n2 ) => {
                    yellow!( "\t({}/{})", n1, n2 );
                },
            }

        } else {
            self.cards[0].print();
            print!("\t");
            dark_grey!( "??" );

        }

        print!("\n\n");
    }

    pub fn reveal( &mut self, deck: Weak<RefCell<Deck>> ) {
        if self.hidden {
            self.hidden = false;

            while !self.auto_lock {
                self.hit_weak( Weak::clone( &deck ) );
            }

            /*
            match self.sum_value.clone().unwrap() {
                components::SumType::SingleValue( n ) => {
                    while n < 17 {
                        self.hit_weak( Weak::clone( &deck ) );
                    }
                },
                components::SumType::MultipleValue( n1, n2 ) => {
                    while n2 < 17 {
                        self.hit_weak( Weak::clone( &deck ) );

                    }

                    if n2 > 21 {
                        while n1 < 17 {
                            self.hit_weak( Weak::clone( &deck ) );

                        }
                    }
                }
            }
            */
        }
    }

    pub fn calc_sum( &mut self ) {
        let mut total_count: [i32; 2] = [ 0, -1 ];

        for card in self.cards.iter() {
            if card.value == components::CardValue::ACE {
                // We may have multiple values
                if total_count[0] + 11 <= 21 {
                    // Index 1 will hold the value with +11, while index 0 will hold the value with +1
                    total_count[1] = total_count[0] + 11;
                    total_count[0] += components::CardValue::to_int( &card.value );
                    continue;

                }
            }

            // If we are keeping a second count
            if total_count[1] != -1 {
                total_count[1] += components::CardValue::to_int( &card.value );

                // Has this count gone overboard? Delete it then
                if total_count[1] > 21 {
                    total_count[1] = -1;
                }
            }
            total_count[0] += components::CardValue::to_int( &card.value );
        }

        if total_count[1] != -1 {
            self.sum_value = Some( components::SumType::MultipleValue( total_count[0], total_count[1] ) );

        } else {
            self.sum_value = Some( components::SumType::SingleValue( total_count[0] ) );

        }

        if total_count[0] >= 17 || total_count[1] >= 17 {
            self.auto_lock = true;
        }

        // If the player has reached or surpassed 21, lock it
        if total_count[0] >= 21 || total_count[1] >= 21 {
            self.locked = true;
        }
    }

    pub fn is_valid( &self ) -> Result<bool, Box<dyn Error>> {
        if let components::SumType::SingleValue( sum ) = self.sum_value.clone().unwrap() {
            return Ok( sum <= 21 );

        } else if let components::SumType::MultipleValue( var1, var2 ) = self.sum_value.clone().unwrap() {
            return Ok( var1 <= 21 || var2 <= 21 );

        }

        Err( Box::new( NotComputedError ) )
    }

    fn hit_weak( &mut self, deck: Weak<RefCell<Deck>> ) {
        if self.is_valid().unwrap() && !self.locked {
            println!("hit_weak() started");
            match deck.upgrade() {
                Some( deck ) => {
                    println!("std::rc::Weak upgraded");

                    let mut deck_mut = deck.borrow_mut();

                    println!("hit_weak() got mutable access to Rc");

                    self.cards.push( deck_mut.take_card() );

                    println!("card pushed");

                    self.calc_sum();

                    println!("sum calculated");
                },
                None => { panic!( "Deck has been dropped while hitting the hand" ); }
            }
        }
    }

    pub fn hit( &mut self, deck: Rc<RefCell<Deck>> ) {
        if self.is_valid().unwrap() && !self.locked {
            let mut deck_mut = deck.borrow_mut();

            println!("hit() got mutable access to Rc");

            self.cards.push( deck_mut.take_card() );

            println!("card pushed");

            self.calc_sum();

            println!("sum calculated");
        }
    }

    pub fn stand( &mut self ) {
        self.locked = true;
    }
}

impl std::fmt::Debug for Hand
{
    fn fmt( &self, f: &mut std::fmt::Formatter ) -> std::fmt::Result {
        f.debug_struct("Hand")
            .field( "cards", &self.cards )
            .field( "sum_value", &self.sum_value )
            .field( "locked", &self.locked )
            .field( "hidden", &self.hidden )
            .finish()
    }
}

