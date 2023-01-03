use colour::*;

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

pub fn print_separator( dimensions: &TerminalResolution ) {
    let mut counter: u8 = 0;

    for _ in 0..dimensions.width {
        match counter {
            0 => { red!( "♥ " ); },
            1 => { grey!( "♣ " ); },
            2 => { red!( "♦ " ); },
            3 => { grey!( "♠ " ); },
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
