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

#[macro_export]
macro_rules! clr {
    () => {
        print!("\x1B[2J\x1B[1;1H");
    }
}

pub struct TerminalResolution {
    pub width: u16,
    pub height: u16
}

impl TerminalResolution
{
    pub fn new( width: u16, height: u16 ) -> TerminalResolution {
        TerminalResolution { width, height }
    }
}

pub fn print_separator_bw( dimensions: &TerminalResolution ) {
    let mut counter: u8 = 0;

    for _ in 0..dimensions.width {
        match counter {
            0 => { dark_grey!( "♥ " ); },
            1 => { dark_grey!( "♣ " ); },
            2 => { dark_grey!( "♦ " ); },
            3 => { dark_grey!( "♠ " ); },
            _ => { panic!( "Counter hit unexpected value" ); }
        }

        if counter < 3 {
            counter += 1;

        } else {
             counter = 0;

        }
    }

    print!( "\n\n" );
}

pub fn help_message( resolution: &TerminalResolution ) {
    let mut input = String::new();

    crate::clr!();

    print_separator_bw( &resolution );

    println!( "HOW TO PLAY" );
    red_ln!( "\n· h\t=\tHIT" );
    yellow_ln!( "· s\t=\tSTAND" );
    green_ln!( "· d\t=\tDOUBLE DOWN" );
    cyan_ln!( "· S\t=\tSPLIT" );
    blue_ln!( "· e\t=\tEXIT" );
    magenta_ln!( "· H\t=\tHELP" );

    println!( "\nTABLE RULES" );
    println!( "\n1. The game is played with 1 (one) single deck." );
    println!( "2. The deck is shuffled for every round." );
    println!( "3. Dealer must draw to 16, and stand on all 17's." );
    println!( "4. Blackjack pays 3 to 2." );
    println!( "5. All players must bet $10 (ten) for every round." );
    println!( "6. Players who cannot pay the above price will lose." );

    dark_grey!( "\n\nFor an in-depth blackjack HOW-TO, you can visit:" );
    dark_blue!( " https://www.blackjackapprenticeship.com/how-to-play-blackjack/" );
    dark_grey_ln!( " (not sponsored)\n" );

    print_separator_bw( &resolution );

    println!( "Press INTRO to go back..." );

    io::stdout().flush().unwrap();
    io::stdin().read_line( &mut input ).unwrap();
}

pub fn upgrade_message( resolution: &TerminalResolution, new_level: i32, new_bets: f64 ) {
    let mut _input = String::new();

    crate::clr!();

    print_separator_bw( &resolution );

    green_ln!( "\t\tTABLE UPGRADED\n" );

    println!( "You have been upgraded to a better table!\n" );

    print!( "You are now on " );
    magenta_ln!( "table {}", new_level );

    print!( "The bets will now start at " );
    blue_ln!( "${}\n", new_bets );

    print_separator_bw( &resolution );

    println!( "Press INTRO to play..." );

    io::stdout().flush().unwrap();
    io::stdin().read_line( &mut _input ).unwrap();
}

