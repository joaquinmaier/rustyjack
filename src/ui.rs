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
use std::thread::sleep;
use std::time::Duration;

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

    dark_red!( "\nThe only way to go is UP." );

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

    animated_print( String::from("\t\tTABLE UPGRADED\n\n"), 25, PrintColorConfig::GREEN );

    animated_print( String::from("You have been upgraded to a better table!\n\n"), 12, PrintColorConfig::DEFAULT );

    animated_print( String::from("You are now on "), 12, PrintColorConfig::DEFAULT );
    animated_print( format!( "table {}\n", new_level ), 12, PrintColorConfig::MAGENTA );

    animated_print( String::from("The bets will now start at "), 12, PrintColorConfig::DEFAULT );
    animated_print( format!( "${}\n\n", new_bets ), 12, PrintColorConfig::BLUE );

    print_separator_bw( &resolution );

    println!( "Press INTRO to play..." );

    io::stdout().flush().unwrap();
    io::stdin().read_line( &mut _input ).unwrap();
}

pub enum PrintColorConfig {
    DEFAULT,
    GREEN,
    MAGENTA,
    BLUE,
    RED
}

pub fn animated_print( message: String, rate_ms: u64, color: PrintColorConfig ) {
    for character in message.chars() {
        match &color {
            PrintColorConfig::DEFAULT => { print!( "{}", character ); },
            PrintColorConfig::GREEN => { green!( "{}", character ); },
            PrintColorConfig::MAGENTA => { magenta!( "{}", character ); },
            PrintColorConfig::BLUE => { blue!( "{}", character ); },
            PrintColorConfig::RED => { red!( "{}", character ); }
        }

        io::stdout().flush().unwrap();

        sleep( Duration::from_millis( rate_ms ) );
    }
}

pub fn game_winner_message( resolution: &TerminalResolution ) {
    let mut _input = String::new();
    crate::clr!();

    print_separator_bw( &resolution );

    animated_print( String::from( "\t\tYOUR ACCOUNT HAS BEEN SUSPENDED\n\n" ), 25, PrintColorConfig::RED );

    animated_print( String::from( "Due to allegations related to your account breaking our Terms Of Service\nwe have decided to suspend your account for further review.\n\n" ), 12, PrintColorConfig::DEFAULT );
    animated_print( String::from( "You will be unable to access your account, participate in games or access/retrieve your funds\nfor as long as your account remains in this state.\n\n" ), 12, PrintColorConfig::DEFAULT );

    animated_print( String::from( "Please be patient while our team works on this issue.\n\n" ), 12, PrintColorConfig::DEFAULT );

    animated_print( String::from( "- Rustyjack Team\n\n" ), 12, PrintColorConfig::DEFAULT );

    print_separator_bw( &resolution );

    println!( "Press INTRO to continue..." );

    io::stdin().read_line( &mut _input ).unwrap();
    io::stdout().flush().unwrap();

    green_ln!( "YOU WIN!\n" );
}

pub fn startup_message() {
    println!( "Connecting to the rustyjack servers..." );

    sleep( Duration::from_secs( 1 ) );
}
