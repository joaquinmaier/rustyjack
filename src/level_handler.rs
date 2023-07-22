use colour::*;

pub struct LevelHandler {
    pub level: i32,
    previous_bet: Option<f64>,
    starting_bet: i32,
    progression: LevelProgressionSystem
}

impl LevelHandler
{
    pub fn new( starting_bet: i32, progression: LevelProgressionSystem ) -> LevelHandler {
        LevelHandler { level: 1, previous_bet: None, starting_bet, progression }
    }

    pub fn get_bet( &self ) -> f64 {
        return match self.progression {
            LevelProgressionSystem::Linear( multiplier )    => {
                if let None = self.previous_bet {
                    return self.starting_bet as f64;
                }

                self.previous_bet.unwrap() * multiplier
            },
            LevelProgressionSystem::Exponential( base, start )     => {
                if let None = start {
                    return base.pow( self.level as u32 ) as f64;
                }

                base.pow( ( start.unwrap() + ( self.level - 1 ) ) as u32 ) as f64
            }
        }
    }

    pub fn increase_level( &mut self ) {
        self.previous_bet = Some( self.get_bet() );
        self.level += 1;
    }

    pub fn peek_next_level_bet( &self ) -> f64 {
        let level = self.level + 1;

        return match self.progression {
            LevelProgressionSystem::Linear( multiplier )    => {
                self.get_bet() * multiplier
            },
            LevelProgressionSystem::Exponential( base, start )     => {
                if let None = start {
                    return base.pow( level as u32 ) as f64;
                }

                base.pow( ( start.unwrap() + ( level - 1 ) ) as u32 ) as f64
            }
        }
    }

    pub fn print_info( &self ) {
        let level = self.level - 1;

        if level == 0 {
            return;
        }

        let mut stars = String::from( "[" );
        for _ in 0..level {
            stars.push_str( "*" );
        }
        stars.push_str( "]" );

        magenta_ln!( "{}", stars );
    }
}

pub enum LevelProgressionSystem {
    Linear( f64 ),
    Exponential( i32, Option<i32> )
}
