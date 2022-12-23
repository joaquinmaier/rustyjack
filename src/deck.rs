use rand::prelude::*;
use crate::card::{ Card, components };

pub struct Deck
{
    hearts: [bool; 13],
    clubs: [bool; 13],
    spades: [bool; 13],
    diamonds: [bool; 13]
}

impl Deck
{
    pub fn new() -> Deck {
        let hearts = [false; 13];
        let clubs = [false; 13];
        let spades = [false; 13];
        let diamonds = [false; 13];

        Deck { hearts, clubs, spades, diamonds }

    }

    pub fn take_card( &mut self ) -> Card {
        let mut rng = rand::thread_rng();

        loop {
            let card_type = rng.gen_range( 0..4 );
            let card_value = rng.gen_range( 0..13 );

            match card_type {
                0 => {
                    if self.hearts[ card_value ] == false {
                        self.hearts[ card_value ] = true;
                        return Card::new( components::CardType::HEARTS, components::CardValue::from( card_value ) );
                    }
                },
                1 => {
                    if self.clubs[ card_value ] == false {
                        self.clubs[ card_value ] = true;
                        return Card::new( components::CardType::CLUBS, components::CardValue::from( card_value ) );

                    }
                },
                2 => {
                    if self.spades[ card_value ] == false {
                        self.spades[ card_value ] = true;
                        return Card::new( components::CardType::SPADES, components::CardValue::from( card_value ) );

                    }
                },
                3 => {
                    if self.diamonds[ card_value ] == false {
                        self.diamonds[ card_value ] = true;
                        return Card::new( components::CardType::DIAMONDS, components::CardValue::from( card_value ) );

                    }
                },
                _ => { panic!( "Forbidden card type received" ); }
            }
        }
    }

    pub fn shuffle( &mut self ) {
        self.hearts = [false; 13];
        self.clubs = [false; 13];
        self.spades = [false; 13];
        self.diamonds = [false; 13];

        // Any additional stuff
    }
}


