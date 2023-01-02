use crate::card::{ Card, components };
use crate::deck::Deck;
use colour::*;
use std::rc::{ Rc, Weak };
use std::cell::RefCell;
use crate::errors::{ NotComputedError, InvalidOperationError };
use crate::card::components::{ CardValue, SumType };

#[derive(Clone)]
pub struct Hand
{
    pub cards: Vec<Card>,
    pub sum_value: Option<components::SumType>,
    pub locked: bool,
        auto_lock: bool,
        croupier: bool,
    pub hidden: bool
}

impl Hand
{
    pub fn new( deck: Rc<RefCell<Deck>>, croupier: bool ) -> Hand {
        let mut deck_mut = deck.borrow_mut();

        let cards = [ deck_mut.take_card(), deck_mut.take_card() ];

        Hand { cards: Vec::from( cards ), sum_value: None, locked: false, auto_lock: false, croupier, hidden: croupier }
    }

    pub fn new_using( existing_card: Card, deck: Weak<RefCell<Deck>> ) -> Hand {
        match deck.upgrade() {
            Some( deck ) => {
                let mut deck_mut = deck.borrow_mut();

                let cards = [ existing_card, deck_mut.take_card() ];

                Hand { cards: Vec::from( cards ), sum_value: None, locked: false, auto_lock: false, croupier: false, hidden: false }

            },
            None => { panic!( "Required deck has been dropped unexpectedly" ); }
        }
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

            if self.is_splittable() && !self.croupier {
                yellow!( " [S]" );
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

    pub fn is_valid( &self ) -> Result<bool, Box<NotComputedError>> {
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

    pub fn split( &mut self, deck: Weak<RefCell<Deck>> ) -> Result<Hand, Box<InvalidOperationError>> {
        // Cannot split with more than 2 cards
        if self.cards.len() > 2     { return Err( Box::new( InvalidOperationError::new( Some( "Hand has more than 3 cards" ) ) ) ); }

        // Cannot split if both cards are not of the same value
        if CardValue::to_int( &self.cards[0].value ) != CardValue::to_int( &self.cards[1].value )     { return Err( Box::new( InvalidOperationError::new( Some( "Hand's cards are not of equal value" ) ) ) ); }

        let splitting_card = self.cards.pop().unwrap();

        let mut new_hand = Hand::new_using( splitting_card, Weak::clone( &deck ) );

        match deck.upgrade() {
            Some( deck_upg ) => {
                let mut deck_mut = deck_upg.borrow_mut();

                self.cards.push( deck_mut.take_card() );


            },
            None => { panic!( "Required Deck has been dropped unexpectedly" ); }
        }

        self.calc_sum();
        new_hand.calc_sum();

        Ok( new_hand )
    }

    pub fn double( &mut self, deck: Weak<RefCell<Deck>> ) -> Result<(), Box<InvalidOperationError>> {
        if self.locked          { return Err( Box::new( InvalidOperationError::new( Some( "Cannot hit hand because it is locked" ) ) ) ); }

        self.hit_weak( Weak::clone( &deck ) );

        self.locked = true;

        Ok(())
    }

    pub fn is_splittable( &self ) -> bool {
        if self.cards.len() > 2                                                                     { return false; }

        if CardValue::to_int( &self.cards[0].value ) != CardValue::to_int( &self.cards[1].value )   { return false; }

        return true;
    }

    pub fn is_blackjack( &self ) -> bool {
        if self.cards.len() == 2 {
            return match self.sum_value.clone().unwrap() {
                SumType::SingleValue( n ) if n == 21                        => true,
                SumType::MultipleValue( n1, n2 ) if n2 == 21 || n1 == 21    => true,
                _                                                           => false
            }
        }

        return false;
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

