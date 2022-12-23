pub mod components;

use components::{ CardType, CardValue };
use colour::*;

#[derive(Debug, Clone)]
pub struct Card
{
    pub card_type: CardType,
    pub value: CardValue
}

impl Card
{
    pub fn new( card_type: CardType, value: CardValue ) -> Card {
        Card { card_type, value }
    }

    pub fn print( &self ) {
        match self.card_type {
            CardType::HEARTS => {
                red!( "{}", format!( "♥ {}", CardValue::to_str( &self.value ) ) );
            },
            CardType::CLUBS => {
                grey!( "{}", format!( "♣ {}", CardValue::to_str( &self.value ) ) );
            },
            CardType::SPADES => {
                grey!( "{}", format!( "♠ {}", CardValue::to_str( &self.value ) ) );
            },
            CardType::DIAMONDS => {
                red!( "{}", format!( "♦ {}", CardValue::to_str( &self.value ) ) );
            }
        }
    }
}

