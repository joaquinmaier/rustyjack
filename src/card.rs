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

