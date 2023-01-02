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
