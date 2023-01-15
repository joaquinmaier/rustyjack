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

